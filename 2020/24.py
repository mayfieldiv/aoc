import sys
from utils import clip, main
from collections import defaultdict

lines = [x.strip() for x in sys.stdin]
neigh = {'e': (1, 0), 'w': (-1, 0), 'nw': (-1, 1), 'se': (1, -1), 'ne': (0, 1), 'sw': (0, -1)}

def solve1():
    tiles = set()
    for line in lines:
        pos = [0, 0]
        i = 0
        while i < len(line):
            d = line[i]
            i += 1
            if d not in neigh:
                d = d + line[i]
                i += 1
            n = neigh[d]
            pos[0] += n[0]
            pos[1] += n[1]
        pos = tuple(pos)
        if pos in tiles:
            tiles.remove(pos)
        else:
            tiles.add(pos)

    return len(tiles)


def solve2():
    tiles = set()
    for line in lines:
        pos = [0, 0]
        i = 0
        while i < len(line):
            d = line[i]
            i += 1
            if d not in neigh:
                d = d + line[i]
                i += 1
            n = neigh[d]
            pos[0] += n[0]
            pos[1] += n[1]
        pos = tuple(pos)
        if pos in tiles:
            tiles.remove(pos)
        else:
            tiles.add(pos)

    for day in range(100):
        remove = []
        new = defaultdict(int)
        for tile in tiles:
            bc = 0
            for nd in neigh.values():
                n = (tile[0]+nd[0], tile[1]+nd[1])
                if n in tiles:
                    bc += 1
                else:
                    new[n] += 1
            if bc == 0 or bc > 2:
                remove.append(tile)
        for tile in remove:
            tiles.remove(tile)
        for tile, bc in new.items():
            if bc == 2:
                tiles.add(tile)

    return len(tiles)

main(solve1, solve2)
