from typing import List, Tuple
import threading


def parse_input(filename: str) -> List[Tuple[str, int]]:
    lines: List[str] = []
    with open(filename, "r") as f:
        lines = f.readlines()

    instructions: List[Tuple[str, int]] = []
    for line in lines:
        ins, arg = line.split()
        arg = int(arg)
        instructions.append((ins, arg))

    return instructions


def run_instructions(instructions, changed):
    i = 0
    accumulator = 0

    while i < len(instructions):
        ins, arg = instructions[i]

        if ins == "jmp":
            i += arg
        elif ins == "acc":
            accumulator += arg
            i += 1
        else:
            i += 1

    print("Terminated", changed, "Accumulator", accumulator)


instructions = parse_input("input.txt")
for i in range(len(instructions)):
    if instructions[i][0] == "jmp":
        copy = instructions.copy()
        copy[i] = ("nop", instructions[i][1])
        t = threading.Thread(target=run_instructions, args=(copy, i + 1))
        t.start()
    elif instructions[i][0] == "nop":
        copy = instructions.copy()
        copy[i] = ("jmp", instructions[i][1])
        t = threading.Thread(target=run_instructions, args=(copy, i + 1))
        t.start()
