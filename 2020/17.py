import sys
from utils import clip, main
from collections import Counter

lines = [list(x.strip()) for x in sys.stdin]

def mat_iterate(mat):
    def iterate(inner, indices):
        for i, val in enumerate(inner):
            newi = indices[:] + [i]
            if isinstance(val, list):
                yield from iterate(val, newi)
            else:
                yield [val, *newi]
    yield from iterate(mat, [])

def mat_apply(mat, func):
    def iterate(inner, indices):
        for i, val in enumerate(inner):
            newi = indices[:] + [i]
            if isinstance(val, list):
                yield list(iterate(val, newi))
            else:
                yield func(val, *newi)
    return list(iterate(mat, []))

def mat_shape(mat):
    shape = []
    while isinstance(mat, list):
        shape.append(len(mat))
        if len(mat) == 0:
            break
        mat = mat[0]
    return shape

def mat_sum(mat, selector = None):
    return sum((selector(val) if selector != None else val) for val, *_ in mat_iterate(mat))

# def mat_neighbors(mat):


def a(v, *i):
    print(i, v)
    return v + '.'


for v, *i in mat_iterate([lines]):
    print(i, v)

def expand3(base):
    shape = mat_shape(base)
    grid = [[['.'
        for k in range(shape[2]+2)]
        for j in range(shape[1]+2)]
        for i in range(shape[0]+2)]
    def update(val,i,j,k): grid[i+1][j+1][k+1] = val
    mat_apply(base, update)
    return grid

def expand4(base):
    grid = [[[['.'
        for l in range(shape[3]+2)]
        for k in range(shape[2]+2)]
        for j in range(shape[1]+2)]
        for i in range(shape[0]+2)]
    def update(val,i,j,k,l): grid[i+1][j+1][k+1][l+1] = val
    mat_apply(base, update)
    return grid

def solve1():
    grid = expand3([lines])
    for step in range(6):
        shape = mat_shape(grid)
        print(step, shape)
        def update(val,i,j,k):
            n = sum(grid[i+di][j+dj][k+dk] == '#'
                for di in (-1,0,1)
                for dj in (-1,0,1)
                for dk in (-1,0,1)
                if (di,dj,dk) != (0,0,0)
                and 0 <= i+di < shape[0]
                and 0 <= j+dj < shape[1]
                and 0 <= k+dk < shape[2]
            )
            return '#' if n == 3 or (val == '#' and n == 2) else '.'

        grid = expand3(mat_apply(grid, update))

    return mat_sum(grid, lambda x: x == '#')

def solve2():
    grid = expand4([[lines]])
    for step in range(6):
        print(step)
        def update(val,i,j,k,l):
            n = sum(grid[i+di][j+dj][k+dk][l+dl] == '#'
                for di in (-1,0,1)
                for dj in (-1,0,1)
                for dk in (-1,0,1)
                for dl in (-1,0,1)
                if (di,dj,dk,dl) != (0,0,0,0)
                and 0 <= i+di < shape[0]
                and 0 <= j+dj < shape[1]
                and 0 <= k+dk < shape[2]
                and 0 <= l+dl < shape[3]
            )
            return '#' if n == 3 or (val == '#' and n == 2) else '.'

        grid = expand4(mat_apply(grid, update))

    return mat_sum(grid, lambda x: x == '#')

main(solve1, solve2)
