use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use cleanbro_core::engine::discover::{self, Candidate};
use cleanbro_core::models::{CategoryDone, Finding, ScanFinished, ScanId, ScanMode, ScanProgress};
use cleanbro_core::rules::{catalog, Category};
use cleanbro_core::store::db;
use cleanbro_core::store::scan_repo::{save_scan, ScanRow};
use cleanbro_core::{
    walk, AppError, AppResult, CancellationToken, ErrorCode, Guard, Matcher, Rule, WalkOptions,
    WalkResult,
};
use rayon::prelude::*;
use serde_json::json;

use crate::services::event::EventEmitter;

const PROGRESS_INTERVAL: Duration = Duration::from_millis(250);

#[derive(Debug, Clone)]
pub struct ScanRequest {
    pub mode: ScanMode,
    pub categories: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ScanContext {
    pub home: PathBuf,
    pub db_path: PathBuf,
}

#[derive(Debug)]
pub struct ScanOutcome {
    pub scan_id: ScanId,
    pub partial: bool,
    pub duration_ms: u64,
    pub findings: Vec<Finding>,
}

/// Orkestrasi scan tanpa tipe Tauri; emitter dan token disuntikkan.
pub fn execute(
    context: &ScanContext,
    request: ScanRequest,
    emitter: Arc<dyn EventEmitter>,
    cancel: &CancellationToken,
) -> AppResult<ScanOutcome> {
    let started_at = now_millis();
    let clock = Instant::now();
    let scan_id = ScanId::new();
    let rules = select_rules(&request)?;
    let matcher = Matcher::new(&context.home);
    let guard = Guard::new(&context.home);

    let visited = Arc::new(AtomicU64::new(0));
    let categories_done = Arc::new(AtomicU64::new(0));
    let throttle = Arc::new(Throttle::new());
    let on_visit = visit_reporter(
        Arc::clone(&emitter),
        scan_id.clone(),
        Arc::clone(&visited),
        Arc::clone(&categories_done),
        Arc::clone(&throttle),
    );

    let discovery = discover::discover(
        &context.home,
        &matcher,
        &rules,
        cancel,
        cleanbro_core::engine::walker::DEADLINE,
        on_visit,
    );
    let measured: Vec<(Candidate, Option<WalkResult>)> = discovery
        .candidates
        .par_iter()
        .map(|candidate| {
            if cancel.is_cancelled() {
                return (candidate.clone(), None);
            }
            match guard.check(&candidate.path, candidate.rule) {
                Ok(()) => (
                    candidate.clone(),
                    Some(walk(&candidate.path, &WalkOptions::default(), cancel)),
                ),
                // Guard menolak: item dilewati, audit penuh menyusul di M3.
                Err(_) => (candidate.clone(), None),
            }
        })
        .collect();

    let mut findings = Vec::new();
    let mut partial = discovery.partial;
    let mut per_rule: BTreeMap<&'static str, (u64, u64)> = BTreeMap::new();
    for (candidate, result) in measured {
        let Some(result) = result else { continue };
        partial |= result.partial || result.cancelled;
        let entry = per_rule.entry(candidate.rule.id).or_insert((0, 0));
        entry.0 += result.bytes;
        entry.1 += result.files;
        findings.push(Finding::new(
            scan_id.clone(),
            candidate.rule.id,
            candidate.path.to_string_lossy(),
            result.bytes,
            result.files,
            candidate.rule.risk,
            project_label(&candidate, &context.home),
        ));
    }

    let total_files: u64 = per_rule.values().map(|(_, files)| files).sum();
    for (rule_id, (bytes, files)) in &per_rule {
        let done = categories_done.fetch_add(1, Ordering::Relaxed) + 1;
        emitter.emit(
            "scan://category",
            json!(CategoryDone {
                scan_id: scan_id.clone(),
                rule_id: (*rule_id).to_owned(),
                bytes: *bytes,
                item_count: *files,
            }),
        );
        emitter.emit(
            "scan://progress",
            json!(ScanProgress {
                scan_id: scan_id.clone(),
                scanned_files: total_files,
                current_path: String::new(),
                categories_done: done,
            }),
        );
    }

    let duration_ms = clock.elapsed().as_millis() as u64;
    persist(context, &scan_id, &request, started_at, partial, &findings)?;
    emitter.emit(
        "scan://finished",
        json!(ScanFinished {
            scan_id: scan_id.clone(),
            partial,
            duration_ms,
        }),
    );
    Ok(ScanOutcome {
        scan_id,
        partial,
        duration_ms,
        findings,
    })
}

/// Validasi filter kategori sebelum scan dijadwalkan; error kembali ke UI.
pub fn validate_request(request: &ScanRequest) -> AppResult<()> {
    select_rules(request).map(|_| ())
}

fn select_rules(request: &ScanRequest) -> AppResult<Vec<&'static Rule>> {
    let mut rules = catalog::for_mode(request.mode);
    if let Some(categories) = &request.categories {
        for id in categories {
            if catalog::by_id(id).is_none() {
                return Err(AppError::new(
                    ErrorCode::NotFound,
                    format!("rule tidak dikenal: {id}"),
                ));
            }
        }
        rules.retain(|rule| categories.iter().any(|id| id == rule.id));
        if rules.is_empty() {
            return Err(AppError::new(
                ErrorCode::InvalidPath,
                "tidak ada rule yang cocok dengan kategori",
            ));
        }
    }
    Ok(rules)
}

fn project_label(candidate: &Candidate, home: &Path) -> Option<String> {
    match candidate.rule.category {
        Category::Build | Category::Media => discover::project_name(&candidate.path, home),
        _ => None,
    }
}

fn persist(
    context: &ScanContext,
    scan_id: &ScanId,
    request: &ScanRequest,
    started_at: i64,
    partial: bool,
    findings: &[Finding],
) -> AppResult<()> {
    let mut connection = db::open(&context.db_path)?;
    let row = ScanRow {
        id: scan_id.as_str().to_owned(),
        mode: request.mode.as_str().to_owned(),
        started_at,
        finished_at: now_millis(),
        partial,
        bytes_found: findings.iter().map(|finding| finding.bytes).sum(),
    };
    save_scan(&mut connection, &row, findings)
}

fn visit_reporter(
    emitter: Arc<dyn EventEmitter>,
    scan_id: ScanId,
    visited: Arc<AtomicU64>,
    categories_done: Arc<AtomicU64>,
    throttle: Arc<Throttle>,
) -> Arc<dyn Fn(&Path) + Send + Sync> {
    Arc::new(move |path| {
        let seen = visited.fetch_add(1, Ordering::Relaxed) + 1;
        if seen.is_multiple_of(64) && throttle.ready() {
            emitter.emit(
                "scan://progress",
                json!(ScanProgress {
                    scan_id: scan_id.clone(),
                    scanned_files: seen,
                    current_path: path.to_string_lossy().into_owned(),
                    categories_done: categories_done.load(Ordering::Relaxed),
                }),
            );
        }
    })
}

fn now_millis() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as i64)
        .unwrap_or(0)
}

/// Batasi emit progress maksimal 4 kali per detik.
struct Throttle {
    last: Mutex<Instant>,
}

impl Throttle {
    fn new() -> Self {
        Self {
            last: Mutex::new(Instant::now() - PROGRESS_INTERVAL),
        }
    }

    fn ready(&self) -> bool {
        let now = Instant::now();
        let mut last = self
            .last
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        if now.duration_since(*last) >= PROGRESS_INTERVAL {
            *last = now;
            true
        } else {
            false
        }
    }
}
