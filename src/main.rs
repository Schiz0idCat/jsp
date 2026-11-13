//! Punto de entrada de la aplicación de línea de comandos.
//!
//! El flujo principal es: leer los argumentos, construir el objeto
//! [`SolverData`], ejecutar la estrategia seleccionada y mostrar la
//! programación resultante.

use std::process::ExitCode;

use clap::Parser;

use jsp::solver::RandomSeed;
use jsp::ui::{Cli, Progress, SolverData};

/// Ejecuta el solucionador y devuelve el código de salida del proceso.
fn main() -> ExitCode {
    // Parsea los argumentos de la línea de comandos.
    let cli = Cli::parse();

    // Usa la semilla indicada por el usuario o genera una nueva.
    let seed = cli.seed().unwrap_or_else(RandomSeed::generate);

    // La semilla siempre se imprime para que la ejecución sea reproducible.
    println!("Semilla: {}\n", seed);

    let mut solver_data = match SolverData::new(cli.filepath(), cli.timeout(), cli.strategy(), seed)
    {
        Ok(data) => data,
        Err(err) => {
            eprintln!("{}", err);
            return ExitCode::FAILURE;
        }
    };

    // Muestra la instancia cargada antes de comenzar la búsqueda.
    println!("{}\n", solver_data.jsp());

    let progress = Progress::start();

    // Ejecuta la estrategia de solución seleccionada.
    let solution = solver_data.solve();

    // Detiene la animación y deja el estado final en la terminal.
    progress.finish(solver_data.cancel_handle());

    // Muestra la mejor programación encontrada, si existe.
    match solution {
        Some(schedule) => println!("{}", schedule),
        None => println!("No solution found"),
    }

    ExitCode::SUCCESS
}
