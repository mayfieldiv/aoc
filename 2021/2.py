import utils, sys

lines = [x.strip() for x in sys.stdin]

def solve1():
    p, d = 0, 0
    for line in lines:
        c, v = line.split()
        v = int(v)
        if (c == "forward"):
            p += v
        elif (c == "down"):
            d += v
        elif (c == "up"):
            d -= v
    return p * d

def solve2():
    p, d, aim = 0, 0, 0
    for line in lines:
        c, v = line.split()
        v = int(v)
        if (c == "forward"):
            p += v
            d += aim * v
        elif (c == "down"):
            aim += v
        elif (c == "up"):
            aim -= v
    return p * d

utils.main(solve1, solve2)
