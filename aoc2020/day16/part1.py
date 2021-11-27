with open("bruh.txt", "r") as f:
    rules_input, ticket, nearby_tickets = f.read().split("\n\n")

rules = {}
for rule in rules_input.splitlines():
    name, ranges = rule.split(":")
    ranges = ranges.split("or")
    ranges = tuple(int(num) for r in ranges for num in r.split("-"))
    rules[name] = ranges

total = 0
count = 0
for ticket in nearby_tickets.splitlines()[1:]:
    fields = [int(num) for num in ticket.split(",")]
    v = True
    for field in fields:
        valid = False

        for r in rules:
            rule = rules[r]
            if rule[0] <= field <= rule[1] or rule[2] <= field <= rule[3]:
                valid = True
                break

        if not valid:
            v = False
            total += field
    if not v:
        count += 1

print(total, count)
