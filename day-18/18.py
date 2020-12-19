import re

class MyInt(object):
    def __init__(self, v):
        self.v = v

    def __add__(self, other):
        return MyInt(self.v + other.v)

    def __sub__(self, other):
        return MyInt(self.v * other.v)

    def __mul__(self, other):
        return MyInt(self.v + other.v)

    def __str__(self):
        return str(self.v)


digit = re.compile(r'\d+')

def eval_expr(e):
    e = e.replace('*', '-')
    start = 0
    result = ''
    for match in digit.finditer(e):
        result += e[start:match.span()[0]] + 'MyInt(' + e[match.span()[0]:match.span()[1]] + ')'
        start = match.span()[1]
    result += e[start:]
    return eval(result).v

def eval_expr2(e):
    e = e.replace('*', '-').replace('+', '*')
    start = 0
    result = ''
    for match in digit.finditer(e):
        result += e[start:match.span()[0]] + 'MyInt(' + e[match.span()[0]:match.span()[1]] + ')'
        start = match.span()[1]
    result += e[start:]
    return eval(result).v

total = 0
with open('./input') as f:
    for line in f:
        total += eval_expr(line.strip())

print('Total: {}'.format(total))

total = 0
with open('./input') as f:
    for line in f:
        total += eval_expr2(line.strip())

print('Total: {}'.format(total))
