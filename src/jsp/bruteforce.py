from dataclasses import dataclass
from itertools import permutations
from jsp.domain.jsp import Jsp


@dataclass(slots=True, frozen=True)
class ScheduleItem:
    job: int
    op: int
    machine: int
    start: int
    end: int
    duration: int

    def __str__(self) -> str:
        return (
            f"{self.job:<6}{self.op:<6}{self.machine:<10}"
            f"{self.start:<8}{self.end:<8}{self.duration:<10}"
        )


@dataclass(slots=True, frozen=True)
class Schedule:
    makespan: int
    items: tuple[ScheduleItem, ...]

    def __str__(self) -> str:
        header = (
            f"Optimal Makespan: {self.makespan}\n\n"
            f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}\n"
            + "-" * 50
        )
        sorted_items = sorted(self.items, key=lambda x: (x.job, x.op))
        rows = "\n".join(str(item) for item in sorted_items)
        return f"{header}\n{rows}"


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


def build_schedule(jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int) -> Schedule:
    """Reconstruye el objeto Schedule para la secuencia óptima."""
    job_end_time = [0] * jsp.total_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * jsp.total_jobs

    items: list[ScheduleItem] = []

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

        items.append(
            ScheduleItem(
                job=job.id,
                op=op_idx + 1,
                machine=m_id,
                start=start_time,
                end=end_time,
                duration=duration,
            )
        )

    return Schedule(makespan=max(job_end_time), items=tuple(items))


def bruteforce(jsp: Jsp) -> Schedule | None:
    if not jsp.jobs:
        return None

    max_machine_id = max(op.machine for job in jsp.jobs for op in job.operations)

    base_sequence: list[int] = []
    for idx, job in enumerate(jsp.jobs):
        base_sequence.extend([idx] * len(job.operations))

    best_makespan = float("inf")
    best_sequence: tuple[int, ...] | None = None

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
        return None

    return build_schedule(jsp, best_sequence, max_machine_id)
