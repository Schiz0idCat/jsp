mod benchmark_result;
mod instance;

use benchmark_result::BenchmarkResult;
use instance::Instance;

use std::process::Command;

const RUNS_PER_INSTANCE: usize = 10;

fn main() {
    let instances = [
        Instance::new("Fácil", "data/facil.txt"),
        Instance::new("Medio", "data/medio.txt"),
        Instance::new("Difícil", "data/dificil.txt"),
    ];

    let binary_path = "./target/release/jsp";

    println!("////////////////////////////////////////////////////");
    println!("/////   INICIANDO EXPERIMENTACIÓN DE TAREA 2   /////");
    println!("////////////////////////////////////////////////////\n");

    let mut results: Vec<BenchmarkResult> = Vec::new();

    for instance in &instances {
        println!(
            "Ejecutando instancia: {} ({})",
            instance.name(),
            instance.path()
        );

        let mut makespans: Vec<f64> = Vec::with_capacity(RUNS_PER_INSTANCE);

        for run in 1..=RUNS_PER_INSTANCE {
            // Ejecuta la CLI
            let output = Command::new(binary_path)
                .arg("-f")
                .arg(instance.path())
                .arg("simulated-annealing")
                .output()
                .expect("Error al ejecutar el binario de JSP");

            if !output.status.success() {
                eprintln!("Error en la ejecución {} de {}", run, instance.name());
                break;
            }

            let stdout = String::from_utf8_lossy(&output.stdout);

            // Extrae el makespan del output de la CLI
            if let Some(makespan) = parse_makespan(&stdout) {
                println!("  Ejecución {:2}/10 -> Makespan: {}", run, makespan);
                makespans.push(makespan);
            } else {
                eprintln!("  Ejecución {:2}/10 -> No se pudo parsear el makespan", run);
                break;
            }
        }

        if let Some(result) = BenchmarkResult::new(instance.name(), makespans) {
            print!("{}", result);
            results.push(result);
        } else {
            eprintln!(
                "No se registraron ejecuciones válidas para {}\n",
                instance.name()
            );
        }
    }
}

/// Extrae el valor numérico del makespan a partir de la salida en consola.
fn parse_makespan(output: &str) -> Option<f64> {
    for line in output.lines() {
        let clean_line = line.replace('\r', "");

        if clean_line.contains("Makespan:") {
            if let Some(after_colon) = clean_line.split("Makespan:").nth(1) {
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
    }

    None
}
