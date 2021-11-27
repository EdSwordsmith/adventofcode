from sympy.ntheory.modular import crt, solve_congruence

with open("input.txt", "r") as f:
    timestamp = int(f.readline())
    buses_s = f.readline().split(",")

buses = []
ts = []
for i, bus in enumerate(buses_s):
    if bus != "x":
        buses.append(int(bus))
        ts.append(-i)

print(crt(buses, ts))
