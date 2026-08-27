from dataclasses import dataclass
from jsp.domain.jsp import Jsp


@dataclass(slots=True, frozen=True)
class ScheduleItem:
    job: int
    op: int
    machine: int
    start: int
    end: int
    duration: int


def simulate_sequence(
    sequence: tuple[int, ...], 
    jsp: Jsp, 
    max_machine_id: int
) -> tuple[int, list[ScheduleItem]]:
    """Simulates a given sequence and calculates its detailed schedule and makespan."""
    job_end_time = [0] * jsp.total_jobs
    machine_end_time = [0] * (max_machine_id + 1)
    job_op_index = [0] * jsp.total_jobs

    schedule: list[ScheduleItem] = []

    for job_idx in sequence:
        job = jsp.jobs[job_idx]
        op_idx = job_op_index[job_idx]
        operation = job.operations[op_idx]

        m_id, duration = operation.machine, operation.duration
        start_time = max(job_end_time[job_idx], machine_end_time[m_id])
        end_time = start_time + duration

        job_end_time[job_idx] = end_time
        machine_end_time[m_id] = end_time
        job_op_index[job_idx] += 1

        schedule.append(
            ScheduleItem(
                job=job.id,
                op=op_idx + 1,
                machine=m_id,
                start=start_time,
                end=end_time,
                duration=duration,
            )
        )

    return max(job_end_time), schedule


def solve_jsp_bruteforce(jsp: Jsp) -> tuple[int, list[ScheduleItem]]:
    """Solves the JSP instance using pure brute force (explores 100% of search space)."""
    if not jsp.jobs:
        return 0, []

    max_machine_id = max(op.machine for job in jsp.jobs for op in job.operations)
    total_ops = sum(len(job.operations) for job in jsp.jobs)

    best_makespan: float | int = float("inf")
    best_sequence: tuple[int, ...] = ()

    remaining_ops = [len(job.operations) for job in jsp.jobs]
    current_seq: list[int] = []

    def generate_all_sequences() -> None:
        nonlocal best_makespan, best_sequence

        if len(current_seq) == total_ops:
            makespan, _ = simulate_sequence(tuple(current_seq), jsp, max_machine_id)
            if makespan < best_makespan:
                best_makespan = makespan
                best_sequence = tuple(current_seq)
            return

        for job_idx in range(jsp.total_jobs):
            if remaining_ops[job_idx] > 0:
                remaining_ops[job_idx] -= 1
                current_seq.append(job_idx)

                generate_all_sequences()

                current_seq.pop()
                remaining_ops[job_idx] += 1

    generate_all_sequences()

    if not best_sequence:
        return 0, []

    return simulate_sequence(best_sequence, jsp, max_machine_id)


def print_schedule(makespan: int, schedule: list[ScheduleItem]) -> None:
    print(f"\nOptimal Makespan: {makespan}\n")
    print(f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}")
    print("-" * 50)

    sorted_schedule = sorted(schedule, key=lambda x: (x.job, x.op))
    for item in sorted_schedule:
        print(
            f"{item.job:<6}{item.op:<6}{item.machine:<10}"
            f"{item.start:<8}{item.end:<8}{item.duration:<10}"
        )


def bruteforce(jsp: Jsp) -> None:
    """Legacy CLI entrypoint."""
    print("Solving JSP using Pure Brute Force...")
    makespan, schedule = solve_jsp_bruteforce(jsp)
    if schedule:
        print_schedule(makespan, schedule)
    else:
        print("No jobs to process.")
