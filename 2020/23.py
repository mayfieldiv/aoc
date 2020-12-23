from utils import clip, main

cups = list(map(int, input()))

class Node:
    def __init__(self, label):
        self.label = label
        self.next = None
    
    def __repr__(self):
        return f'{self.label}->{self.next.label}'

def solve1():
    num_cups = len(cups)
    moves = 100

    nodes = dict()
    head = prev = nodes[cups[0]] = Node(cups[0])
    for c in cups[1:]:
        prev.next = nodes[c] = Node(c)
        prev = nodes[c]
    nodes[cups[-1]].next = head
    print(nodes)

    for _ in range(moves):
        hold = head.next
        head.next = hold.next.next.next
        hold.next.next.next = None
        held = {hold.label, hold.next.label, hold.next.next.label}
        for dec in range(1, 5):
            label = head.label - dec
            if label < 1:
                label = ((label-1) % num_cups) + 1
            if label not in held:
                dest = nodes[label]
                hold.next.next.next = dest.next
                dest.next = hold
                break
        head = head.next

    print(nodes)
    out = ''
    node = nodes[1].next
    while node.label != 1:
        out += str(node.label)
        node = node.next
    return out

def solve2():
    num_cups = 1_000_000
    moves = 10_000_000
    cups.extend(range(len(cups)+1, num_cups+1))

    nodes = dict()
    head = prev = nodes[cups[0]] = Node(cups[0])
    for c in cups[1:]:
        prev.next = nodes[c] = Node(c)
        prev = nodes[c]
    nodes[cups[-1]].next = head

    for move in range(moves):
        if move % 10_000 == 0:
            print(move)

        hold = head.next
        head.next = hold.next.next.next
        hold.next.next.next = None
        held = {hold.label, hold.next.label, hold.next.next.label}
        for dec in range(1, 5):
            label = head.label - dec
            if label < 1:
                label = ((label-1) % num_cups) + 1
            if label not in held:
                dest = nodes[label]
                hold.next.next.next = dest.next
                dest.next = hold
                break
        head = head.next

    return nodes[1].next.label * nodes[1].next.next.label

main(solve1, solve2)
