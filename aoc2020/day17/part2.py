import copy
CYCLES = 6
def getZ(n): return n+6
def getW(n): return n+6
with open("input.txt") as file:
    lines = file.readlines()
    for i in range(len(lines)):
        if "\n" in lines[i]:
            lines[i] = lines[i][:-1]
x_max_init = len(lines[0])
y_max_init = len(lines)
x_max = x_max_init+2*CYCLES
y_max = y_max_init+2*CYCLES
z_max = 2*CYCLES+1
w_max = z_max
actual_cube = [[[["." for i in range(x_max_init+2*CYCLES)] for i in range(y_max_init+2*CYCLES)] for i in range(z_max)] for j in range(w_max)]

for x in range(CYCLES, CYCLES + x_max_init):
    for y in range(CYCLES, CYCLES + y_max_init):
        actual_cube[getW(0)][getZ(0)][x][y] = lines[x-CYCLES][y-CYCLES]

old_cube = copy.deepcopy(actual_cube)

def checkCube(w,z,y,x):
    neighb = [
                [
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ]
                ],
                [
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ]
                ],
                [
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ],
                    [
                        [0,0,0],
                        [0,0,0],
                        [0,0,0],
                    ]
                ]
             ]
    for n in range(3):
        if n==0 and w-1<0 or n==2 and w+1 >= w_max:
            continue
        for k in range(3):
            if k==0 and z-1<0 or k==2 and z+1 >= z_max:
                continue
            for l in range(3):
                if l==0 and y-1<0 or l==2 and y+1 >= y_max:
                    continue
                for m in range(3):
                    if m==0 and x-1<0 or m==2 and x+1 >= x_max:
                        continue
                    if not (n == 1 and k == 1 and l == 1 and m == 1):
                        neighb[n][k][l][m] = old_cube[w-1+n][z-1+k][y-1+l][x-1+m]
    
    active_count = 0
    for w_thing in neighb:
        for z_line in w_thing:
            for yz_line in z_line:
                for xyz in yz_line:
                    if xyz == "#":
                        active_count += 1

    if old_cube[w][z][y][x] == "." and active_count == 3:
        actual_cube[w][z][y][x] = "#"
    elif old_cube[w][z][y][x] == "#" and not 2 <= active_count <= 3:
        actual_cube[w][z][y][x] = "."
    else:
        actual_cube[w][z][y][x] = old_cube[w][z][y][x]

actual_cycle = 1

while actual_cycle <= CYCLES:
    for w_thing in range(w_max):
        for z_line in range(z_max):
            for y_line in range(y_max):
                for x in range(x_max):
                    checkCube(w_thing, z_line,y_line,x)
    old_cube = copy.deepcopy(actual_cube)
    actual_cycle += 1


active_count = 0
for w_thing in actual_cube:
    for z_slice in w_thing:
        for y_line in z_slice:
            for x_pos in y_line:
                if x_pos == "#":
                    active_count += 1
print(active_count)