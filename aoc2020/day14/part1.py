with open("input.txt", "r") as f:
    instructions = f.readlines()

mask = ""
andmask = 0
ormask = 0
memory = {}


def set_value(loc: int, value: int):
    value = (value | ormask) & andmask
    memory[loc] = value


for instruction in instructions:
    if "mask" in instruction:
        mask = instruction.replace("mask = ", "")
        ormask = int(mask.replace("X", "0"), base=2)
        andmask = int(mask.replace("X", "1"), base=2)
    else:
        a1, a2 = instruction.replace("mem[", "").split("] = ")
        loc, value = int(a1), int(a2)
        set_value(loc, value)

sum = 0
for loc in memory:
    sum += memory[loc]
print(sum)
