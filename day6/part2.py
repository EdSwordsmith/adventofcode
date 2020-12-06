def get_group_count(group_str: str) -> int:
    people = group_str.splitlines()
    answers = {}

    for person in people:
        for q in person:
            if q not in answers and "a" <= q <= "z":
                answers[q] = 1
            elif "a" <= q <= "z":
                answers[q] += 1

    count = 0
    for answer in answers:
        if answers[answer] == len(people):
            count += 1

    return count


input = None
with open("input.txt", "r") as f:
    input = f.read().split("\n\n")

total = 0
for group_str in input:
    total += get_group_count(group_str)
print(total)
