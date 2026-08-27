from itertools import permutations
from jsp.solver import JSPSolver
from jsp.domain import Jsp
from jsp.domain.solution import Schedule
from jsp.domain.solution import ScheduleItem

class BruteForceSolver(JSPSolver):
    def solve(self, jsp: Jsp) -> Schedule | None:
        if not jsp.jobs:
            return None

        max_machine_id = max(
            op.machine for job in jsp.jobs for op in job.operations
        )

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
            makespan = self._evaluate_makespan(jsp, seq, max_machine_id)

            if makespan < best_makespan:
                best_makespan = makespan
                best_sequence = seq

        if best_sequence is None:
            return None

        return self._build_schedule(jsp, best_sequence, max_machine_id)

    def _evaluate_makespan(
        self, jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int
    ) -> int:
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

    def _build_schedule(
        self, jsp: Jsp, sequence: tuple[int, ...], max_machine_id: int
    ) -> Schedule:
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
