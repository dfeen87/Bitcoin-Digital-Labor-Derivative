use bitcoin_digital_labor_derivative::simulation::scenarios::{
    address_reuse_sybil_attempt, demand_shock_near_zero, future_height_utxo_corruption,
    zero_participation_zero_stake,
};
use bitcoin_digital_labor_derivative::simulation::{run_all_scenarios, run_scenario};

#[test]
fn simulation_reports_are_deterministic() {
    let report_a = run_all_scenarios().to_json().unwrap();
    let report_b = run_all_scenarios().to_json().unwrap();
    assert_eq!(report_a.as_bytes(), report_b.as_bytes());
}

#[test]
fn demand_shock_near_zero_not_healthy() {
    let scenario = demand_shock_near_zero();
    let report = run_scenario(&scenario);
    for step in report.steps {
        assert_eq!(step.is_healthy, Some(false));
        assert!(step.rbi_status.is_some());
    }
}

#[test]
fn zero_participants_not_healthy() {
    let scenario = zero_participation_zero_stake();
    let report = run_scenario(&scenario);
    for step in report.steps {
        assert_eq!(step.participant_count, 0);
        assert_eq!(step.total_stake_sats, 0);
        assert_eq!(step.is_healthy, Some(false));
    }
}

#[test]
fn future_height_utxo_rejected() {
    let scenario = future_height_utxo_corruption();
    let report = run_scenario(&scenario);
    let step = &report.steps[0];
    let error = step.error.as_ref().expect("expected error");
    assert!(error.contains("exceeds current height"));
}

#[test]
fn address_reuse_rejected() {
    let scenario = address_reuse_sybil_attempt();
    let report = run_scenario(&scenario);
    let step = &report.steps[0];
    let error = step.error.as_ref().expect("expected error");
    assert!(error.contains("address reused"));
}
