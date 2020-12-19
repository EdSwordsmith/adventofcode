operators = ("+", "*")
parentheses = ("(", ")")


def get_expr(expression: str):
    expr = list(expression.replace(" ", "").replace("\n", ""))
    
    for i in range(len(expr)):
        if expr[i] not in operators and expr[i] not in parentheses:
            expr[i] = int(expr[i])
    """
    i = 0
    while i < len(expr) - 2:
        if expr[i] == "+":
            expr.insert(i - 1, "(")
            i += 1

            if expr[i + 1] == "(":
                j = i + 1
                while expr[j] != ")":
                    j += 2
                expr.insert(j , ")")
            else:
                while expr[i] == "+":
                    i += 2
                expr.insert(i , ")")
            i += 1
        i += 1
    """

    return expr


def perform_operation(v1: int, oper: str, v2: int) -> int:
    if oper == "+":
        return v1 + v2
    elif oper == "*":
        return v1 * v2


def evaluate_expression(expression: str) -> int:
    expr = get_expr(expression)
    print(expr)

    while len(expr) != 1:
        count = 0
        while count < len(expr) - 1:
            if expr[count] == "(":
                if expr[count + 2] == ")":
                    del expr[count + 2]
                    del expr[count]
            count += 1

        count = 0
        while count < len(expr) - 1:
            if expr[count] == "+" and not (expr[count+1] in parentheses or expr[count-1] in parentheses):
                expr[count - 1] = perform_operation(expr[count - 1], expr[count], expr[count + 1])
                del expr[count + 1]
                del expr[count]
            count += 1

        count = 0
        while count < len(expr) - 1:
            if expr[count] == "*" and not (expr[count+1] in parentheses or expr[count-1] in parentheses or \
                (count > 1 and expr[count - 2] == "+") or (count < len(expr) - 2 and expr[count + 2] == "+")):
                expr[count - 1] = perform_operation(expr[count - 1], expr[count], expr[count + 1])
                del expr[count + 1]
                del expr[count]
            count += 1
        print(expr)
    
    return expr[0]


with open("input.txt", "r") as f:
    expressions = f.readlines()

result = 0
for expression in expressions:
    result += evaluate_expression(expression)
print(result)

