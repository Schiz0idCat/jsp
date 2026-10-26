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
Job Shop Scheduling Problem (JSP) Solver

Usage: jsp [OPTIONS] --file <FILE>

Options:
  -f, --file <FILE>        Ruta al archivo con el problema
  -t, --timeout <SECONDS>  Tiempo límite opcional
  -h, --help               Print help
```

El programa recibe 2 argumentos:
- '-f' 'ruta/al/archivo': Ruta a una instancia del problema. Almacenados en: [data](./data).
- '-t' 'int': Tiempo límite en segundos.

El argumento '-f' es obligatorio, pues el programa no puede resolver una instancia vacía.
Mientras que el argumento '-t' para el timeout, es opcional.

Al terminar (o al ser interrumpido), el programa muestra por consola la mejor solución obtenida.

Ejemplo:
```sh
❯ ./target/release/jsp -f data/minimo.txt -t 60
JSP Instance [3 Jobs x 3 Machines]
---------------------------------------------
       |    Op 1    |    Op 2    |    Op 3   
---------------------------------------------
Job 1  |  M0 (2h)   |  M1 (7h)   |  M2 (9h)  
Job 2  |  M1 (4h)   |  M0 (9h)   |  M2 (7h)  
Job 3  |  M0 (9h)   |  M1 (8h)   |  M2 (3h)  
---------------------------------------------

  Done in: 00:00:00                                                                                                                              Optimal Makespan: 30

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

## Autores
- Agustín Guzmán - 21.572.150-7
- Nicolás Leiva - 21.855.076-2
- Cristobal Pinto - 21.994.622-8
