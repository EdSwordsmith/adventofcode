with open("input.txt") as f:
    data = f.readlines()

possible = {}
ingredient_list = {}

for item in data:
    ingredients, allergens = item.replace("(", "").replace(")", "").replace("\n", "").split(" contains ")
    ingredients = ingredients.split(" ")

    for ing in ingredients:
        if ing not in ingredient_list:
            ingredient_list[ing] = 1
        else:
            ingredient_list[ing] += 1

    allergens = allergens.split(", ")
    for allergen in allergens:
        if allergen not in possible:
            possible[allergen] = ingredients
        else:
            possible[allergen] = [ingredient for ingredient in possible[allergen] if ingredient in ingredients]

for item in possible:
    for ing in possible[item]:
        if ing in ingredient_list:
            del(ingredient_list[ing])

total = 0
for ing in ingredient_list:
    total += ingredient_list[ing]
print(total)

while True:
    changed = False
    for allergen in possible:
        if len(possible[allergen]) == 1:
            for a in possible:
                if a == allergen:
                    continue
                if possible[allergen][0] in possible[a]:
                    changed = True
                    possible[a].remove(possible[allergen][0])
    if not changed:
        break

print(",".join(list(dict.fromkeys([ing for k in sorted(possible.keys()) for ing in possible[k]]))))
