import subprocess
import platform

def clip(obj):
    s = str(obj).strip()
    if platform.system() == 'Darwin':
        clip_cmd = 'pbcopy'
    elif platform.system() == 'Windows':
        clip_cmd = 'clip'
    else:
        clip_cmd = 'xsel --clipboard'

    subprocess.check_call(f'printf {s} | {clip_cmd}', shell=True)
    print('Copied "' + s + '" to clipboard')

def main(solve1, solve2):
    print('******************* PART 1 *******************')
    s1 = solve1()
    print('solve1():', s1)

    print('\n******************* PART 2 *******************')
    s2 = solve2()
    print('solve1():', s1)
    print('solve2():', s2)

    if s2 != None:
        clip(s2)
        return s2
    elif s1 != None:
        clip(s1)
        return s1

