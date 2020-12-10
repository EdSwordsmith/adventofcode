from typing import List


with open("input.txt", "r") as f:
    numbers = [int(line) for line in f.readlines()]

numbers.append(0)
numbers = sorted(numbers)
numbers.append(numbers[-1] + 3)

store = {}
def get_count(numbers: List[int]) -> int:
    if len(numbers) <= 1:
        return 1

    number = numbers[0]
    if number in store:
        return store[number]

    count = 0
    for i, num in enumerate(numbers[1:]):
        if num - number > 3:
            break
        count += get_count(numbers[i+1:])
    store[number] = count

    return count


print(get_count(numbers))
