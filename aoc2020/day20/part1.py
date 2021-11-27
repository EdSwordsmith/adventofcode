from typing import List


tiles = {}
borders = {}
bruhhh = 0

with open("input.txt", "r") as f:
    tmp = f.read()
    bruhhh = tmp.count("#")
    tmp = tmp.split("\n\n")

for tile in tmp:
    header, data = tile.split(":\n")
    num = int(header.replace("Tile ", ""))
    data = data.splitlines()
    d = []
    d.append(data[0])
    d.append(data[-1])
    tmp = ""
    tmp2 = ""
    for s in data:
        tmp += s[0]
        tmp2 += s[-1]
    d.append(tmp)
    d.append(tmp2)
    tiles[num] = d
    borders[num] = []


def can_border(t1: List[str], t2: List[str]) -> bool:
    global bruhhh
    def aux(s1: str, s2: str):
        return s1 == s2 or "".join(reversed(s1)) == s2
    for s1 in t1:
        for s2 in t2:
            if aux(s1, s2):
                bruhhh -= s1.count("#")
                return True
    return False


for tile in tiles:
    for t in tiles:
        if tile == t:
            continue
        
        if can_border(tiles[tile], tiles[t]):
            borders[tile].append(t)

value = 1
for tile in borders:
    if len(borders[tile]) == 2:
        value *= tile
print(value)
print()
print(bruhhh)
