# 25.in.txt
import sys, re, math, itertools, heapq
from utils import clip, main
from collections import Counter, defaultdict, deque, OrderedDict
from functools import partial, reduce, lru_cache


# lines = [x.strip() for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))
# lines = [x.strip().split('\n') for x in sys.stdin.read().split('\n\n')]

def step(sn, val):
    return (sn * val) % 20201227

def crack(sn, pk):
    res = 1
    i = 0
    while res != pk:
        if i % 100000 == 0: print(i)
        res = step(sn, res)
        i += 1
    return i

doorpk = int(input())
cardpk = int(input())

def solve1():
    cardsn = 7
    doorsn = 7
    # doorls = crack(doorsn, doorpk)
    # doorls = 14665099
    cardls = crack(cardsn, cardpk)
    # cardls = 10092352
    enckey = 1
    for i in range(cardls):
        if i % 100000 == 0: print(i)
        enckey = step(doorpk, enckey)
    print(cardls, doorpk, enckey)
    return enckey

def solve2():
    
    return

main(solve1, solve2)
