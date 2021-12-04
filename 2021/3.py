import utils, sys
from collections import Counter

lines = [x.strip() for x in sys.stdin]

def solve1():
    gamma = []
    eps = []
    for p in zip(*lines):
        c = Counter(p)
        gamma.append(c.most_common()[0][0])
        eps.append(c.most_common()[-1][0])
    gamma = ''.join(gamma)
    eps = ''.join(eps)
    print(gamma, eps)
    return int(gamma, 2) * int(eps, 2)

def solve2():
    ox = lines[:]
    for i in range(len(ox[0])):
        if len(ox) == 1:
            break
        c = Counter(x[i] for x in ox)
        most = '0' if c['0'] > c['1'] else '1'
        ox = [x for x in ox if x[i] == most]

    co2 = lines[:]
    for i in range(len(co2[0])):
        if len(co2) == 1:
            break
        c = Counter(x[i] for x in co2)
        most = '0' if c['0'] <= c['1'] else '1'
        co2 = [x for x in co2 if x[i] == most]
    
    return int(ox[0], 2) * int(co2[0], 2)

utils.main(solve1, solve2)
