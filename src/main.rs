use std::process::ExitCode;
use std::time::Duration;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};

use jsp::domain::Jsp;
use jsp::solver::BruteForce;
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

    // barra de progreso
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} Solving... elapsed: {elapsed_precise}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let solution = jsp.solve(BruteForce::new());

    spinner.finish_and_clear();

    // solución
    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution"),
    }

    ExitCode::SUCCESS
}
