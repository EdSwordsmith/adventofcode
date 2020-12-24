cups = [int(num) for num in "523764819"]
lowest = min(cups)
highest = max(cups)
current_i = 0
current = 0


def cups_str() -> str:
    string = " ".join([str(num) for num in cups])
    ci = string.find(str(current))
    return string[:ci] + "(" + string[ci] + ")" + string[ci + 1:]


for i in range(100):
    print(f"-- move {i + 1} --")
    current = cups[current_i]
    print("cups:", cups_str())

    picked_up = []
    for _ in range(3):
        n = cups.index(current)
        picked_up.append(cups.pop((n + 1) % len(cups)))
    print("pick up:", ", ".join([str(num) for num in picked_up]))
    
    dest = current - 1
    while dest not in cups:
        dest -= 1
        if dest < lowest:
            dest = highest
    print("destination:", dest)
    print()
    
    dest_pos = cups.index(dest)
    for _ in range(3):
        cups.insert(dest_pos + 1, picked_up.pop())
    
    current_i = (cups.index(current) + 1) % len(cups)

print("-- final --")
current = cups[current_i]
print("cups:", cups_str())

res = ""
one_i = cups.index(1)
for cup in cups[one_i + 1:]:
    res += str(cup)
for cup in cups[:one_i]:
    res += str(cup)
print(res)
