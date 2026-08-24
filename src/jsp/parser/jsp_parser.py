from jsp.domain.job import Job
from jsp.domain.jsp import Jsp
from jsp.domain.operation import Operation
from jsp.parser.parser import Parser


class JspParser(Parser[Jsp]):
    """Parser concreto que transforma texto plano en un JspInstance."""

    def parse_content(self, content: str) -> Jsp:
        lines = (line.strip() for line in content.splitlines() if line.strip())

        header = next(lines, None)
        if not header:
            raise ValueError("El contenido a parsear está vacío.")

        total_jobs, total_machines = map(int, header.split())

        jobs: list[Job] = []
        for job_id, line in enumerate(lines, start=1):
            values = list(map(int, line.split()))

            iterator = iter(values)
            operations = [
                Operation(duration=duration, machine=machine)
                for duration, machine in zip(iterator, iterator)
            ]

            jobs.append(Job(id=job_id, operations=operations))

        return Jsp(
            total_jobs=total_jobs,
            total_machines=total_machines,
            jobs=jobs
        )
