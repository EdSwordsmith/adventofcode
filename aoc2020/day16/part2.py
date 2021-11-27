with open("bruh.txt", "r") as f:
    rules_input, my_ticket, nearby_tickets = f.read().split("\n\n")

rules = {}
for rule in rules_input.splitlines():
    name, ranges = rule.split(":")
    ranges = ranges.split("or")
    ranges = tuple(int(num) for r in ranges for num in r.split("-"))
    rules[name] = ranges

valid_tickets = []
for ticket in nearby_tickets.splitlines()[1:]:
    fields = [int(num) for num in ticket.split(",")]
    valid_ticket = True
    for field in fields:
        valid = False

        for r in rules:
            rule = rules[r]
            if rule[0] <= field <= rule[1] or rule[2] <= field <= rule[3]:
                valid = True
                break

        if not valid:
            valid_ticket = False

    if valid_ticket:
        valid_tickets.append(fields)

rules_order = []
for _ in rules:
    possible = []
    for rule in rules:
        possible.append(rule)
    rules_order.append(possible)

for ticket in valid_tickets:
    for i, field in enumerate(ticket):
        possible = []

        for r in rules:
            rule = rules[r]
            if rule[0] <= field <= rule[1] or rule[2] <= field <= rule[3]:
                possible.append(r)
        
        rules_order[i] = [value for value in rules_order[i] if value in possible]

while True:
    keep = False

    for possible in rules_order:
        if len(possible) == 1:
            for p2 in rules_order:
                if possible == p2:
                    continue
                if possible[0] in p2:
                    p2.remove(possible[0])
        else:
            keep = True

    if not keep:
        break

order = [p[0] for p in rules_order]
ticket = [int(f) for f in my_ticket.splitlines()[1].split(",")]
res = 1
for i, n in enumerate(order):
    if "departure" in n:
        res = res * ticket[i]
print(res)
