from typing import List, Tuple


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


instructions = parse_input("input.txt")
visited = []
i = 0
accumulator = 0
while i < len(instructions):
    if i in visited:
        print(accumulator)
        break
    else:
        visited.append(i)

    ins, arg = instructions[i]

    #print(i, "-", ins, arg, "-", accumulator, "-", second)

    if ins == "jmp":
        i += arg
    elif ins == "acc":
        accumulator += arg
        i += 1
    else:
        i += 1
