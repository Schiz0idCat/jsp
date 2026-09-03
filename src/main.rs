use std::process::ExitCode;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use jsp::domain::Jsp;
use jsp::solver::solution::BruteForce;
use jsp::solver::stop::{CancelTokenCondition, StopHandle, TimeoutCondition};
use jsp::ui::Cli;

fn main() -> ExitCode {
    // cli
    let cli = Cli::parse();
    let filepath = cli.filepath();

    // jsp parser
    let jsp = match Jsp::from_file(filepath) {
        Ok(jsp) => jsp,
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", filepath, err);
            return ExitCode::FAILURE;
        }
    };

    println!("{}\n", jsp);

    let cancel_token = Arc::new(AtomicBool::new(false));
    let ctrlc_token = Arc::clone(&cancel_token);

    if let Err(err) = ctrlc::set_handler(move || {
        ctrlc_token.store(true, Ordering::Relaxed);
    }) {
        eprintln!("Error setting Ctrl+C handler: {}", err);
        return ExitCode::FAILURE;
    }

    // barra de progreso
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} Solving... elapsed: {elapsed_precise}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let mut stop =
        StopHandle::new().with_condition(CancelTokenCondition::new(Arc::clone(&cancel_token)));

    if let Some(timeout) = cli.timeout() {
        stop = stop.with_condition(TimeoutCondition::new(timeout));
    }

    let solution = jsp.solve_with_stop(BruteForce::new(), &mut stop);

    spinner.finish_and_clear();

    if cancel_token.load(Ordering::Relaxed) {
        println!("\n[!] Process interrupted by user. Best solution found so far:");
    }

    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution found"),
    }

    ExitCode::SUCCESS
}
