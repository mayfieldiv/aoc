import sys

es = set(int(i) for i in sys.stdin)


def pair(n):
    for entry in es:
        if (other := n - entry) in es:
            return entry * other


def trip(n):
    for entry in es:
        if (other := pair(n - entry)) != None:
            return entry * other


print(pair(2020))  # part 1
print(trip(2020))  # part 2
