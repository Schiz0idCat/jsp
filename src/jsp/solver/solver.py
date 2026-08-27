from abc import ABC, abstractmethod
from jsp.domain import Jsp
from jsp.domain.solution import Schedule

class JSPSolver(ABC):
    @abstractmethod
    def solve(self, jsp: Jsp) -> Schedule | None:
        """Resuelve una instancia de JSP y retorna un Schedule.

        Raises:
            EmptyJSPError: Si la instancia no contiene trabajos.
            NoSolutionFoundError: Si no se encuentra un algoritmo válido.
        """
        pass
