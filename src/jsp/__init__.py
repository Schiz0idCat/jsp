import argparse
import sys
from pathlib import Path

from jsp.file import Instance

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
        matrix = Instance.from_file(args.filepath)
        print(f"Successfully loaded instance from: {args.filepath}\n")
        matrix.display()

    except FileNotFoundError as e:
        print(f"Error: {e}", file=sys.stderr)
        sys.exit(1)
    except ValueError as e:
        print(f"Invalid file format: {e}", file=sys.stderr)
        sys.exit(1)
