use bitcoin_digital_labor_derivative::simulation::run_all_scenarios;

fn main() {
    let report = run_all_scenarios();
    match report.to_json() {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize report: {err}");
            std::process::exit(1);
        }
    }
}
