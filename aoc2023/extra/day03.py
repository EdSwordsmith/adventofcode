import itertools
from typing import Generator


def lines() -> Generator[str, None, None]:
    try:
        while True:
            yield input()
    except EOFError:
        pass


def parse_line(line: str, row: int):
    numbers = []
    symbols = []

    number = None
    start = None

    for col, c in enumerate(line):
        if "0" <= c <= "9":
            if number is None:
                start = col
                number = 0

            number = number * 10 + int(c)
            continue
        elif number is not None:
            numbers.append((number, row, start, col - 1))
            start = None
            number = None

        if c != ".":
            symbols.append((c, row, col))

    if number is not None:
        numbers.append((number, row, start, col - 1))

    return numbers, symbols


if __name__ == "__main__":
    numbers = {}
    symbols = {}
    colc = 0
    row = 0

    for row, line in enumerate(lines()):
        new_numbers, new_symbols = parse_line(line, row)
        if colc == 0:
            colc = len(line)

        for n, r, s, e in new_numbers:
            for c in range(s, e + 1):
                numbers[(r, c)] = (n, r, s, e)

        for s, r, c in new_symbols:
            symbols[(r, c)] = s

    rowc = row + 1
    part1 = 0
    for v, row, start, end in set(numbers.values()):
        rows = range(row - 1, row + 2)
        cols = range(start - 1, end + 2)
        for r, c in itertools.product(rows, cols):
            if (r, c) in symbols:
                part1 += v
                break

    part2 = 0
    for pos, s in symbols.items():
        if s != "*":
            continue

        nums = set()
        row, col = pos
        rows = range(row - 1, row + 2)
        cols = range(col - 1, col + 2)
        for r, c in itertools.product(rows, cols):
            if (r, c) in numbers:
                nums.add(numbers[(r, c)])

        if len(nums) == 2:
            res = 1
            for n in nums:
                res = res * n[0]
            part2 += res

    print(part1)
    print(part2)
