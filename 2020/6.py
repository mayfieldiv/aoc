import sys

groups = [x.strip().split() for x in sys.stdin.read().split("\n\n")]
count = 0
for g in groups:
    # count += len(set(''.join(g))) # part 1
    s = set(g[0])
    for p in g[1:]:
        s.intersection_update(p)
    # print(s, g)
    count += len(s)
print(count)
