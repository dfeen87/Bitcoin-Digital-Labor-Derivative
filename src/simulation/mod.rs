pub mod invariants;
pub mod report;
pub mod scenarios;
pub mod state;
pub mod step;

use crate::alerts::AlertThresholds;
use crate::simulation::invariants::evaluate_invariants;
use crate::simulation::report::{ScenarioReport, SimulationReport, StepReport};
use crate::simulation::scenarios::SimulationScenario;
use crate::simulation::step::execute_step;
use crate::velocity_config::VelocityConfig;
use std::collections::BTreeMap;

pub fn run_scenario(scenario: &SimulationScenario) -> ScenarioReport {
    let cfg = VelocityConfig::default();
    let thresholds = AlertThresholds::default();

    let mut steps = Vec::new();
    let mut invariants = Vec::new();

    for step_input in &scenario.steps {
        let execution = execute_step(step_input, &cfg, &thresholds);
        invariants.extend(evaluate_invariants(&execution));

        let alert = execution
            .rbi_snapshot
            .as_ref()
            .and_then(|snapshot| snapshot.alert.as_ref())
            .map(|alert| format!("{alert:?}"));

        let step_report = StepReport {
            step_index: execution.step_index,
            block_height: execution.block_height,
            participant_count: execution.participant_count,
            total_stake_sats: execution.total_stake_sats,
            average_velocity: execution.average_velocity,
            rbi_value: execution
                .rbi_snapshot
                .as_ref()
                .map(|snapshot| snapshot.rbi_value),
            rbi_status: execution
                .rbi_snapshot
                .as_ref()
                .map(|snapshot| snapshot.status),
            is_healthy: execution
                .rbi_snapshot
                .as_ref()
                .map(|snapshot| snapshot.is_healthy),
            demand_shock: execution.rbi_snapshot.as_ref().map(|snapshot| snapshot.d_s),
            productivity_a: execution
                .rbi_snapshot
                .as_ref()
                .map(|snapshot| snapshot.productivity_a),
            alert,
            error: execution.error.clone(),
        };

        steps.push(step_report);
    }

    steps.sort_by(|a, b| a.step_index.cmp(&b.step_index));
    invariants.sort_by(|a, b| {
        a.step_index
            .cmp(&b.step_index)
            .then_with(|| a.kind.cmp(&b.kind))
    });

    ScenarioReport {
        name: scenario.name.clone(),
        steps,
        invariants,
    }
}

pub fn run_all_scenarios() -> SimulationReport {
    let scenarios = scenarios::all_scenarios();
    let mut reports = BTreeMap::new();
    for scenario in scenarios {
        let report = run_scenario(&scenario);
        reports.insert(scenario.name.clone(), report);
    }
    SimulationReport { scenarios: reports }
}
