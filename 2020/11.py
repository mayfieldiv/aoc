import sys
from utils import clip

lines = [x.strip() for x in sys.stdin]
h = len(lines)
w = len(lines[0])
neigh = [(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)]

def solve():
    curr = lines[:]
    while True:
        new = [list(x) for x in curr]
        # for x in curr: print(x)
        # print()
        for i in range(h):
            for j in range(w):
                occ = 0
                # print('me', i, j)
                for a, b in neigh:
                    ii = i+a
                    jj = j+b
                    if 0 <= ii < h and 0 <= jj < w:
                        # print("neighbor", ii, jj, curr[ii][jj])
                        occ += curr[ii][jj] == "#"

                # print(i, j, occ)
                if curr[i][j] == 'L' and occ == 0:
                    new[i][j] = '#'
                elif curr[i][j] == '#' and occ >= 4:
                    new[i][j] = 'L'
        new = [''.join(x) for x in new]
        if ''.join(new) == ''.join(curr):
            break
        curr = new[:]

    return ''.join(curr).count('#')

def solve2():
    curr = lines[:]
    while True:
        new = [list(x) for x in curr]
        # for x in curr: print(x)
        # print()
        for i in range(h):
            for j in range(w):
                occ = 0
                if curr[i][j] == '.':
                    continue
                # print('me', i, j)
                for a, b in neigh:
                    for mul in range(1, min(w, h)):
                        ii = i+a*mul
                        jj = j+b*mul
                        if 0 <= ii < h and 0 <= jj < w:
                            # print("neighbor", ii, jj, curr[ii][jj])
                            occ += curr[ii][jj] == "#"
                        else:
                            break
                        if curr[ii][jj] != '.':
                            break

                if curr[i][j] == 'L' and occ == 0:
                    new[i][j] = '#'
                elif curr[i][j] == '#' and occ >= 5:
                    new[i][j] = 'L'
        new = [''.join(x) for x in new]
        if ''.join(new) == ''.join(curr):
            break
        curr = new[:]

    return ''.join(curr).count('#')

print("******************* PART 1 *******************")
s1 = solve()
print("solve():", s1)

s2 = solve2()
print("\n******************* PART 2 *******************")
print("solve2():", s2)

if s2 != None: clip(s2)
elif s1 != None: clip(s1)
