# Job Shop Problem
Este código resuelve el problema de optimización ***Job Shop Problem (JSP)***.

## Descarga y ejecución
```sh
git clone https://github.com/Schiz0idCat/jsp.git    # Clonar el repositorio
cd jsp                                              # Entrar a la carpeta
```

```sh
cargo build --release   # Compilación
./target/release/jsp    # Ejecución (Por defecto, muestra el menú de ayuda)
```

### Guía de uso
```sh
❯ ./target/release/jsp --help
Job Shop Problem (JSP) Solver

Usage: jsp [OPTIONS] --file <FILE> <COMMAND>

Commands:
  bruteforce
  random
  swap
  local-search
  simulated-annealing
  help                  Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>        Ruta al archivo con el problema
  -t, --timeout <SECONDS>  Tiempo límite opcional
      --seed <U64>         Semilla aleatoria para reproducir la ejecución
  -h, --help               Print help
```

Argumentos generales:
- `-f, --file <ruta>`: ruta a una instancia del problema. Las instancias de ejemplo están en [data](./data).
- `-t, --timeout <int>`: tiempo límite en segundos. Es opcional.
- `--seed <U64>`: semilla para las estrategias que usan aleatoriedad. Es opcional.
  Si se omite, se genera una semilla a partir de la entropía del sistema.
  La semilla utilizada **siempre se imprime** en la primera línea de la salida.

Subcomandos:
- `bruteforce`: búsqueda exhaustiva para encontrar la solución óptima. No usa aleatoriedad.
- `random`: búsqueda aleatoria basada en muestreo (`-s, --samples`).
- `swap`: genera un vecino mediante un intercambio aleatorio.
- `local-search`: búsqueda local por primera mejora (`-s, --samples`).
- `simulated-annealing`: metaheurística Simulated Annealing (`-i`, `-m`, `-a`).

Nota:
Algunos subcomandos tienen sus propios argumentos. Para más información:
```sh
❯ ./target/release/jsp <COMMAND> --help
```

Ejemplos de uso:
```sh
# Ejecución reproducible de Simulated Annealing
❯ ./target/release/jsp -f data/facil.txt --seed 2026 simulated-annealing

# Con parámetros personalizados
❯ ./target/release/jsp -f data/medio.txt --seed 2027 simulated-annealing \
    -i 1000.0 -m 0.001 -a 0.9995

# Estrategia determinista con tiempo límite
❯ ./target/release/jsp -f data/facil.txt -t 60 bruteforce

# Muestreo aleatorio reproducible
❯ ./target/release/jsp -f data/facil.txt --seed 2026 random -s 99999999
```

El argumento `-f` es obligatorio, pues el programa no puede resolver una instancia vacía.
Los argumentos `-t` y `--seed` son opcionales.

Al terminar (o al ser interrumpido), el programa muestra por consola la semilla utilizada
y la mejor solución obtenida.

Ejemplo:
```sh
❯ ./target/release/jsp -f data/minimo.txt --seed 42 bruteforce
Semilla: 42

JSP Instance [3 Jobs x 3 Machines]
---------------------------------------------
       |    Op 1    |    Op 2    |    Op 3
---------------------------------------------
Job 1  |  M0 (2h)   |  M1 (7h)   |  M2 (9h)
Job 2  |  M1 (4h)   |  M0 (9h)   |  M2 (7h)
Job 3  |  M0 (9h)   |  M1 (8h)   |  M2 (3h)
---------------------------------------------

Optimal Makespan: 30

Job   Op    Machine   Start   End     Duration
--------------------------------------------------
1     1     0         0       2       2
1     2     1         4       11      7
1     3     2         11      20      9
2     1     1         0       4       4
2     2     0         11      20      9
2     3     2         20      27      7
3     1     0         2       11      9
3     2     1         11      19      8
3     3     2         27      30      3
```

## Benchmark
El binario `benchmark` ejecuta diez repeticiones de Simulated Annealing por instancia,
usando las semillas `2026` a `2035`. Para cada repetición imprime la semilla y el makespan,
y al final reporta el mejor valor, el peor valor, la media y la desviación estándar.

```sh
cargo build --release
./target/release/benchmark
```

## Autores
- Agustín Guzmán - 21.572.150-7
- Nicolás Leiva - 21.855.076-2
- Cristobal Pinto - 21.994.622-8
