# import itertools
# from typing import TypedDict
# from jsp.file import JspInstance
#
#
# class ScheduleItem(TypedDict):
#     job: int
#     op: int
#     machine: int
#     start: int
#     end: int
#     duration: int
#
#
# def evaluate_schedule(
#     jsp: JspInstance, sequence: tuple[int, ...]
# ) -> tuple[int, list[ScheduleItem]]:
#     n_jobs = jsp.components
#     n_ops = jsp.operations
#
#     # Parse matrix into jobs: list of (machine, duration) tuples per job
#     jobs: list[list[tuple[int, int]]] = []
#     for row in jsp.matrix:
#         if len(row) == 2 * n_ops:
#             # Standard format: pairs of (machine, duration)
#             job_ops = [(row[2 * k], row[2 * k + 1]) for k in range(n_ops)]
#         else:
#             # Fallback: single value duration, machine index equals operation index
#             job_ops = [(k, row[k]) for k in range(n_ops)]
#         jobs.append(job_ops)
#
#     job_end_time = [0] * n_jobs
#     machine_end_time: dict[int, int] = {}
#     job_op_index = [0] * n_jobs
#
#     schedule: list[ScheduleItem] = []
#
#     for job_id in sequence:
#         op_idx = job_op_index[job_id]
#         machine, duration = jobs[job_id][op_idx]
#
#         start_time = max(job_end_time[job_id], machine_end_time.get(machine, 0))
#         end_time = start_time + duration
#
#         job_end_time[job_id] = end_time
#         machine_end_time[machine] = end_time
#         job_op_index[job_id] += 1
#
#         schedule.append(
#             {
#                 "job": job_id,
#                 "op": op_idx,
#                 "machine": machine,
#                 "start": start_time,
#                 "end": end_time,
#                 "duration": duration,
#             }
#         )
#
#     makespan = max(job_end_time)
#     return makespan, schedule
#
#
# def bruteforce(jsp: Jsp) -> None:
#     n_jobs = jsp.components
#     n_ops = jsp.operations
#
#     # Build initial job list: each job ID appears n_ops times
#     base_sequence: list[int] = []
#     for job_id in range(n_jobs):
#         base_sequence.extend([job_id] * n_ops)
#
#     # Generate unique permutations to avoid redundant evaluations
#     unique_sequences = set(itertools.permutations(base_sequence))
#     total_combinations = len(unique_sequences)
#
#     print(
#         f"Solving JSP with Brute Force ({total_combinations} unique combinations)..."
#     )
#
#     best_makespan = float("inf")
#     best_schedule: list[ScheduleItem] | None = None
#
#     for seq in unique_sequences:
#         makespan, schedule = evaluate_schedule(jsp, seq)
#         if makespan < best_makespan:
#             best_makespan = makespan
#             best_schedule = schedule
#
#     if best_schedule is None:
#         print("No schedule found.")
#         return
#
#     # Display results
#     print(f"\nOptimal Makespan: {best_makespan}\n")
#     print(
#         f"{'Job':<6}{'Op':<6}{'Machine':<10}{'Start':<8}{'End':<8}{'Duration':<10}"
#     )
#     print("-" * 50)
#
#     sorted_schedule = sorted(best_schedule, key=lambda x: (x["job"], x["op"]))
#     for item in sorted_schedule:
#         print(
#             f"{item['job']:<6}{item['op']:<6}{item['machine']:<10}"
#             f"{item['start']:<8}{item['end']:<8}{item['duration']:<10}"
#         )
