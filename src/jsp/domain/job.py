from dataclasses import dataclass
from jsp.domain.operation import Operation


@dataclass
class Job:
    id: int
    operations: list[Operation]

    def __str__(self) -> str:
        ops = " | ".join(f"{str(op):^10}" for op in self.operations)
        return f"Job {self.id:<2} | {ops}"
