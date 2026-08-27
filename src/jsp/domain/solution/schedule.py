from dataclasses import dataclass
from jsp.domain.solution.schedule_item import ScheduleItem


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
