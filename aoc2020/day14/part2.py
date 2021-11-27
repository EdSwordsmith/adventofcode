import itertools

with open("example.txt", "r") as f:
    instructions = f.readlines()

mask1 = 0
mask_x = []
memory = {}


def get_addresses(loc: int):
    res = loc | mask1
    res &= (2 ** 36 - 1) - sum(2 ** i for i in mask_x)

    return [
        res + sum(2 ** mask_x[i] for i, value in enumerate(values) if value == 1)
        for values in itertools.product([0, 1], repeat=len(mask_x))
    ]


def get_masks(mask: str):
    if mask.count("X") == 0:
        return [(int(mask.replace("A", "1"), base=2), int(mask.replace("A", "0"), base=2))]
    return get_masks(mask.replace("X", "0", 1)) + get_masks(mask.replace("X", "1", 1))


def set_value(loc: int, value: int):
    locs = get_addresses(loc)
    for l in locs:
        memory[l] = value


for instruction in instructions:
    if "mask" in instruction:
        mask = instruction.replace("mask = ", "")
        mask_x.clear()
        mask1 = 0
        for i, mask in enumerate(reversed(mask)):
            if mask == "1":
                mask1 += 2 ** i
            elif mask == "X":
                mask_x.append(i)
    else:
        a1, a2 = instruction.replace("mem[", "").split("] = ")
        loc, value = int(a1), int(a2)
        set_value(loc, value)

sum = 0
for loc in memory:
    sum += memory[loc]
print(sum)
