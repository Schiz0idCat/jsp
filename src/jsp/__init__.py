import argparse
import sys
from pathlib import Path

from jsp.bruteforce import bruteforce
from jsp.parser.jsp_parser import JspParser

def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Read and print a Job Shop Scheduling matrix instance."
    )
    parser.add_argument(
        "filepath",
        type=Path,
        help="Path to the .txt instance file"
    )
    return parser.parse_args()

def main() -> None:
    args = parse_args()

    try:
        parser = JspParser()
        instance = parser.parse_file(args.filepath)

        print(instance)

        print()

        solution = bruteforce(instance)

        if solution is None:
            print("No solution")
            return

        print(solution)

    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except ValueError as e:
        print(f"Invalid file format: {e}", file=sys.stderr)
        sys.exit(1)
