#[derive(Debug, Clone)]
pub enum RBIAlert {
    CriticalDeflationary { rbi: f64, message: String },
    ModerateDeflationary { rbi: f64, message: String },
    Overheating { rbi: f64, message: String },
}

#[derive(Debug, Clone)]
pub struct AlertThresholds {
    pub critical_low: f64,  // default 0.8
    pub warning_low: f64,   // default 1.0
    pub overheating_high: f64, // default 2.0
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            critical_low: 0.8,
            warning_low: 1.0,
            overheating_high: 2.0,
        }
    }
}

pub fn evaluate_alert(rbi: f64, thresholds: &AlertThresholds) -> Option<RBIAlert> {
    if rbi.is_nan() {
        return Some(RBIAlert::ModerateDeflationary {
            rbi,
            message: "WARNING: RBI is NaN (invalid). Check data provider inputs.".to_string(),
        });
    }

    if rbi < thresholds.critical_low {
        Some(RBIAlert::CriticalDeflationary {
            rbi,
            message: "CRITICAL: RBI indicates severe deflationary risk. Consider increasing miner contributions or adjusting velocity incentives.".to_string(),
        })
    } else if rbi < thresholds.warning_low {
        Some(RBIAlert::ModerateDeflationary {
            rbi,
            message: "WARNING: RBI below 1.0 indicates deflationary pressure. Monitor closely and consider parameter adjustments.".to_string(),
        })
    } else if rbi > thresholds.overheating_high {
        Some(RBIAlert::Overheating {
            rbi,
            message: "NOTICE: RBI above 2.0 may indicate excessive distribution. Consider reducing contribution rate to avoid inflationary pressure.".to_string(),
        })
    } else {
        None
    }
}
