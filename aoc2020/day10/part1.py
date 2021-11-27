with open("input.txt", "r") as f:
    numbers = [int(line) for line in f.readlines()]

numbers.append(0)
numbers = sorted(numbers)
numbers.append(numbers[-1] + 3)

diffs = {}
for i, number in enumerate(numbers[1:]):
    diff = abs(number - numbers[i])
    if diff not in diffs:
        diffs[diff] = 1
    else:
        diffs[diff] += 1

print(diffs[1] * diffs[3])