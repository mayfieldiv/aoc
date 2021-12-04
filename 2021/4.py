import utils, sys
from collections import OrderedDict

things = sys.stdin.read().split('\n\n')
draws = [int(x) for x in things[0].split(',')]
boards = [[[int(x) for x in r.split()] for r in b.split('\n')] for b in things[1:]]

def solve1():
    for i in range(len(draws)):
        drawn = set(draws[:i+1])
        def win(board):
            for r in board:
                if all(x in drawn for x in r):
                    return True
            for r in zip(*board):
                if all(x in drawn for x in r):
                    return True
            return False

        for board in boards:
            if win(board):
                print(board)
                return draws[i] * sum(sum(x for x in r if x not in drawn) for r in board)

def solve2():
    scores = OrderedDict()
    for i in range(len(draws)):
        drawn = set(draws[:i+1])
        def win(board):
            for r in board:
                if all(x in drawn for x in r):
                    return True
            for r in zip(*board):
                if all(x in drawn for x in r):
                    return True
            return False

        for b, board in enumerate(boards):
            if b not in scores and win(board):
                print(i, draws[i], board)
                scores[b] = draws[i] * sum(sum(x for x in r if x not in drawn) for r in board)

    return list(scores.values())[-1]

utils.main(solve1, solve2)
