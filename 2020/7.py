import sys
from functools import cache
from collections import defaultdict

g = dict()

for line in sys.stdin:
    b, contents = line.strip().split(' bags contain ')
    g[b] = []
    contents = contents.split(', ')
    for c in contents:
        if not c.startswith('no'):
            n, color = c.split(maxsplit=1)
            color = color.split(' bag')[0]
            g[b].append((color, int(n)))

@cache
def contains(bag, target):
    return target in [x[0] for x in g[bag]] or any([contains(x[0], target) for x in g[bag]])

@cache
def count(bag):
    print(bag, g[bag])
    return 1 + sum(x[1] * count(x[0]) for x in g[bag])

asdf = 'shiny gold'
print(sum(1 if contains(bag, asdf) else 0 for bag in g))
print(count(asdf) - 1)