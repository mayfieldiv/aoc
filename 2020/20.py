# 20.in.txt
import sys, re, math, itertools, heapq
from utils import clip, main
from collections import Counter, defaultdict, deque, OrderedDict, namedtuple
from functools import partial, reduce, lru_cache, cached_property

Transform = namedtuple('Transform', ['xflip', 'yflip', 'rot', 'val'])

class Tile:
    def __init__(self, s):
        a = s.split('\n', maxsplit=1)
        self.id = int(a[0].split()[1][:-1])
        self.mat = a[1].split()

    def transforms(self):
        # def inner():
        for xflip in [True, False]:
            for yflip in [True, False]:
                for r in range(2):
                    yield Transform(xflip, yflip, r, self.transform(xflip, yflip, r))
        # d = dict()
        # for t in inner():
        #     d[''.join(t.val)] = t
        # return [t for t in d.values()]

    def transform(self, xflip, yflip, rot):
        t = self.mat[:]
        if xflip:
            t = t[::-1]
        if yflip:
            t = [x[::-1] for x in t]

        for r in range(rot):
            t = list(zip(*t))

        return [''.join(list(x)) for x in t]

    @cached_property
    def edges(self):
        return [Transform(t.xflip, t.yflip, t.rot, t.val[0]) for t in self.transforms()]

# lines = [x.strip() for x in sys.stdin]
# lines = [int(x) for x in sys.stdin]
# lines = list(map(int, input().split()))
tiles = list(map(Tile, sys.stdin.read().split('\n\n')))
# tiles = {t.id: t for t in tiles}

def solve1():
    return
    edges = defaultdict(set)
    # list(tiles[1951].edges())
    for i, tile in enumerate(tiles):
        print(i)
        for edge in tile.edges():
            for btile in tiles[i+1:]:
                for bedge in btile.edges():
                    # if tile.id == 1951 and btile.id == 2311:
                    #     print(edge, bedge)
                    if edge.val == bedge.val:
                        a, b = sorted([(tile.id, edge), (btile.id, bedge)])
                        edges[(a[0], b[0])].add((a[1], b[1]))
    ct = Counter()
    for a, b in edges.keys():
        ct[a] += 1
        ct[b] += 1
    corners = [x for x, c in ct.items() if c == 2]
    assert len(corners) == 4
    return math.prod(corners)

def solve2():
    edges = defaultdict(set)
    for i, tile in enumerate(tiles):
        print(i)
        for edge in tile.edges:
            if len(edges[tile.id]) == 4:
                break
            for btile in tiles[i+1:]:
                if len(edges[tile.id]) == 4:
                    break
                if btile.id in edges[tile.id] or len(edges[btile.id]) == 4:
                    continue
                for bedge in btile.edges:
                    if edge.val == bedge.val:
                        edges[tile.id].add(btile.id)
                        edges[btile.id].add(tile.id)
                        break

    corners = [x for x, e in edges.items() if len(e) == 2]
    assert len(corners) == 4

    corner = corners[0]
    

    return math.prod(corners)

main(solve1, solve2)
