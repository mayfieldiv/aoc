# 23.in.txt
import sys, re, math, itertools, heapq
from utils import clip, main
from collections import Counter, defaultdict, deque, OrderedDict
from functools import partial, reduce, lru_cache

# lines = [x.strip() for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))
# lines = [x.strip().split('\n') for x in sys.stdin.read().split('\n\n')]
cups = list(map(int, input()))

def solve1():
    d = deque(cups)
    m = max(cups)
    lastidx = 0
    for _ in range(100):
        print(d)
        curr = d[0]
        d.rotate(-1)
        hold = (d.popleft(), d.popleft(), d.popleft())
        for j in range(1, 5):
            label = curr-j
            if label < 1:
                label = ((label-1) % m)+1
            if label != hold[0] and label != hold[1] and label != hold[2]:
                if d[lastidx-1] == label:
                    idx = lastidx-1
                else:
                    idx = d.index(label)
                lastidx = idx
                [d.insert(idx+1, x) for x in reversed(hold)]
                break
    d.rotate(-d.index(1))
    print(d)
    return ''.join(map(str, list(d)[1:]))

def solve2():
    d = deque(cups)
    m = 1_000_000
    num_cups = 1_000_000
    for i in range(max(cups)+1, num_cups+1):
        d.append(i)
    lastidx = 0
    moves = 10_000_000
    for move in range(moves):
        if move % 1000 == 0:
            print(move)
        curr = d[0]
        d.rotate(-1)
        hold = (d.popleft(), d.popleft(), d.popleft())
        for j in range(1, 5):
            label = curr-j
            if label < 1:
                label = ((label-1) % m)+1
            if label != hold[0] and label != hold[1] and label != hold[2]:
                idx = -1
                fudge = 3 + move // 10000
                i = lastidx-1
                while 0 < i < min(lastidx+fudge, num_cups-4):
                    if d[i] == label:
                        idx = i
                        break
                    i = i+2*(lastidx-i) if i < lastidx else i+1
                if idx == -1:
                    idx = d.index(label)
                lastidx = idx
                [d.insert(idx+1, x) for x in reversed(hold)]
                break
    d.rotate(-d.index(1))
    print(list(d)[:10])
    return d[1] * d[2]

main(solve1, solve2)
