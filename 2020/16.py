import sys, re, math, itertools, heapq
from utils import clip, main
from collections import Counter, defaultdict, deque, OrderedDict
from functools import reduce, lru_cache

# lines = [x.strip() for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))
fields, mine, near = [x.strip().split('\n') for x in sys.stdin.read().split("\n\n")]
lf = len(fields)

def in_range(v, r):
    a, b = map(int, r.split('-'))
    # print(v, r, a <= v <= b)
    return a <= v <= b

def valid(v, field):
    return any(in_range(int(v), r) for r in field.split(': ')[1].split(' or '))

def solve1():
    invalid = []
    for tickets in near[1:]:
        values = [int(x) for x in tickets.split(',')]
        for i, v in enumerate(values):
            if not any(valid(v, fields[ii]) for ii in range(lf)):
                invalid.append(v)

    print(invalid)
    return sum(invalid)

def solve2():
    invalid = []
    for i, tickets in enumerate(near[1:]):
        values = [int(x) for x in tickets.split(',')]
        for v in values:
            if not any(valid(v, fields[ii]) for ii in range(lf)):
                invalid.append(i)
    tickets = mine[1:] + [x for i, x in enumerate(near[1:]) if i not in invalid]

    # print(tickets)
    poss = [[] for _ in range(lf)]
    for field in fields:
        for i in range(lf):
            # print(i, sorted([int(x.split(',')[i]) for x in tickets]), field)
            if all(valid(ticket.split(',')[i], field) for ticket in tickets):
                # print('TRUE')
                poss[i].append(field)

    final = [None] * lf
    while len([x for x in final if x != None]) < len(final):
        for i, p in enumerate(poss):
            print(i, [x.split(':')[0] for x in p])
        for i, p in enumerate(poss):
            if len(p) == 1:
                loner = p[0]
                final[i] = loner
                for x in poss:
                    if loner in x:
                        x.remove(loner)
                break
    
    print(final)
    return math.prod(int(mine[1].split(',')[i]) for i, field in enumerate(final) if field.startswith('departure'))

main(solve1, solve2)