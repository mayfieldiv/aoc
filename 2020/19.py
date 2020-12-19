import sys, re
from utils import clip, main
from functools import lru_cache

rules, msgs = [x.strip().split('\n') for x in sys.stdin.read().split('\n\n')]
rules = dict(x.split(': ') for x in rules)

@lru_cache
def tore(rule):
    if rule in rules:
        if re.search(r'\b%s\b' % rule, rules[rule]): # part 2 self-referencing rules
            if rule == '8':
                return '%s+' % tore('42')
            if rule == '11':
                return '(%s)' % '|'.join("%s{%d}%s{%d}" % (tore('42'), i, tore('31'), i) for i in range(1, 5))
            assert False
        return tore(rules[rule])
    if match := re.fullmatch(r'"(\w+)"', rule):
        return match[1]
    if match := re.fullmatch(r'(.+) \| (.+)', rule):
        return '(%s|%s)' % (tore(match[1]), tore(match[2]))
    if match := re.fullmatch(r'(.+) (.+)', rule):
        return tore(match[1]) + tore(match[2])
    assert False

def solve1():
    for key in map(str, reversed(sorted(map(int, rules.keys())))):
        print(key, rules[key], tore(key), sep=' ::: ')

    return sum(bool(re.fullmatch(tore('0'), msg)) for msg in msgs)

def solve2():
    tore.cache_clear()
    rules['8'] = '42 | 42 8'
    rules['11'] = '42 31 | 42 11 31'
    return solve1()

main(solve1, solve2)
