from typing import TypedDict

from jsp.domain.jsp import Jsp


class ScheduleItem(TypedDict):
    job: int
    op: int
    machine: int
    start: int
    end: int
    duration: int


def calculate_initial_upper_bound(
    jobs_ops: list[list[tuple[int, int]]], n_jobs: int, max_machine_id: int
) -> int:
    """Calcula una cota superior inicial rápida usando una estrategia codiciosa (Greedy/FIFO)."""
    job_end_time = [0] * n_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * n_jobs

    # Generamos una secuencia determinista simple pasando por cada trabajo en orden
    total_ops = sum(len(ops) for ops in jobs_ops)
    
    for _ in range(total_ops):
        # Elegimos el primer trabajo con operaciones pendientes
        for job_idx in range(n_jobs):
            if job_op_index[job_idx] < len(jobs_ops[job_idx]):
                op_idx = job_op_index[job_idx]
                machine, duration = jobs_ops[job_idx][op_idx]

                start_time = max(job_end_time[job_idx], machine_end_time[machine])
                end_time = start_time + duration

                job_end_time[job_idx] = end_time
                machine_end_time[machine] = end_time
                job_op_index[job_idx] += 1
                break

    return max(job_end_time)


def build_schedule(
    jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int
) -> list[ScheduleItem]:
    """Reconstruye el cronograma detallado únicamente para la secuencia ganadora final."""
    job_end_time = [0] * jsp.total_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * jsp.total_jobs

    schedule: list[ScheduleItem] = []

    for job_idx in sequence:
        job = jsp.jobs[job_idx]
        op_idx = job_op_index[job_idx]

        operation = job.operations[op_idx]
        m_id = operation.machine
        duration = operation.duration

        start_time = max(job_end_time[job_idx], machine_end_time[m_id])
        end_time = start_time + duration

        job_end_time[job_idx] = end_time
        machine_end_time[m_id] = end_time
        job_op_index[job_idx] += 1

        schedule.append(
            {
                "job": job.id,
                "op": op_idx + 1,
                "machine": m_id,
                "start": start_time,
                "end": end_time,
                "duration": duration,
            }
        )

    return schedule


def bruteforce(jsp: Jsp) -> None:
    if not jsp.jobs:
        print("No jobs to process.")
        return

    n_jobs = jsp.total_jobs
    max_machine_id = max(
        op.machine for job in jsp.jobs for op in job.operations
    ) if jsp.jobs else 0

    # Pre-caching: Aplanamos las operaciones para evitar lookups de atributos en la simulación
    jobs_ops: list[list[tuple[int, int]]] = [
        [(op.machine, op.duration) for op in job.operations]
        for job in jsp.jobs
    ]

    total_ops = sum(len(ops) for ops in jobs_ops)

    # Reemplazamos Counter por una lista de enteros para acceso O(1) rápido sin overhead
    remaining_ops = [len(ops) for ops in jobs_ops]

    # Cota superior inicial para podar activamente desde el primer árbol de búsqueda
    best_makespan = calculate_initial_upper_bound(jobs_ops, n_jobs, max_machine_id)
    best_sequence: tuple[int, ...] | None = None

    current_seq: list[int] = []
    job_op_index = [0] * n_jobs
    job_end_time = [0] * n_jobs
    machine_end_time = [0] * (max_machine_id + 1)

    def backtrack(depth: int) -> None:
        nonlocal best_makespan, best_sequence

        if depth == total_ops:
            current_makespan = max(job_end_time)
            if current_makespan < best_makespan:
                best_makespan = current_makespan
                best_sequence = tuple(current_seq)
            return

        for job_idx in range(n_jobs):
            if remaining_ops[job_idx] > 0:
                op_idx = job_op_index[job_idx]
                machine, duration = jobs_ops[job_idx][op_idx]

                prev_job_end = job_end_time[job_idx]
                prev_machine_end = machine_end_time[machine]

                start_time = max(prev_job_end, prev_machine_end)
                end_time = start_time + duration

                # PODA: Si ya igualamos o superamos la cota actual, descartamos la rama
                if end_time >= best_makespan:
                    continue

                # Aplicar paso
                job_end_time[job_idx] = end_time
                machine_end_time[machine] = end_time
                job_op_index[job_idx] += 1
                remaining_ops[job_idx] -= 1
                current_seq.append(job_idx)

                backtrack(depth + 1)

                # Revertir paso (Backtrack)
                current_seq.pop()
                remaining_ops[job_idx] += 1
                job_op_index[job_idx] -= 1
                machine_end_time[machine] = prev_machine_end
                job_end_time[job_idx] = prev_job_end

    print("Solving JSP using Optimized Brute Force (Branch and Bound)...")
    backtrack(0)

    # Si la poda inicial era tan buena que ninguna secuencia la superó explícitamente,
    # ejecutamos la simulación determinista para obtener la secuencia base equivalente
    if best_sequence is None:
        # Reconstruimos la secuencia base que produjo la cota inicial
        dummy_seq: list[int] = []
        op_idx_dummy = [0] * n_jobs
        for _ in range(total_ops):
            for j_idx in range(n_jobs):
                if op_idx_dummy[j_idx] < len(jobs_ops[j_idx]):
                    dummy_seq.append(j_idx)
                    op_idx_dummy[j_idx] += 1
                    break
        best_sequence = tuple(dummy_seq)

    best_schedule = build_schedule(jsp, best_sequence, max_machine_id)

    print(f"\nOptimal Makespan: {best_makespan}\n")
    print(f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}")
    print("-" * 50)

    sorted_schedule = sorted(best_schedule, key=lambda x: (x["job"], x["op"]))
    for item in sorted_schedule:
        print(
            f"{item['job']:<6}{item['op']:<6}{item['machine']:<10}"
            f"{item['start']:<8}{item['end']:<8}{item['duration']:<10}"
        )
