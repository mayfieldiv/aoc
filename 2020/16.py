import sys, math
from utils import clip, main
from functools import partial

fields, mine, near = [x.strip().split('\n') for x in sys.stdin.read().split("\n\n")]
mine = list(map(int, mine[1].split(',')))
near = [list(map(int, t.split(','))) for t in near[1:]]

def in_range(rang, v):
    def inner(r):
        a, b = map(int, r.split('-'))
        # print(v, r, a <= v <= b)
        return a <= v <= b
    r1, r2 = rang.split(' or ')
    return inner(r1) or inner(r2)

fields = {field: partial(in_range, rang) for field, rang in [x.split(': ') for x in fields]}

def solve1():
    invalid = []
    for ticket in near:
        for v in ticket:
            if not any(f(v) for f in fields.values()):
                invalid.append(v)

    print(invalid)
    return sum(invalid)

def solve2():
    valid = [mine]
    for ticket in near:
        valid.append(ticket)
        for v in ticket:
            if not any(f(v) for f in fields.values()):
                valid.pop()
                break

    # [print(x) for x in valid]
    maybe = [{field
        for field, f in fields.items() if all(f(x[i]) for x in valid)}
            for i in range(len(fields))]

    final = [None] * len(fields)
    while not all(final):
        # [print(*x) for x in enumerate(maybe)]
        for i, poss in enumerate(maybe):
            if len(poss) == 1:
                final[i] = poss.pop()
                for x in maybe:
                    x.discard(final[i])

        print(final)

    return math.prod(mine[i] for i, field in enumerate(final) if field.startswith('departure'))

main(solve1, solve2)
