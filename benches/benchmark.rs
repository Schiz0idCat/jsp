//! Benchmark reproducible de las instancias del informe.
//!
//! Ejecuta la CLI con las semillas `2026..2035` para cada instancia,
//! recoge los makespan y escribe los resultados en CSV para que pgfplots
//! y el informe los consuman directamente.
//!
//! Además, genera una traza del mejor makespan acumulado en cada iteración
//! de Simulated Annealing para cada instancia.

mod benchmark_result;
mod instance;

use benchmark_result::BenchmarkResult;
use instance::Instance;

use jsp::domain::Jsp;
use jsp::solver::RandomSeed;
use jsp::solver::solution::strategies::SimulatedAnnealing;
use jsp::solver::stop::SearchControl;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Cantidad de ejecuciones independientes por instancia.
const RUNS_PER_INSTANCE: usize = 10;

/// Primera semilla de la secuencia reproducible.
const BASE_SEED: u64 = 2026;

/// Parámetros por defecto de Simulated Annealing.
const SA_T_INITIAL: f64 = 1000.0;
const SA_T_MIN: f64 = 0.001;
const SA_ALPHA: f64 = 0.9995;

/// Directorio donde se escriben los CSV del informe.
const RESULTS_DIR: &str = "docs/informe/Tarea2/datos";

/// Datos de una ejecución individual.
#[derive(Clone, Copy)]
struct Run {
    /// Número de ejecución dentro de la instancia.
    execution: usize,

    /// Semilla usada por la CLI.
    seed: u64,

    /// Makespan obtenido.
    makespan: f64,
}

/// Mejor makespan acumulado después de una iteración de Simulated Annealing.
#[derive(Clone, Copy)]
struct IterationSample {
    /// Número de iteración, comenzando en 1.
    iteration: u64,

    /// Mejor makespan encontrado hasta esa iteración.
    makespan: usize,
}

/// Punto de entrada del benchmark.
fn main() {
    let instances = [
        Instance::new(1, "Fácil", "facil", "data/facil.txt"),
        Instance::new(2, "Medio", "medio", "data/medio.txt"),
        Instance::new(3, "Difícil", "dificil", "data/dificil.txt"),
    ];

    let binary_path = "./target/release/jsp";
    let results_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(RESULTS_DIR);

    println!(
        "Semillas: {} a {}\n",
        BASE_SEED,
        BASE_SEED + RUNS_PER_INSTANCE as u64 - 1
    );

    let mut all_runs: Vec<(Instance, Run)> = Vec::new();
    let mut results: Vec<(Instance, BenchmarkResult)> = Vec::new();

    for instance in &instances {
        println!(
            "Ejecutando instancia: {} ({})",
            instance.name(),
            instance.path()
        );

        let mut runs: Vec<Run> = Vec::with_capacity(RUNS_PER_INSTANCE);

        for execution in 1..=RUNS_PER_INSTANCE {
            let seed = RandomSeed::from_u64(BASE_SEED + execution as u64 - 1);

            // Ejecuta la CLI con una semilla explícita.
            let output = Command::new(binary_path)
                .arg("-f")
                .arg(instance.path())
                .arg("--seed")
                .arg(seed.to_string())
                .arg("simulated-annealing")
                .output()
                .expect("Error al ejecutar el binario de JSP");

            if !output.status.success() {
                eprintln!("Error en la ejecución {} de {}", execution, instance.name());
                break;
            }

            let stdout = String::from_utf8_lossy(&output.stdout);

            // Verifica que la CLI haya usado la semilla solicitada.
            if parse_seed(&stdout) != Some(seed.value()) {
                eprintln!(
                    "  Ejecución {:2}/10 -> La semilla impresa no coincide con {}",
                    execution, seed
                );
                break;
            }

            // Extrae el makespan del output de la CLI.
            if let Some(makespan) = parse_makespan(&stdout) {
                println!(
                    "  Ejecución {:2}/10 -> Semilla: {} | Makespan: {}",
                    execution, seed, makespan
                );
                runs.push(Run {
                    execution,
                    seed: seed.value(),
                    makespan,
                });
            } else {
                eprintln!(
                    "  Ejecución {:2}/10 -> No se pudo parsear el makespan",
                    execution
                );
                break;
            }
        }

        let makespans: Vec<f64> = runs.iter().map(|run| run.makespan).collect();

        if let Some(result) = BenchmarkResult::new(instance.name(), makespans) {
            print!("{}", result);
            all_runs.extend(runs.iter().map(|run| (*instance, *run)));
            results.push((*instance, result));
        } else {
            eprintln!(
                "No se registraron ejecuciones válidas para {}\n",
                instance.name()
            );
        }
    }

    if let Err(err) = write_results(&results_dir, &all_runs, &results) {
        eprintln!("No se pudieron escribir los resultados en CSV: {err}");
    } else {
        println!("Resultados escritos en: {}\n", results_dir.display());
    }

    println!("Generando trazas de iteraciones...");

    for instance in &instances {
        match write_iteration_trace(instance, RandomSeed::from_u64(BASE_SEED), &results_dir) {
            Ok(()) => println!(
                "  {} -> {}",
                instance.name(),
                results_dir
                    .join(format!("iteraciones_{}.csv", instance.slug()))
                    .display()
            ),
            Err(err) => eprintln!(
                "  No se pudo generar la traza de {}: {err}",
                instance.name()
            ),
        }
    }
}

/// Ejecuta Simulated Annealing y escribe el mejor makespan de cada iteración.
///
/// El CSV resultante se ordena únicamente por número de iteración ascendente.
fn write_iteration_trace(instance: &Instance, seed: RandomSeed, dir: &Path) -> io::Result<()> {
    let jsp_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(instance.path());
    let jsp = Jsp::from_file(jsp_path).map_err(|error| io::Error::other(error.to_string()))?;

    let solver = SimulatedAnnealing::new(SA_T_INITIAL, SA_T_MIN, SA_ALPHA, seed);
    let mut control = SearchControl::default();
    let mut samples = Vec::new();

    let schedule = solver.solve_with_trace(&jsp, &mut control, |iteration, makespan| {
        samples.push(IterationSample {
            iteration,
            makespan,
        });
    });

    if schedule.is_none() {
        return Err(io::Error::other(format!(
            "Simulated Annealing no produjo una solución para {}",
            instance.name()
        )));
    }

    // Orden solicitado: solo por número de iteración.
    samples.sort_by_key(|sample| sample.iteration);

    write_iteration_csv(
        &dir.join(format!("iteraciones_{}.csv", instance.slug())),
        &samples,
    )
}

/// Escribe las muestras de iteraciones en formato CSV.
fn write_iteration_csv(path: &Path, samples: &[IterationSample]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "iteracion,makespan")?;

    for sample in samples {
        writeln!(writer, "{},{}", sample.iteration, sample.makespan)?;
    }

    writer.flush()
}

/// Escribe los datos crudos y el resumen estadístico en formato CSV.
fn write_results(
    dir: &Path,
    all_runs: &[(Instance, Run)],
    results: &[(Instance, BenchmarkResult)],
) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    write_combined_csv(&dir.join("benchmark.csv"), all_runs)?;
    write_summary_csv(&dir.join("benchmark_resumen.csv"), results)?;

    for (instance, _) in results {
        // Filtra las ejecuciones que pertenecen a la instancia actual.
        let runs: Vec<Run> = all_runs
            .iter()
            .filter_map(|(current, run)| (current.id() == instance.id()).then_some(*run))
            .collect();
        write_instance_csv(
            &dir.join(format!("benchmark_{}.csv", instance.slug())),
            &runs,
        )?;
    }

    Ok(())
}

/// Escribe todas las ejecuciones en un único archivo.
fn write_combined_csv(path: &Path, all_runs: &[(Instance, Run)]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "instancia,id,ejecucion,semilla,makespan")?;

    for (instance, run) in all_runs {
        writeln!(
            writer,
            "{},{},{},{},{}",
            instance.name(),
            instance.id(),
            run.execution,
            run.seed,
            run.makespan
        )?;
    }

    writer.flush()
}

/// Escribe las ejecuciones de una instancia para que pgfplots las consuma.
///
/// La columna `x` distribuye los puntos alrededor del centro de la caja para
/// que las ejecuciones con el mismo makespan no queden exactamente superpuestas.
fn write_instance_csv(path: &Path, runs: &[Run]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(writer, "ejecucion,semilla,makespan,x")?;

    let center = (RUNS_PER_INSTANCE as f64 + 1.0) / 2.0;

    for run in runs {
        let x = 1.0 + (run.execution as f64 - center) * 0.025;
        writeln!(
            writer,
            "{},{},{},{:.4}",
            run.execution, run.seed, run.makespan, x
        )?;
    }

    writer.flush()
}

/// Escribe el resumen estadístico por instancia.
fn write_summary_csv(path: &Path, results: &[(Instance, BenchmarkResult)]) -> io::Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);
    writeln!(
        writer,
        "instancia,id,muestras,minimo,maximo,media,desviacion"
    )?;

    for (instance, result) in results {
        if result.is_empty() {
            continue;
        }

        writeln!(
            writer,
            "{},{},{},{:.0},{:.0},{:.2},{:.2}",
            instance.name(),
            instance.id(),
            result.len(),
            result.min(),
            result.max(),
            result.mean(),
            result.std_dev()
        )?;
    }

    writer.flush()
}

/// Extrae la semilla impresa por la CLI.
fn parse_seed(output: &str) -> Option<u64> {
    output.lines().find_map(|line| {
        let line = line.trim();
        line.strip_prefix("Semilla:")
            .or_else(|| line.strip_prefix("Seed:"))
            .and_then(|value| value.trim().parse().ok())
    })
}

/// Extrae el valor numérico del makespan desde la salida en consola.
fn parse_makespan(output: &str) -> Option<f64> {
    for line in output.lines() {
        let clean_line = line.replace('\r', "");

        if clean_line.contains("Makespan:")
            && let Some(after_colon) = clean_line.split("Makespan:").nth(1)
        {
            // Toma los primeros dígitos numéricos encontrados después de ':'.
            let number_str: String = after_colon
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .collect();

            if let Ok(value) = number_str.parse::<f64>() {
                return Some(value);
            }
        }
    }

    None
}
