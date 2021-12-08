import utils, sys, re, math, itertools, heapq
from collections import Counter, defaultdict, deque, OrderedDict
from functools import partial, reduce, lru_cache
from pprint import pprint

lines = [x.strip().split(" | ") for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))
# lines = [x.strip().split('\n') for x in sys.stdin.read().split('\n\n')]

d = { 2: [1], 3: [7], 4: [4], 5: [2, 3, 5], 6: [0, 6, 9], 7: [8] }
d2 = {'a': 8, 'b': 6, 'c': 8, 'd': 7, 'e': 4, 'f': 9, 'g': 7}

# a = 7 - 1
# g = in 8, not in 1,4,7

def solve1():
    count = 0
    for signal, output in lines:
        outputs = [len(x) for x in output.split()]
        for o in outputs:
            if len(d[o]) == 1:
                count += 1

    return count

def solve2():
    count = 0
    for signal, output in lines:
        outputs = output.split()
        signals = signal.split()
        c = Counter()
        for s in signals:
            c.update(s)
        

    return count

utils.main(solve1, solve2)
