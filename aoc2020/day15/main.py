start = [17,1,3,16,19,0]
occurs = {}
turn = 1
num = 0

while turn <= 30000000:
    if turn <= len(start):
        occurs[start[turn - 1]] = (turn, False)
        num = start[turn - 1]
    else:
        if num in occurs and occurs[num][1]:
            num = occurs[num][0] - occurs[num][1]
        else:
            num = 0
        
        if num in occurs:
            occurs[num] = (turn, occurs[num][0])
        else:
            occurs[num] = (turn, False)
    turn += 1

print(num)
