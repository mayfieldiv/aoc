import sys
from utils import clip, main
from collections import Counter, defaultdict

lines = [x.strip() for x in sys.stdin]

algct = Counter()
ingct = Counter()
poss = defaultdict(Counter)
for line in lines:
    ings, algs = line.split(' (contains ')
    algs = algs[:-1].split(', ')
    ings = ings.split()
    algct.update(algs)
    ingct.update(ings)
    for alg in algs:
        poss[alg].update(ings)
matches = dict()
while len(algct) > 0:
    for alg, count in algct.items():
        p = [x for x, c in poss[alg].items() if c >= count]
        assert len(p) > 0
        if len(p) == 1:
            match = p[0]
            matches[alg] = match
            algct.pop(alg)
            ingct.pop(match)
            for alg2 in algct:
                poss[alg2].pop(match, 0)
            break

def solve1():
    print(ingct)
    return sum(ingct.values())

def solve2():
    print(matches)
    return ','.join([matches[i] for i in sorted(matches.keys())])

main(solve1, solve2)
