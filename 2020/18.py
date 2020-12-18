import sys, re
from utils import clip, main

lines = [x.strip() for x in sys.stdin]

class A:
    def __init__(self, a):
        self.a = a
    def __truediv__(self, o): # + => /
        return A(self.a + o.a)
    def __mul__(self, o):
        return A(self.a * o.a)
    def __add__(self, o): # * => +
        return A(self.a * o.a)

def solve1():
    return sum(eval(re.sub(r'(\d+)', r'A(\1)', line).replace('+', '/')).a for line in lines)

def solve2():
    return sum(eval(re.sub(r'(\d+)', r'A(\1)', line).replace('+', '/').replace('*', '+')).a for line in lines)

main(solve1, solve2)
