from collections import defaultdict

with open("input") as f:
    parsed_input = [[int(num) for num in line.split()] for line in f.readlines()]

left = sorted(nums[0] for nums in parsed_input)
right = sorted(nums[1] for nums in parsed_input)
part1 = sum(abs(l - r) for l, r in zip(left, right))

print("Part 1:", part1)

def occurrence_count(l: list[int]) -> defaultdict[int, int]:
    d = defaultdict(int)
    for n in l:
        d[n] += 1
    return d

left_count = occurrence_count(left)
right_count = occurrence_count(right)
part2 = sum(num * left_count[num] * right_count[num] for num in left_count)

print("Part 2:", part2)
