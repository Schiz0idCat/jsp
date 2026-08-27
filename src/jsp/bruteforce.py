from collections import Counter
from collections.abc import Iterator
from typing import TypedDict

from jsp.domain.jsp import Jsp


class ScheduleItem(TypedDict):
    job: int
    op: int
    machine: int
    start: int
    end: int
    duration: int


def unique_permutations(elements: list[int]) -> Iterator[tuple[int, ...]]:
    """Genera solo permutaciones únicas de una lista con duplicados sin saturar RAM."""
    counts = Counter(elements)

    def _backtrack(current: list[int]) -> Iterator[tuple[int, ...]]:
        if len(current) == len(elements):
            yield tuple(current)
            return

        for elem in counts:
            if counts[elem] > 0:
                counts[elem] -= 1
                current.append(elem)
                yield from _backtrack(current)
                current.pop()
                counts[elem] += 1

    yield from _backtrack([])


def evaluate_makespan(jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int) -> int:
    """Calcula rápidamente el makespan sin construir la estructura de schedule."""
    job_end_time = [0] * jsp.total_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * jsp.total_jobs

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

    return max(job_end_time)


def build_schedule(jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int) -> list[ScheduleItem]:
    """Reconstruye el cronograma detallado solo para la secuencia óptima."""
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
    # Determinamos la ID de máquina más alta para dimensionar listas fijas
    max_machine_id = max(
        op.machine for job in jsp.jobs for op in job.operations
    ) if jsp.jobs else 0

    base_sequence: list[int] = []
    for job_idx, job in enumerate(jsp.jobs):
        base_sequence.extend([job_idx] * len(job.operations))

    print("Solving JSP using Brute Force...")

    best_makespan = float("inf")
    best_sequence: tuple[int, ...] | None = None

    for seq in unique_permutations(base_sequence):
        makespan = evaluate_makespan(jsp, seq, max_machine_id)
        if makespan < best_makespan:
            best_makespan = makespan
            best_sequence = seq

    if best_sequence is None:
        print("No valid schedule found.")
        return

    best_schedule = build_schedule(jsp, best_sequence, max_machine_id)

    # Presentación de resultados
    print(f"\nOptimal Makespan: {best_makespan}\n")
    print(f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}")
    print("-" * 50)

    sorted_schedule = sorted(best_schedule, key=lambda x: (x["job"], x["op"]))
    for item in sorted_schedule:
        print(
            f"{item['job']:<6}{item['op']:<6}{item['machine']:<10}"
            f"{item['start']:<8}{item['end']:<8}{item['duration']:<10}"
        )
