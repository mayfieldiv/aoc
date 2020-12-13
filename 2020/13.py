# 13.in.txt
import sys, re, math, collections, functools
from utils import clip
# from collections import defaultdict, Counter
# from functools import reduce, lru_cache

# lines = [x.strip().split('\n') for x in sys.stdin.read().split("\n\n")]
# lines = [x.strip() for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))

start = int(input())
ids = input().split(',')

def solve():
    valid = [int(x) for x in ids if x != 'x']
    wait = 999999
    best = 0
    for v in valid:
        m = v - (start % v)
        if m < wait:
            wait = m
            best = v
        print(m, v, wait, best)
    return wait * best

def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1
 

def solve2():
    valid = [(int(x), int(x)-i) for i, x in enumerate(ids) if x != 'x']
    print(valid)
    prod = math.prod([x[0] for x in valid])
    sum = 0
    for n_i, a_i in valid:
        p = prod / n_i
        sum += a_i * mul_inv(p, n_i) * p
    return int(sum % prod)

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

print("\n******************* PART 2 *******************")
s2 = solve2()
print("solve2():", s2)

if s2 != None: clip(s2)
elif s1 != None: clip(s1)
