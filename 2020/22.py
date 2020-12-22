import sys, itertools
from utils import clip, main
from collections import deque

p1, p2 = [list(map(int, x.strip().split('\n')[1:])) for x in sys.stdin.read().split('\n\n')]

def solve1():
    def run(d1, d2):
        while len(d1) > 0 and len(d2) > 0:
            a, b = d1.popleft(), d2.popleft()
            if a > b:
                d1.append(a)
                d1.append(b)
            else:
                d2.append(b)
                d2.append(a)
        return d1 if len(d2) == 0 else d2

    winner = run(deque(p1), deque(p2))
    print(winner)
    return sum(x * (len(winner) - i) for i, x in enumerate(winner))

def solve2():
    def run(d1, d2):
        prev = set()
        while len(d1) > 0 and len(d2) > 0:
            # print(d1, d2)
            curr = (tuple(d1), tuple(d2))
            if curr in prev:
                return True
            prev.add(curr)
            a, b = d1.popleft(), d2.popleft()
            if len(d1) >= a and len(d2) >= b:
                w = run(deque(itertools.islice(d1, a)), deque(itertools.islice(d2, b)))
            else:
                w = a > b
            if w:
                d1.append(a)
                d1.append(b)
            else:
                d2.append(b)
                d2.append(a)
        return len(d2) == 0

    d1, d2 = deque(p1), deque(p2)
    winner = d1 if run(d1, d2) else d2
    print(winner)
    return sum(x * (len(winner) - i) for i, x in enumerate(winner))

main(solve1, solve2)
