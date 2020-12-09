import math
import sys
g = [x.strip() for x in sys.stdin]


def sled(r, d):
    count = 0
    for i in range(1, len(g) // d):
        if g[d * i][r * i % len(g[0])] == '#':
            count += 1
    return count


print(sled(3, 1))  # part 1

# part 2
print(math.prod(sled(r, d)
                for r, d in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]))
