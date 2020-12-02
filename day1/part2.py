nums = None
with open("input.txt", "r") as f:
    nums = [int(line) for line in f.readlines()]

for x in nums:
    for y in nums:
        for z in nums:
            if x + y + z == 2020:
                print(x * y * z)
