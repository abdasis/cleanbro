use std::path::{Component, Path, PathBuf};

use globset::{Glob, GlobBuilder, GlobSet, GlobSetBuilder};

use crate::rules::{Rule, RULES};

/// Pencocokan path absolut ke rule katalog memakai globset.
#[derive(Clone)]
pub struct Matcher {
    home: PathBuf,
    compiled: Vec<(&'static Rule, GlobSet)>,
}

impl Matcher {
    pub fn new(home: impl Into<PathBuf>) -> Self {
        let home = normalize(&home.into());
        let compiled = RULES
            .iter()
            .map(|rule| {
                let mut builder = GlobSetBuilder::new();
                for pattern in rule.globs {
                    builder.add(build_glob(&home, pattern));
                }
                let set = builder.build().expect("glob katalog harus valid");
                (rule, set)
            })
            .collect();
        Self { home, compiled }
    }

    pub fn home(&self) -> &Path {
        &self.home
    }

    /// Rule pertama yang cocok dan tidak melewati `max_depth` rule tersebut.
    pub fn match_rule(&self, path: &Path) -> Option<&'static Rule> {
        let path = normalize(path);
        let relative = path.strip_prefix(&self.home).ok()?;
        let depth = relative.components().count();
        self.compiled
            .iter()
            .find(|(rule, set)| depth <= rule.max_depth as usize && set.is_match(&path))
            .map(|(rule, _)| *rule)
    }
}

fn build_glob(home: &Path, pattern: &str) -> Glob {
    let full = home.join(pattern);
    GlobBuilder::new(&full.to_string_lossy())
        .literal_separator(false)
        .build()
        .expect("pola glob katalog harus valid")
}

/// Kanonikalisasi bila path ada; jika tidak, normalisasi leksikal (`.`/`..`).
pub fn normalize(path: &Path) -> PathBuf {
    if let Ok(canonical) = path.canonicalize() {
        return canonical;
    }
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}
