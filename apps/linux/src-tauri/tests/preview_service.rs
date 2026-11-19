mod support;

use cleanbro_core::rules::RiskLevel;
use sapu_bersih::services::plan_service::{self, PlanContext};

#[test]
fn plan_excludes_review_risk_by_default() {
    let temp = support::TempDir::new("preview-default");
    let scan = cleanbro_core::ScanId::new();
    let safe = support::finding(
        &scan,
        "cache.gradle",
        &temp.path.join(".gradle/caches"),
        100,
        RiskLevel::Safe,
    );
    let review = support::finding(
        &scan,
        "build.dist",
        &temp.path.join("proyek/dist"),
        200,
        RiskLevel::Review,
    );
    let risky = support::finding(
        &scan,
        "build.vendor",
        &temp.path.join("proyek/vendor"),
        300,
        RiskLevel::Risky,
    );
    support::seed_scan(&temp.path, vec![safe, review, risky]);

    let context = PlanContext::new(temp.db());
    let plan = plan_service::preview_selection(&context, scan.as_str(), &[]).expect("plan");

    assert_eq!(plan.items.len(), 3);
    assert_eq!(plan.total_bytes, 100);
    assert_eq!(plan.total_items, 1);
    let review_item = plan
        .items
        .iter()
        .find(|item| item.rule_id == "build.dist")
        .expect("item review");
    assert!(review_item.excluded);
}

#[test]
fn plan_recomputes_bytes_when_item_excluded() {
    let temp = support::TempDir::new("preview-recompute");
    let scan = cleanbro_core::ScanId::new();
    let safe = support::finding(
        &scan,
        "cache.gradle",
        &temp.path.join(".gradle/caches"),
        100,
        RiskLevel::Safe,
    );
    let review = support::finding(
        &scan,
        "build.dist",
        &temp.path.join("proyek/dist"),
        250,
        RiskLevel::Review,
    );
    let review_id = review.id.clone();
    support::seed_scan(&temp.path, vec![safe.clone(), review]);
    let safe_id = safe.id.clone();

    let context = PlanContext::new(temp.db());
    let before = plan_service::preview_selection(&context, scan.as_str(), &[]).expect("plan awal");
    assert_eq!(before.total_bytes, 100);

    let after = plan_service::set_item_excluded(&context, scan.as_str(), &review_id, false)
        .expect("sertakan item review");
    assert_eq!(after.total_bytes, 350);
    assert_eq!(after.total_items, 2);

    let excluded = plan_service::set_item_excluded(&context, scan.as_str(), &safe_id, true)
        .expect("kecualikan item aman");
    assert_eq!(excluded.total_bytes, 250);
    assert!(!excluded.requires_confirmation);
}
