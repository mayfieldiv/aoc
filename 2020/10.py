import sys
from utils import clip

w = [int(x) for x in sys.stdin]
w += [0, max(w) + 3]
w.sort()

def solve():
    print(w)
    c1 = c3 = 0
    for a, b in zip(w, w[1:]):
        if b-a == 1: c1 += 1
        elif b-a == 3: c3 += 1
        else: assert False
    print(c1, c3)
    return c1 * c3

def solve2():
    return
    start = 0
    count = 1
    for i, (a, b) in enumerate(zip(w, w[1:])):
        if b-a == 3:
            if (i - start > 1):
                print(w[start:i+1])
                opt = i - start - 1
                assert opt <= 3
                if opt == 3: count *= 7
                else: count *= 2**opt
            start = i+1
    return count

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

s2 = solve2()
print("\n******************* PART 2 *******************")
print("solve2():", s2)

clip(s2 if s2 != None else s1)
