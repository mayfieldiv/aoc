import math
from utils import clip

start = int(input())
ids = input().split(',')

def solve():
    valid = [int(x) for x in ids if x != 'x']
    best = (valid[0], valid[0])
    for v in valid:
        m = -start % v
        if m < best[1]:
            best = v, m
        print(v, m, best)
    return math.prod(best)

def mul_inv(a, mod):
    for x in range(1, mod):
        if (a * x) % mod == 1:
            return x

def solve2():
    valid = [(int(x), -i) for i, x in enumerate(ids) if x != 'x']
    print(valid)
    # chinese remainder
    prod = math.prod([x[0] for x in valid])
    sum = 0
    for n, a in valid:
        p = prod // n
        sum += a * mul_inv(p, n) * p
    print(sum)
    return sum % prod

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

print("\n******************* PART 2 *******************")
s2 = solve2()
print("solve2():", s2)

if s2 != None: clip(s2)
elif s1 != None: clip(s1)
