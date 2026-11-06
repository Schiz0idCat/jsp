use std::process::ExitCode;

use clap::Parser;

use jsp::solver::RandomSeed;
use jsp::ui::{Cli, Progress, SolverData};

fn main() -> ExitCode {
    // parseo de cli
    let cli = Cli::parse();

    let seed = cli.seed().unwrap_or_else(RandomSeed::generate);
    println!("Seed: {}\n", seed);

    let mut solver_data = match SolverData::new(cli.filepath(), cli.timeout(), cli.strategy(), seed)
    {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    println!("{}\n", solver_data.jsp());

    // barra de progreso
    let progress = Progress::start();

    // algoritmo de solución
    let solution = solver_data.solve();

    progress.finish(solver_data.cancel_handle()); // limpieza de la barra de progreso

    // muestra la solución encontrada
    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution found"),
    }

    ExitCode::SUCCESS
}
