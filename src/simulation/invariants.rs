use crate::rbi_engine::RbiStatus;
use crate::simulation::report::InvariantViolation;
use crate::simulation::step::StepExecution;

pub fn evaluate_invariants(step: &StepExecution) -> Vec<InvariantViolation> {
    let mut violations = Vec::new();

    if let Some(error) = &step.error {
        violations.push(InvariantViolation {
            step_index: step.step_index,
            kind: "step_error".to_string(),
            message: error.clone(),
        });
        return violations;
    }

    if let Some(snapshot) = &step.rbi_snapshot {
        if !snapshot.rbi_value.is_finite() {
            violations.push(InvariantViolation {
                step_index: step.step_index,
                kind: "rbi_non_finite".to_string(),
                message: "rbi_value must be finite".to_string(),
            });
        }
        if snapshot.d_s.abs() < f64::EPSILON
            && !matches!(
                snapshot.status,
                RbiStatus::Indeterminate | RbiStatus::Invalid
            )
        {
            violations.push(InvariantViolation {
                step_index: step.step_index,
                kind: "demand_shock_near_zero".to_string(),
                message: "near-zero demand shock must be indeterminate".to_string(),
            });
        }
        if (step.participant_count == 0 || step.total_stake_sats == 0) && snapshot.is_healthy {
            violations.push(InvariantViolation {
                step_index: step.step_index,
                kind: "zero_participation".to_string(),
                message: "zero participation/stake must not be healthy".to_string(),
            });
        }
    }

    violations
}
