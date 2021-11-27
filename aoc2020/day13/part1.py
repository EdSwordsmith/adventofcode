with open("input.txt", "r") as f:
    timestamp = int(f.readline())
    buses = [int(bus) for bus in f.readline().replace("x,", "").split(",")]

wait = 0
while True:
    stop = False
    for bus in buses:
        if (timestamp + wait) % bus == 0:
            print(wait * bus)
            stop = True
            break
    if stop:
        break
    wait += 1
