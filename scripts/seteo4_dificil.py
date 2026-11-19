#!/usr/bin/env python3
"""Traza del Seteo 4 del algoritmo genético sobre una instancia JSP.

Reproduce la configuración del Seteo 4 del notebook `extra/genetico.ipynb`
y registra el mejor makespan acumulado después de cada evaluación.
"""

import argparse
import csv
import random
from pathlib import Path


def cargar_instancia(filepath):
    """Lee una instancia JSP con el formato del proyecto."""
    with open(filepath, "r", encoding="utf-8") as file:
        lines = [
            line.strip()
            for line in file
            if line.strip() and not line.startswith("#")
        ]

    num_trabajos, num_maquinas = map(int, lines[0].split())
    matriz_tm = []

    for line in lines[1 : num_trabajos + 1]:
        partes = list(map(int, line.split()))
        trabajo = [
            (partes[i], partes[i + 1])
            for i in range(0, len(partes), 2)
        ]
        matriz_tm.append(trabajo)

    return matriz_tm, num_trabajos, num_maquinas


def makespan(matriz_tm, orden, num_maquinas):
    """Calcula el makespan de una secuencia de trabajos."""
    cantidad_trabajos = len(matriz_tm)
    t_maquinas = [0] * num_maquinas
    t_trabajos = [0] * cantidad_trabajos
    proceso_trabajos = [0] * cantidad_trabajos

    for trabajo in orden:
        etapa = proceso_trabajos[trabajo]
        proceso_trabajos[trabajo] = etapa + 1

        maquina, duracion = matriz_tm[trabajo][etapa]
        t_fin = max(t_maquinas[maquina], t_trabajos[trabajo]) + duracion

        t_maquinas[maquina] = t_fin
        t_trabajos[trabajo] = t_fin

    return max(t_trabajos)


def generar_individuo_aleatorio(num_trabajos, num_maquinas):
    """Genera una secuencia válida con cada trabajo repetido M veces."""
    individuo = []
    for trabajo in range(num_trabajos):
        individuo.extend([trabajo] * num_maquinas)
    random.shuffle(individuo)
    return individuo


def calcular_pesos_ruleta(costos):
    """Calcula los pesos de la ruleta para minimización."""
    max_fitness = max(costos)
    min_fitness = min(costos)

    if max_fitness == min_fitness:
        return [1.0] * len(costos)

    return [((max_fitness - costo) + 1.0) ** 2 for costo in costos]


def seleccion_ruleta(poblacion, k=2):
    """Selecciona k padres mediante ruleta."""
    costos = [individuo[0] for individuo in poblacion]
    pesos = calcular_pesos_ruleta(costos)
    seleccionados = random.choices(poblacion, weights=pesos, k=k)
    return [individuo[1] for individuo in seleccionados]


def cruzamiento_pox(padre1, padre2, num_trabajos):
    """Aplica Precedence Operation Crossover (POX)."""
    trabajos = list(range(num_trabajos))
    random.shuffle(trabajos)
    corte = random.randint(1, max(1, num_trabajos - 1))
    j1 = set(trabajos[:corte])

    n = len(padre1)
    hijo1 = [-1] * n
    hijo2 = [-1] * n

    for idx in range(n):
        p1_val = padre1[idx]
        p2_val = padre2[idx]

        if p1_val in j1:
            hijo1[idx] = p1_val
        if p2_val in j1:
            hijo2[idx] = p2_val

    iter_p2 = (op for op in padre2 if op not in j1)
    iter_p1 = (op for op in padre1 if op not in j1)

    for idx in range(n):
        if hijo1[idx] == -1:
            hijo1[idx] = next(iter_p2)
        if hijo2[idx] == -1:
            hijo2[idx] = next(iter_p1)

    return hijo1, hijo2


def mutacion_swap(individuo, prob_mutacion=0.30):
    """Aplica el operador Swap del Seteo 4."""
    mutado = list(individuo)

    if random.random() < prob_mutacion:
        n = len(mutado)
        num_swaps = 2 if n >= 150 else 1

        for _ in range(num_swaps):
            for _ in range(10):
                idx1, idx2 = random.sample(range(n), 2)
                if mutado[idx1] != mutado[idx2]:
                    mutado[idx1], mutado[idx2] = (
                        mutado[idx2],
                        mutado[idx1],
                    )
                    break

    return mutado


def ag_memetico_elite_jsp(
    matriz_tm,
    num_trabajos,
    num_maquinas,
    mu=30,
    lam=30,
    elitismo=0.20,
    prob_mutacion=0.30,
    max_evals=27_550,
    pasos_elite=40,
):
    """Ejecuta Seteo 4 y registra el mejor makespan por evaluación."""
    evaluaciones = 0
    n_ops = num_trabajos * num_maquinas
    trace = []
    mejor_global = float("inf")

    def evaluar(orden):
        nonlocal evaluaciones, mejor_global

        costo = makespan(matriz_tm, orden, num_maquinas)
        evaluaciones += 1

        if costo < mejor_global:
            mejor_global = costo

        trace.append((evaluaciones, mejor_global))
        return costo

    candidatos_init = []

    for _ in range(mu * 2):
        genotipo = generar_individuo_aleatorio(num_trabajos, num_maquinas)
        costo = evaluar(genotipo)
        candidatos_init.append((costo, genotipo))

    candidatos_init.sort(key=lambda individuo: individuo[0])
    poblacion = candidatos_init[:mu]
    mejor_historico = poblacion[0]
    num_elites = max(1, int(mu * elitismo))

    while evaluaciones < max_evals:
        hijos = []

        while len(hijos) < lam and evaluaciones < max_evals:
            padre1, padre2 = seleccion_ruleta(poblacion, k=2)
            hijo1, hijo2 = cruzamiento_pox(padre1, padre2, num_trabajos)

            for hijo in (hijo1, hijo2):
                if len(hijos) < lam and evaluaciones < max_evals:
                    hijo_mutado = mutacion_swap(hijo, prob_mutacion)
                    costo_hijo = evaluar(hijo_mutado)
                    hijos.append((costo_hijo, hijo_mutado))

        combinados = poblacion + hijos
        combinados.sort(key=lambda individuo: individuo[0])

        mejor_gen_costo = combinados[0][0]
        mejor_gen_seq = list(combinados[0][1])

        for _ in range(pasos_elite):
            if evaluaciones >= max_evals:
                break

            idx1 = random.randrange(n_ops)
            idx2 = random.randrange(n_ops)

            if mejor_gen_seq[idx1] != mejor_gen_seq[idx2]:
                mejor_gen_seq[idx1], mejor_gen_seq[idx2] = (
                    mejor_gen_seq[idx2],
                    mejor_gen_seq[idx1],
                )
                costo_vecino = evaluar(mejor_gen_seq)

                if costo_vecino <= mejor_gen_costo:
                    mejor_gen_costo = costo_vecino
                else:
                    mejor_gen_seq[idx1], mejor_gen_seq[idx2] = (
                        mejor_gen_seq[idx2],
                        mejor_gen_seq[idx1],
                    )

        combinados[0] = (mejor_gen_costo, mejor_gen_seq)

        if mejor_gen_costo < mejor_historico[0]:
            mejor_historico = combinados[0]

        nueva_poblacion = []
        vistos = set()
        resto_candidatos = []

        for individuo in combinados:
            clave = tuple(individuo[1])
            if len(nueva_poblacion) < num_elites and clave not in vistos:
                nueva_poblacion.append(individuo)
                vistos.add(clave)
            else:
                resto_candidatos.append(individuo)

        cupos_restantes = mu - len(nueva_poblacion)
        costos_resto = [individuo[0] for individuo in resto_candidatos]
        pesos_resto = calcular_pesos_ruleta(costos_resto)

        elegidos = random.choices(
            resto_candidatos,
            weights=pesos_resto,
            k=cupos_restantes,
        )
        nueva_poblacion.extend(elegidos)
        poblacion = nueva_poblacion

    return trace, mejor_historico[1], mejor_historico[0], evaluaciones


def main():
    parser = argparse.ArgumentParser(
        description="Traza del Seteo 4 del algoritmo genético sobre JSP."
    )
    parser.add_argument(
        "--instance",
        default="data/dificil.txt",
        help="Instancia a resolver",
    )
    parser.add_argument(
        "--seed",
        type=int,
        default=101,
        help="Semilla de Python usada por el Seteo 4",
    )
    parser.add_argument(
        "--max-evals",
        type=int,
        default=27_550,
        help="Presupuesto de evaluaciones del Seteo 4",
    )
    parser.add_argument(
        "--output",
        default="docs/informe/Tarea2/datos/iteraciones_dificil_genetico_seteo4.csv",
        help="Archivo CSV de salida",
    )

    args = parser.parse_args()

    matriz_tm, num_trabajos, num_maquinas = cargar_instancia(args.instance)
    random.seed(args.seed)

    trace, mejor_secuencia, mejor_costo, evaluaciones = ag_memetico_elite_jsp(
        matriz_tm,
        num_trabajos,
        num_maquinas,
        max_evals=args.max_evals,
    )

    output = Path(args.output)
    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("w", newline="", encoding="utf-8") as file:
        writer = csv.writer(file)
        writer.writerow(["iteracion", "makespan"])

        for iteracion, makespan_actual in trace:
            writer.writerow([iteracion, makespan_actual])

    print(f"Semilla: {args.seed}")
    print(f"Evaluaciones: {evaluaciones}")
    print(f"Mejor makespan: {mejor_costo}")
    print(f"CSV: {output}")
    print(f"Secuencia: {mejor_secuencia}")


if __name__ == "__main__":
    main()
