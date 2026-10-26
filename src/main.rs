use std::process::ExitCode;

use clap::Parser;

use jsp::domain::Jsp;
use jsp::solver::solution::BruteForce;
use jsp::solver::stop::{CancelCondition, StopHandle, TimeoutCondition};
use jsp::ui::{Cli, Progress};

fn main() -> ExitCode {
    // parseo de cli
    let cli = Cli::parse();

    // jsp parser
    let jsp = match Jsp::from_file(cli.filepath()) {
        Ok(jsp) => jsp,
        Err(err) => {
            eprintln!("Error reading file {:?}: {}", cli.filepath(), err);
            return ExitCode::FAILURE;
        }
    };

    println!("{}\n", jsp);

    // barra de progreso
    let progress = Progress::start();

    // configuración de la interrupción del programa
    let cancel = CancelCondition::handle_ctrlc().expect("Error setting Ctrl+C handler");

    // condiciones de parada
    let mut stop = StopHandle::new().with_condition(cancel.clone());

    if let Some(timeout) = cli.timeout() {
        stop = stop.with_condition(TimeoutCondition::new(timeout));
    }

    // algoritmo de solución
    let solution = jsp.solve_with_stop(BruteForce::new(), &mut stop);

    progress.finish(&cancel); // limpieza de la barra de progreso

    // muestra la solución encontrada
    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution found"),
    }

    ExitCode::SUCCESS
}
