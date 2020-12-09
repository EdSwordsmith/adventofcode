import sys

bruh = int(sys.argv[1])

with open("input.txt") as f:
    numbers = [int(line) for line in f.readlines()]

for i in range(len(numbers)):
    if numbers[i] == bruh:
        continue

    sum = 0
    smallest = numbers[i]
    largest = numbers[i]

    for num in numbers[i:]:
        sum += num

        if num < smallest:
            smallest = num
        elif num > largest:
            largest = num

        if sum == bruh:
            print(smallest + largest)
