mod benchmark_result;
mod instance;

use benchmark_result::BenchmarkResult;
use instance::Instance;

use jsp::solver::RandomSeed;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

const RUNS_PER_INSTANCE: usize = 10;
const BASE_SEED: u64 = 2026;
const RESULTS_DIR: &str = "docs/informe/Tarea2/datos";

#[derive(Clone, Copy)]
struct Run {
    execution: usize,
    seed: u64,
    makespan: f64,
}

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

            // Extrae el makespan del output de la CLI
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
        line.trim()
            .strip_prefix("Semilla:")
            .and_then(|value| value.trim().parse().ok())
    })
}

/// Extrae el valor numérico del makespan a partir de la salida en consola.
fn parse_makespan(output: &str) -> Option<f64> {
    for line in output.lines() {
        let clean_line = line.replace('\r', "");

        if clean_line.contains("Makespan:")
            && let Some(after_colon) = clean_line.split("Makespan:").nth(1)
        {
            // Extrae los primeros dígitos numéricos encontrados después de ':'
            let number_str: String = after_colon
                .chars()
                .skip_while(|c| !c.is_numeric())
                .take_while(|c| c.is_numeric())
                .collect();

            if let Ok(val) = number_str.parse::<f64>() {
                return Some(val);
            }
        }
    }

    None
}
