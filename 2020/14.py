import sys, re
from utils import clip

lines = [x.strip() for x in sys.stdin]

def solve():
    mem = dict()
    for line in lines:
        if match := re.match(r'mask = (.*)', line):
            mask = match[1]
            ones = sum([1 << i for i, v in enumerate(mask[::-1]) if v == '1'])
            zeros = sum([1 << i for i, v in enumerate(mask[::-1]) if v == '0'])
        else:
            loc, val = map(int, re.match(r'mem\[(.*)\] = (.*)', line).groups())
            mem[loc] = (val | ones) & ~zeros
            # print(loc, val, mem[loc])

    return sum(mem.values())

def solve2():
    mem = dict()
    for line in lines:
        if match := re.match(r'mask = (.*)', line):
            mask = match[1]
            base_ones = sum([1 << i for i, v in enumerate(mask[::-1]) if v == '1'])
            floating = [i for i, v in enumerate(mask[::-1]) if v == 'X']
        else:
            base, val = map(int, re.match(r'mem\[(.*)\] = (.*)', line).groups())
            base = base | base_ones
            for bits in range(1 << len(floating)):
                ones = sum([1 << v for i, v in enumerate(floating) if bits & (1 << i)])
                zeros = sum([1 << v for i, v in enumerate(floating) if not bits & (1 << i)])
                loc = (base | ones) & ~zeros
                mem[loc] = val
                # print(loc, val, base, ' '.join([f'{1 << v}{"✖✔"[bits & (1 << i) != 0]}' for i, v in enumerate(floating)]))

    return sum(mem.values())

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

print("\n******************* PART 2 *******************")
s2 = solve2()
print("solve2():", s2)

if s2 != None: clip(s2)
elif s1 != None: clip(s1)
