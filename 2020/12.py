import sys, math
from utils import clip

lines = [x.strip() for x in sys.stdin]

def solve():
    d = 0
    pos = [0, 0]
    for inst in lines:
        inst, val = inst[0], int(inst[1:])
        if inst == 'R': d = (d - val) % 360
        elif inst == 'L': d = (d + val) % 360
        elif inst == 'N': pos[1] += val
        elif inst == 'S': pos[1] -= val
        elif inst == 'E': pos[0] += val
        elif inst == 'W': pos[0] -= val
        elif inst == 'F':
            pos[0] += val * math.cos(math.radians(d))
            pos[1] += val * math.sin(math.radians(d))

    print(abs(pos[0]) + abs(pos[1]))
    return round(abs(pos[0]) + abs(pos[1]))

def solve2():
    way = [0, 0]
    pos = [0, 0]
    d, mag = 0, 0

    def trans(t):
        nonlocal d, mag
        way[0] += t[0]
        way[1] += t[1]

        diff = [way[0] - pos[0], way[1] - pos[1]]
        d = math.atan2(diff[1], diff[0])
        mag = math.sqrt(diff[0] ** 2 + diff[1] ** 2)

    def rot(degs):
        nonlocal d
        d = math.radians((math.degrees(d) + degs) % 360)
        way[0] = pos[0] + mag * math.cos(d)
        way[1] = pos[1] + mag * math.sin(d)
        pass

    trans([10, 1])

    print(pos, way, d, mag)
    for inst in lines:
        inst, val = inst[0], int(inst[1:])
        if inst == 'R': rot(-val)
        elif inst == 'L': rot(val)
        elif inst == 'N': trans([0, val])
        elif inst == 'S': trans([0, -val])
        elif inst == 'E': trans([val, 0])
        elif inst == 'W': trans([-val, 0])
        elif inst == 'F':
            x = val * mag * math.cos(d)
            y = val * mag * math.sin(d)
            pos[0] += x
            pos[1] += y
            way[0] += x
            way[1] += y
        print(pos, way, d, mag)

    print(abs(pos[0]) + abs(pos[1]))
    return round(abs(pos[0]) + abs(pos[1]))

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

print("\n******************* PART 2 *******************")
s2 = solve2()
print("solve2():", s2)

if s2 != None: clip(s2)
elif s1 != None: clip(s1)
