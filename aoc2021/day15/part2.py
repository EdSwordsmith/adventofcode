with open("input.in", "r") as f:
    lines = f.readlines()

bruh = []
for line in lines:
    row = []
    for c in line:
        try:
            num = int(c)
            row.append(num)
        except:
            pass
    bruh.append(row)

for j in range(5):
    for row in bruh:
        for i in range(5):
            for num in row:
                n = num + i + j
                if n >= 10:
                    n = n % 10 + 1
                print(n, end='')
        print()