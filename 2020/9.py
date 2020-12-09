import sys

w = [int(x.strip()) for x in sys.stdin]

def solve():
    for i in range(25, len(w)):
        val = w[i]
        previous = set(w[i-25 : i])
        valid = any(val-x in previous for x in previous)
        if not valid:
            print(previous)
            return val

def solve2(target):
    for i in range(len(w)):
        for j in range(i+1, len(w)):
            sub = w[i:j]
            s = sum(sub)
            if s > target:
                break
            if s == target:
                print(sub)
                return min(sub) + max(sub)

print(target := solve())
print(solve2(target))