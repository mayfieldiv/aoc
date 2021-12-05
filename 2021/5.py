import utils, sys
from collections import defaultdict
from pprint import pprint

lines = [x.strip() for x in sys.stdin]

def solve(diag):
    field = defaultdict(list)
    for line in lines:
        start, end = line.split(' -> ')
        start = [int(x) for x in start.split(',')]
        end = [int(x) for x in end.split(',')]
        xs = list(range(start[0], end[0]+1) if end[0] >= start[0] else range(start[0], end[0]-1, -1))
        ys = list(range(start[1], end[1]+1) if end[1] >= start[1] else range(start[1], end[1]-1, -1))

        if diag or len(xs) == 1 or len(ys) == 1:
            for x, y in zip(xs if len(xs) > 1 else xs * len(ys), ys if len(ys) > 1 else ys * len(xs)):
                field[(x, y)].append((start, end))

    # print(pprint(field))
    return len([x for x in field.values() if len(x) > 1])

def solve1():
    return solve(False)

def solve2():
    return solve(True)

utils.main(solve1, solve2)
