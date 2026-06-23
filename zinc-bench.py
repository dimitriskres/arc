import re
import typing
import subprocess
import sys
import csv

MZN = './puzzles/latin/zinc/model-eq.mzn'
DZN = './puzzles/latin/zinc/data.dzn'

COUNT = 100

SOLVERS = [
    # 'Chuffed', 
    'Gecode', 
    # 'CP-SAT'
]

TIMEOUT = 60 * 10

PATTERN = re.compile(r"%%%mzn-stat:\s*solveTime=([0-9.]+)")

def execute(solver: str, mzn: str, dzn: str) -> typing.Iterator[float]:
    command = ['MINIZINC', '--solver', solver, mzn, dzn, '--statistics']
    for i in range(1, COUNT + 1):
        result = subprocess.run(command, text = True, capture_output = True, timeout = TIMEOUT)
        if result.returncode != 0:
            print(result.stdout)
            print(result.stderr)
            raise SystemExit(result.returncode)
        output = result.stdout + result.stderr
        match = PATTERN.search(output)
        if match is None:
            raise RuntimeError(f"solveTime not found in run {i}")
        duration = float(match.group(1))
        print(f"{i:02}: {duration:.4f}s")
        yield duration

def main():
    with open('./zinc-bench.csv', mode = 'a', newline = '') as file:
        writer = csv.writer(file)
        for solver in SOLVERS:
            try:
                for duration in execute(solver, MZN, DZN):
                    writer.writerow([solver, duration])
            except subprocess.TimeoutExpired:
                print(f"{solver} timed out")

if __name__ == '__main__':
    main()
