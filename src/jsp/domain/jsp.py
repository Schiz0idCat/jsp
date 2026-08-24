from dataclasses import dataclass
from jsp.domain.job import Job


@dataclass
class Jsp:
    total_jobs: int
    total_machines: int
    jobs: list[Job]

    def __str__(self) -> str:
        # Encabezado dinámico según la cantidad máxima de operaciones por trabajo
        max_ops = max(len(job.operations) for job in self.jobs) if self.jobs else 0
        ops_header = " | ".join(f"{f'Op {i+1}':^10}" for i in range(max_ops))

        title = f"JSP Instance [{self.total_jobs} Jobs x {self.total_machines} Machines]"
        header = f"{'':<6} | {ops_header}"
        divider = "-" * len(header)

        body = "\n".join(str(job) for job in self.jobs)

        return f"{title}\n{divider}\n{header}\n{divider}\n{body}\n{divider}"
