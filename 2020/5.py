import sys

def decode(seat):
    row = int(''.join('0' if c == 'F' else '1' for c in seat[:7]), base=2)
    col = int(''.join('0' if c == 'L' else '1' for c in seat[7:]), base=2)
    id = 8 * row + col
    return id, row, col, seat

seats = [decode(x.strip()) for x in sys.stdin]
m = max(seats, key=lambda x: x[0])
print(m) # part 1

seats = {s[0]: s for s in seats}
for i in range(m[0]):
    if i not in seats:
        print("MISSING:", i)