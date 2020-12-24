from typing import List, Tuple

directions_t = ("nw", "ne", "sw", "se", "w", "e")
directions = ((1, -1, 0), (0, -1, 1), (-1, 0, 1), (-1, 1, 0), (0, 1, -1), (1, 0, -1))


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

        if direction not in directions_t:
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


def get_adjacent(coord: Tuple[int, int, int]) -> List[Tuple[int, int, int]]:
    adjacent = []
    x, y, z = coord
    for d in directions:
        dx, dy, dz = d
        adjacent.append((x + dx, y + dy, z + dz))
    return adjacent


with open("input.txt") as f:
    lines = f.readlines()

map = set()
for line in lines:
    coords = text_to_coords(line)
    if coords in map:
        map.remove(coords)
    else:
        map.add(coords)


def count_adjacent_black(coord: Tuple[int, int, int]) -> int:
    count = 0
    for adjacent in get_adjacent(coord):
        if adjacent in map:
            count += 1
    return count


def next_cycle():
    global map
    newMap = set()

    positions = set()
    for coord in map:
        positions.add(coord)
        for position in get_adjacent(coord):
            positions.add(position)
    
    for pos in positions:
        count = count_adjacent_black(pos)
        if (pos in map and count == 1) or count == 2:
            newMap.add(pos)

    map = newMap

for _ in range(100):
    next_cycle()

print(len(map))
