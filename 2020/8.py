import sys

instructions = sys.stdin.readlines()

def run(flip_i=-1):
    ip = 0
    acc = 0
    visited = set()
    while True:
        if ip in visited:
            break
        visited.add(ip)
        inst, num = instructions[ip].strip().split()
        if ip == flip_i:
            inst = 'acc jmp nop'.split()['acc nop jmp'.split().index(inst)]
        num = int(num)
        if inst == 'acc':
            acc += num
            ip += 1
        if inst == 'nop':
            ip += 1
        if inst == 'jmp':
            ip += num
        print(ip, acc)

run() # part 1
for i in range(len(instructions)):
    run(i) # throws when correct