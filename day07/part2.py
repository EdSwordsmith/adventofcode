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
    

def bags_count(bags):
    def bags_count_aux(bags, count):
        if len(bags) == 0:
            return count
        else:
            contained = ()
            for rule in rules[bags[0]]:
                contained += (rule,rules[bags[0]][rule])
            inside = bags_count(contained)
            return bags_count_aux(bags[2:], count + bags[1] * (inside + 1))
    return bags_count_aux(bags, 0)
    


bags = ("shiny gold", 1)
print(bags_count(bags) - 1)
