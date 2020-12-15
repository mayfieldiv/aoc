from utils import clip, main

lines = list(map(int, input().split(',')))

def solve1():
    map = {v: i for i, v in enumerate(lines[:-1])}
    last = lines[-1]
    for i in range(len(lines), 2020):
        curr = 0 if last not in map else i-1-map[last]
        map[last] = i - 1
        last = curr
        # print(curr, map)
    return last

def solve2():
    map = {v: i for i, v in enumerate(lines[:-1])}
    last = lines[-1]
    for i in range(len(lines), 30000000):
        curr = 0 if last not in map else i-1-map[last]
        map[last] = i - 1
        last = curr
        # print(curr, map)
    return last

main(solve1, solve2)