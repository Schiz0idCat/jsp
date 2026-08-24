from abc import ABC, abstractmethod
from pathlib import Path
from typing import Generic, TypeVar


T = TypeVar("T")

class Parser(ABC, Generic[T]):
    """Parser que transforme texto en un tipo T."""

    @abstractmethod
    def parse_content(self, content: str) -> T:
        """Transforma una cadena de texto en un objeto de tipo T."""
        pass

    def parse_file(self, filepath: str | Path) -> T:
        """Lee un archivo de disco y delega el parseo."""
        path = Path(filepath)
        if not path.exists():
            raise FileNotFoundError(f"El archivo '{filepath}' no existe.")

        return self.parse_content(path.read_text(encoding="utf-8"))
