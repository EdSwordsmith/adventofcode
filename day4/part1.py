def is_valid(p):
    return "byr" in p and "iyr" in p and "eyr" in p and "hgt" in p and "hcl" in p and "ecl" in p and "pid" in p


passports = None
total = 0

with open("input.txt", "r") as f:
    passports = f.read().split("\n\n")
    for p in passports:
        p2 = p.replace("\n", " ")
        args = p2.split(" ")
        passaport = {}
        for a in args:
            tmp = a.split(":")
            passaport[tmp[0]] = True
        if is_valid(passaport):
            total += 1

print(total)
        
        