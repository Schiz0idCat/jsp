class Instance:
    def __init__(self, components: int, operations: int, matrix: list[list[int]]):
        self.components = components
        self.operations = operations
        self.matrix = matrix

    @classmethod
    def from_file(cls, filepath: str) -> "Instance":
        with open(filepath, 'r') as f:
            first_line = f.readline().strip()
            components, operations = map(int, first_line.split())

            matrix = []
            for line in f:
                line_str = line.strip()
                if line_str:
                    row = list(map(int, line_str.split()))
                    matrix.append(row)

        return cls(components, operations, matrix)

    def display(self) -> None:
            print(f"Components: {self.components} | Operations: {self.operations}")
            print("-" * 40)
            for row in self.matrix:
                formatted_row = " ".join(f"{num:3d}" for num in row)
                print(formatted_row)
