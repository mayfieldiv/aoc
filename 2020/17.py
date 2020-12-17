import sys
from utils import clip, main
from collections import Counter

lines = [list(x.strip()) for x in sys.stdin]

def apply(mat, f):
    def iterate(inner, indices):
        for i, val in enumerate(inner):
            newi = indices[:] + [i]
            yield list(iterate(val, newi)) if isinstance(val, list) else f(val, newi)
    return list(iterate(mat, []))

def a(v, i):
    print(i, v)
    return v + '.'

print(apply([lines], a))

def expand3(base):
    grid = [[['.' for k in range(len(base[0][0])+2)] for j in range(len(base[0])+2)] for i in range(len(base)+2)]
    for i, row in enumerate(base):
        for j, col in enumerate(row):
            for k, val in enumerate(col):
                grid[i+1][j+1][k+1] = base[i][j][k]
    return grid

def expand4(base):
    grid = [[[['.' for l in range(len(base[0][0][0])+2)] for k in range(len(base[0][0])+2)] for j in range(len(base[0])+2)] for i in range(len(base)+2)]
    for i, row in enumerate(base):
        for j, col in enumerate(row):
            for k, wat in enumerate(col):
                for l, val in enumerate(wat):
                    grid[i+1][j+1][k+1][l+1] = base[i][j][k][l]
    return grid

expand4([lines])

def solve1():
    grid = expand3(lines)
    for step in range(6):
        print(step)
        nxt = [[['.' for _ in col] for col in row] for row in grid]
        for i, row in enumerate(grid):
            for j, col in enumerate(row):
                for k, val in enumerate(col):
                    n = sum(grid[i+di][j+dj][k+dk] == '#' for di in (-1,0,1) for dj in (-1,0,1) for dk in (-1,0,1)
                        if 0 <= i+di < len(grid) and 0 <= j+dj < len(grid[i]) and 0 <= k+dk < len(grid[i][j]) and (di,dj,dk) != (0,0,0))
                    if n == 3 or (val == '#' and n == 2):
                        nxt[i][j][k] = '#'
        grid = expand3(nxt)
    return sum(grid[i][j][k] == '#' for i in range(len(grid)) for j in range(len(grid[0])) for k in range(len(grid[0][0])))

def solve2():
    grid = expand4([lines])
    for step in range(6):
        print(step)
        nxt = [[[['.' for _ in wat] for wat in col] for col in row] for row in grid]
        for i, row in enumerate(grid):
            for j, col in enumerate(row):
                for k, wat in enumerate(col):
                    for l, val in enumerate(wat):
                        n = sum(grid[i+di][j+dj][k+dk][l+dl] == '#' for di in (-1,0,1) for dj in (-1,0,1) for dk in (-1,0,1) for dl in (-1,0,1)
                            if 0 <= i+di < len(grid) and 0 <= j+dj < len(grid[i]) and 0 <= k+dk < len(grid[i][j]) and 0 <= l+dl < len(grid[i][j][k]) and (di,dj,dk,dl) != (0,0,0,0))
                        if n == 3 or (val == '#' and n == 2):
                            nxt[i][j][k][l] = '#'
        grid = expand4(nxt)
    return sum(grid[i][j][k][l] == '#' for i in range(len(grid)) for j in range(len(grid[0])) for k in range(len(grid[0][0])) for l in range(len(grid[0][0][0])))

main(solve1, solve2)
