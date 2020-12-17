import sys, itertools
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
    val = mat
    while isinstance(val, list):
        shape.append(len(val))
        if len(val) == 0:
            break
        val = val[0]
    return shape

def mat_sum(mat, selector=None):
    return sum((selector(val) if selector != None else val) for val, *_ in mat_iterate(mat))

def mat_index(mat, index):
    res = mat
    for i in index:
        res = res[i]
    return res

def mat_set_index(mat, index, val):
    res = mat
    for i in index[:-1]:
        res = res[i]
    res[index[-1]] = val

def mat_create(shape, val):
    return [mat_create(shape[1:], val) if len(shape) > 1 else val for _ in range(shape[0])]

def mat_neighbors(mat, index):
    shape = mat_shape(mat)
    dims = len(shape)
    for delta in itertools.product((-1, 0, 1), repeat=dims):
        if delta != tuple([0]*dims) and all(0 <= index[i]+delta[i] < shape[i] for i in range(dims)):
            neigh_index = [x+d for x,d in zip(index, delta)]
            yield [mat_index(mat, neigh_index), *neigh_index]

def expand(mat):
    shape = mat_shape(mat)
    new = mat_create([x+2 for x in shape], '.')
    mat_apply(mat, lambda val,*index: mat_set_index(new, [x+1 for x in index], val))
    return new

def solve():
    global lines
    lines = [lines]
    grid = expand(lines)
    for step in range(6):
        shape = mat_shape(grid)
        print(step, shape, mat_sum(grid, lambda x: x == '#'))
        def update(val,*index):
            n = sum(neighbor == '#' for neighbor, *_ in mat_neighbors(grid, index))
            return '#' if n == 3 or (val == '#' and n == 2) else '.'

        grid = expand(mat_apply(grid, update))

    return mat_sum(grid, lambda x: x == '#')

main(solve, solve)
