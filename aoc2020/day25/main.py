card_key = 13233401
door_key = 6552760

key = 1
loops = 0
while key != card_key:
    key = key * 7
    key = key % 20201227
    loops += 1

key = 1
for _ in range(loops):
    key = key * door_key
    key = key % 20201227
print(key)
