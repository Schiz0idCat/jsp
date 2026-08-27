from dataclasses import dataclass


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
