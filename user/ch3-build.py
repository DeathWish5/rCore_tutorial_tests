import os

def build(apps):
    app_id = 0
    base_address = 0x80400000
    step = 0x20000
    linker = 'src/linker.ld'
    for app in apps:
        lines = []
        lines_before = []
        with open(linker, 'r') as f:
            for line in f.readlines():
                lines_before.append(line)
                line = line.replace(hex(base_address), hex(base_address+step*app_id))
                lines.append(line)
        with open(linker, 'w+') as f:
            f.writelines(lines)
        os.system('cargo build --bin %s --release' % app)
        print('[build.py] application %s start with address %s' %(app, hex(base_address+step*app_id)))
        with open(linker, 'w+') as f:
            f.writelines(lines_before)
        app_id = app_id + 1

if __name__ == '__main__':
    apps = os.listdir('src/bin')
    apps.sort()
    base, yield_, stride = [], [], []
    for app in apps:
        app = app[:app.find('.')]
        if app.startswith('ch2') or app.startswith('ch3_0') or app.startswith('ch3t'):
            base.append(app)
        elif app.startswith('ch3_1'):
            yield_.append(app)
        elif app.startswith('ch3_2'):
            stride.append(app)
    build(base)
    build(yield_)
    build(stride)

