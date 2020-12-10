import subprocess

def clip(obj):
    s = str(obj).strip()
    cmd = 'printf ' + s + ' | xsel --clipboard'
    subprocess.check_call(cmd, shell=True)
    print('Copied "' + s + '" to clipboard')