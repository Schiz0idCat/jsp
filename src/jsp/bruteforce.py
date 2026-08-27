from dataclasses import dataclass
from itertools import permutations
from jsp.domain.jsp import Jsp


@dataclass(slots=True, frozen=True)
class Solution:
    job: int
    op: int
    machine: int
    start: int
    end: int
    duration: int


def evaluate_makespan(jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int) -> int:
    """Calcula el makespan para una secuencia dada."""
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


def build_schedule(jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int) -> list[Solution]:
    """Reconstruye el cronograma detallado para la secuencia óptima."""
    job_end_time = [0] * jsp.total_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * jsp.total_jobs

    schedule: list[Solution] = []

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
            Solution(
                job=job.id,
                op=op_idx + 1,
                machine=m_id,
                start=start_time,
                end=end_time,
                duration=duration,
            )
        )

    return schedule


def bruteforce(jsp: Jsp) -> None:
    if not jsp.jobs:
        print("No jobs to process.")
        return

    max_machine_id = max(
        op.machine for job in jsp.jobs for op in job.operations
    )

    # 1. Construimos la lista base con los IDs repetidos según la cantidad de operaciones por trabajo
    base_sequence: list[int] = []
    for job_idx, job in enumerate(jsp.jobs):
        base_sequence.extend([job_idx] * len(job.operations))

    print("Solving JSP using Pure Iterative Brute Force...")

    best_makespan = float("inf")
    best_sequence: tuple[int, ...] | None = None

    # 2. Fuerza bruta pura: iteramos sobre TODAS las permutaciones e ignoramos duplicados con un set
    seen_sequences: set[tuple[int, ...]] = set()

    for seq in permutations(base_sequence):
        if seq in seen_sequences:
            continue
        seen_sequences.add(seq)

        makespan = evaluate_makespan(jsp, seq, max_machine_id)
        if makespan < best_makespan:
            best_makespan = makespan
            best_sequence = seq

    if best_sequence is None:
        print("No valid schedule found.")
        return

    best_schedule = build_schedule(jsp, best_sequence, max_machine_id)

    print(f"\nOptimal Makespan: {best_makespan}\n")
    print(f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}")
    print("-" * 50)

    sorted_schedule = sorted(best_schedule, key=lambda x: (x.job, x.op))
    for item in sorted_schedule:
        print(
            f"{item.job:<6}{item.op:<6}{item.machine:<10}"
            f"{item.start:<8}{item.end:<8}{item.duration:<10}"
        )
