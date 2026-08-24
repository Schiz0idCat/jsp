from dataclasses import dataclass


@dataclass(frozen=True)
class Operation:
    duration: int
    machine: int

    def __str__(self) -> str:
        return f"M{self.machine} ({self.duration}h)"
