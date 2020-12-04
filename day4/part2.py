import re


def valid_year(p, f, ymin, ymax):
    return f in p and len(p[f]) == 4 and ymin <= int(p[f]) <= ymax


def valid_height(p):
    if "hgt" not in p:
        return False
    tmp = p["hgt"]
    value = int(tmp[:len(tmp)-2])
    if "cm" in tmp:
        return 150 <= value <= 193
    return 59 <= value <= 76


def valid_ecl(p):
    if "ecl" not in p:
        return False
    ecl = p["ecl"]
    valid_colors = ("amb", "blu", "brn", "gry", "grn", "hzl", "oth")
    return ecl in valid_colors


def valid_pid(p):
    if "pid" not in p:
        return False
    try:
        pid = int(p["pid"])
    except:
        return False
    return len(p["pid"]) == 9


def valid_hcl(p):
    if "hcl" not in p:
        return False

    match = re.search(r'^#(?:[0-9a-fA-F]{3}){1,2}$', p["hcl"])
    if match:
        return True
    else:
        return False


def is_valid(p):
    return valid_year(p, "byr", 1920, 2002) \
        and valid_year(p, "iyr", 2010, 2020) \
        and valid_year(p, "eyr", 2020, 2030) \
        and valid_height(p) \
        and valid_hcl(p) \
        and valid_ecl(p) \
        and valid_pid(p)


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
            if len(tmp) != 2:
                continue
            passaport[tmp[0]] = tmp[1]
            
        if is_valid(passaport):
            total += 1

print(total)
        
        
