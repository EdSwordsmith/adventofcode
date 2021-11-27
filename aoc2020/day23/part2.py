from __future__ import annotations
from dataclasses import dataclass


@dataclass
class Cup:
    label: int
    prev: Cup
    next: Cup


cup_list = [int(num) for num in "523764819"]
lowest = min(cup_list)
highest = max(cup_list)
cups: dict[int, Cup] = {}
prev: Cup = None

for i, cup in enumerate(cup_list):
    cups[cup] = Cup(cup, cup_list[i - 1], None)
    if prev is not None:
        prev.next = cups[cup]
    prev = cups[cup]

current = cup_list[0]

while highest < 1000000:
    highest += 1
    prev.next = cups[highest] = Cup(highest, prev, None)
    prev = prev.next

cups[cup_list[0]].prev = prev
prev.next = cups[cup_list[0]]

for i in range(10000000):
    print(f"-- move {i + 1} --")
    print(f"{10000000 - i - 1} remaining")
    current_cup: Cup = cups[current]

    picked_up: list[Cup] = []
    picked_up.append(current_cup.next)
    picked_up.append(current_cup.next.next)
    picked_up.append(current_cup.next.next.next)
    current_cup.next = current_cup.next.next.next.next
    current_cup.prev = current_cup
    labels = [p.label for p in picked_up]
    
    dest = current - 1
    if dest < lowest:
            dest = highest
    while dest in labels:
        dest -= 1
        if dest < lowest:
            dest = highest

    dest_cup = cups[dest]
    after_dest_cup = dest_cup.next
    dest_cup.next = picked_up[0]
    picked_up[0].prev = dest_cup
    picked_up[2].next = after_dest_cup
    after_dest_cup.prev = picked_up[2]
    
    current = current_cup.next.label

print(cups[1].next.label * cups[1].next.next.label)
