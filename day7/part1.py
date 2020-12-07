with open("input.txt", "r") as f:
    input_lines = f.readlines()

rules = {}
for line in input_lines:
    bag, inner_bags = line.split("contain")
    bag = bag.replace("bags", "").strip()

    rules[bag] = {}
    inner_bags = inner_bags.split(",")
    for inner_bag in inner_bags:
        inner_bag = inner_bag.replace(".", "").replace("bags", "").replace("bag", "").strip()
        if inner_bag.startswith("no"):
            break
        
        number = int(inner_bag[0])
        inner_bag = inner_bag[2:]
        rules[bag][inner_bag] = number
    
bags = ["shiny gold"]
for bag in bags:
    for rule in rules:
        if bag in rules[rule] and rule not in bags:
            bags.append(rule)

print(len(bags) - 1)
