from typing import Tuple


def text_to_coords(text: str) -> Tuple[int, int, int]:
    x = 0
    y = 0
    z = 0

    i = 0
    direction = ""
    length = len(text)

    while i < length:
        direction += text[i]
        i += 1

        if direction in ("n", "s"):
            continue

        if direction == "nw":
            z -= 1
            y += 1
        elif direction == "ne":
            z -= 1
            x += 1
        elif direction == "sw":
            z += 1
            x -= 1
        elif direction == "se":
            z += 1
            y -= 1
        elif direction == "e":
            x += 1
            y -= 1
        elif direction == "w":
            x -= 1
            y += 1
        
        direction = ""
    
    return (x, y, z)


with open("input.txt") as f:
    lines = f.readlines()

map = {}
for line in lines:
    coords = text_to_coords(line)
    if coords not in map:
        map[coords] = False
    map[coords] = not map[coords]

count = 0
for coord in map:
    if map[coord]:
        count += 1
print(count)
