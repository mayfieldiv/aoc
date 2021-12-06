import utils
from collections import defaultdict

nums = list(map(int, input().split(",")))

def solve1():
    state = nums[:]
    for day in range(80):
        for i in range(len(state)):
            state[i] -= 1
            if state[i] < 0:
                state[i] = 6
                state.append(8)
    print(state)
    return len(state)

def solve2():
    state = defaultdict(int)
    for s in nums:
        state[s] += 1
    for day in range(256):
        new = state[0]
        for i in range(0, 8):
            state[i] = state[i+1]
        state[6] += new
        state[8] = new
        print(day, state)

    return sum(state.values())

utils.main(solve1, solve2)
