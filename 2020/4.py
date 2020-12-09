import sys, re

ps = [{y.split(':')[0]: y.split(':')[1] for y in x.split()}
      for x in sys.stdin.read().split("\n\n")]

req = ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']
opt = ['cid']

def valid(p, field):
    # return field in p # part 1
    # part 2
    if field not in p:
        return False
    try:
        val = p[field]
        # print(field, val)
        if field == 'byr':
            return 1920 <= int(val) <= 2002
        if field == 'iyr':
            return 2010 <= int(val) <= 2020
        if field == 'eyr':
            return 2020 <= int(val) <= 2030
        if field == 'hgt':
            if val[-2:] == 'cm':
                return 150 <= int(val[:-2]) <= 193
            elif val[-2:] == 'in':
                return 59 <= int(val[:-2]) <= 76
            else:
                return False
        if field == 'hcl':
            return re.fullmatch('#[0-9a-f]{6}', val) != None
        if field == 'ecl':
            return val in 'amb blu brn gry grn hzl oth'.split()
        if field == 'pid':
            return re.fullmatch('\d{9}', val) != None
        return True
    except:
        return False

print(sum(1 if all(valid(p, r) for r in req) else 0 for p in ps))
