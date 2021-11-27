import re

rules = {}
messages = []


def parse_rule(r: int) -> str:
    if r not in rules:
        return None

    rule = rules[r]

    if r == 8:
        return f"({parse_rule(42)})+"
    elif r == 11:
        r42 = parse_rule(42)
        r31 = parse_rule(31)
        bruh = []
        for i in range(1, 10):
            bruh2 = "("
            for j in range(i):
                bruh2 += r42
            for j in range(i):
                bruh2 += r31
            bruh2 += ")"
            bruh.append(bruh2)

        return "(" + "|".join(bruh) + ")"
    
    if type(rule) is str:
        return rule
    elif type(rule[0]) is int:
        res = ""
        for n in rule:
            res += parse_rule(n)
        return res
    else:
        rs = []
        for ns in rule:
            res = ""
            for n in ns:
                res += parse_rule(n)
            rs.append(res)
        return "(" + "|".join(rs) + ")"


with open("input.txt") as f:
    rules_txt, messages_txt = f.read().split("\n\n")
messages = messages_txt.splitlines()

for rule_txt in rules_txt.splitlines():
    number, args = rule_txt.split(":")
    number = int(number)

    if '"' in args:
        rules[number] = args.replace('"', '').replace(" ", "")
    else:
        either = args.split("|")
        if len(either) == 1:
            matches = either[0].removesuffix(" ").removeprefix(" ").split(" ")
            rules[number] = [int(rule) for rule in matches]
        else:
            matches = []
            for r in either:
                r = r.removesuffix(" ").removeprefix(" ").split(" ")
                matches.append([int(rule) for rule in r])
            rules[number] = matches


regex = "^" + parse_rule(0) + "$"
print(regex)
total = 0
for message in messages:
    match = re.match(regex, message)
    if match:
        total += 1
print(total)
