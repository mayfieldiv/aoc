import sys

count = 0
for line in sys.stdin:
    r, c, s = line.strip().split()
    l, h = map(int, r.split('-'))
    c = c[0]
    cc = s.count(c)

    # if l <= cc <= h: # part 1
    if (s[l-1] == c) ^ (s[h-1] == c):  # part 2
        print(r, c, s)
        count += 1

print(count)
