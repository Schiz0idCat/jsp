use std::env;
use std::path::Path;
use std::process::ExitCode;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};

use jsp_r::domain::Jsp;
use jsp_r::solver::BruteForce;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filepath>", args[0]);
        return ExitCode::FAILURE;
    }

    let filepath = Path::new(&args[1]);

    let jsp = match Jsp::from_file(filepath) {
        Ok(jsp) => jsp,
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", filepath, err);
            return ExitCode::FAILURE;
        }
    };

    println!("{}\n", jsp);

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} Solving... elapsed: {elapsed_precise}")
            .unwrap(),
    );
    spinner.enable_steady_tick(Duration::from_millis(100));

    let solution = jsp.solve(BruteForce::new());

    spinner.finish_and_clear();

    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution"),
    }

    ExitCode::SUCCESS
}
