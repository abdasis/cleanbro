use serde::{Deserialize, Serialize};

/// Ambang peringatan ruang disk (FR-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WarnLevel {
    Ok,
    Warn,
    Critical,
}

impl WarnLevel {
    /// Sisa < 3 persen merah, < 10 persen kuning, selebihnya aman.
    pub fn for_free_percent(free_percent: f64) -> Self {
        if free_percent < 3.0 {
            Self::Critical
        } else if free_percent < 10.0 {
            Self::Warn
        } else {
            Self::Ok
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DiskSummary {
    pub total: u64,
    pub used: u64,
    pub free: u64,
    pub percent: f64,
    pub mount: String,
    pub warn_level: WarnLevel,
}

impl DiskSummary {
    /// `free` adalah ruang tersedia untuk pengguna (bukan sekadar free blok).
    pub fn from_space(total: u64, free: u64, mount: impl Into<String>) -> Self {
        let used = total.saturating_sub(free);
        let percent = if total == 0 {
            0.0
        } else {
            used as f64 / total as f64 * 100.0
        };
        let free_percent = if total == 0 {
            0.0
        } else {
            free as f64 / total as f64 * 100.0
        };
        Self {
            total,
            used,
            free,
            percent,
            mount: mount.into(),
            warn_level: WarnLevel::for_free_percent(free_percent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_warn_level_thresholds() {
        assert_eq!(WarnLevel::for_free_percent(50.0), WarnLevel::Ok);
        assert_eq!(WarnLevel::for_free_percent(10.0), WarnLevel::Ok);
        assert_eq!(WarnLevel::for_free_percent(9.99), WarnLevel::Warn);
        assert_eq!(WarnLevel::for_free_percent(3.0), WarnLevel::Warn);
        assert_eq!(WarnLevel::for_free_percent(2.99), WarnLevel::Critical);
        assert_eq!(WarnLevel::for_free_percent(0.0), WarnLevel::Critical);
    }

    #[test]
    fn summary_computes_used_and_percent() {
        let summary = DiskSummary::from_space(1000, 100, "/");
        assert_eq!(summary.used, 900);
        assert_eq!(summary.free, 100);
        assert!((summary.percent - 90.0).abs() < f64::EPSILON);
        assert_eq!(summary.warn_level, WarnLevel::Ok);
        assert_eq!(serde_json::to_value(&summary).unwrap()["warn_level"], "ok");
    }
}
