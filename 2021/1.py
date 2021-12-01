import utils, sys

lines = [int(x) for x in sys.stdin]

def solve1():
    c = 0
    last = -1
    for v in lines:
        if v > last:
            c += 1
        last = v
    return c

def solve2():
    c = 0
    last = -1
    for i in range(len(lines)-3):
        v = sum(lines[i:i+3])
        if v > last:
            c += 1
        last = v
    return c

utils.main(solve1, solve2)
