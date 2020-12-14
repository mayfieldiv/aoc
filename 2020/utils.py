import subprocess

def clip(obj):
    s = str(obj).strip()
    cmd = 'printf ' + s + ' | xsel --clipboard'
    subprocess.check_call(cmd, shell=True)
    print('Copied "' + s + '" to clipboard')

def main(solve1, solve2):
    print("******************* PART 1 *******************")
    s1 = solve1()
    print("solve1():", s1)

    print("\n******************* PART 2 *******************")
    s2 = solve2()
    print("solve2():", s2)

    if s2 != None:
        clip(s2)
        return s2
    elif s1 != None:
        clip(s1)
        return s1

