def get_group_count(group_str: str) -> int:
    questions = []
    count = 0
    for q in group_str:
        if q not in questions and "a" <= q <= "z":
            count += 1
            questions.append(q)
    return count


input = None
with open("input.txt", "r") as f:
    input = f.read().split("\n\n")

total = 0
for group_str in input:
    total += get_group_count(group_str)
print(total)
