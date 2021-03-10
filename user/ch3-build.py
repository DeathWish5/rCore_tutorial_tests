import os

def set_base_address(old, new):
    linker = 'src/linker.ld'
    lines = []
    with open(linker, 'r') as f:
        for line in f.readlines():
            line = line.replace(hex(old), hex(new))
            lines.append(line)
    with open(linker, 'w+') as f:
        f.writelines(lines)

def build(apps, base_address):
    app_id = 0
    address = base_address
    step = 0x20000
    for app in apps:
        os.system('cargo build --bin %s --release' % app)
        print('[build.py] application %s start with address %s' %(app, hex(address)))
        set_base_address(address, address+step)
        address = address+step
    set_base_address(address, base_address)

if __name__ == '__main__':
    origin_base_address = 0x0
    target_base_address = 0x80400000
    set_base_address(origin_base_address, target_base_address)
    apps = os.listdir('src/bin')
    apps.sort()
    base, yield_, stride, others = [], [], [], []
    for app in apps:
        app = app[:app.find('.')]
        if app.startswith('ch2') or app.startswith('ch3_0') or app.startswith('ch3t'):
            base.append(app)
        elif app.startswith('ch3_1'):
            yield_.append(app)
        elif app.startswith('ch3_2'):
            stride.append(app)
        else:
            others.append(app)
    build(base, target_base_address)
    build(yield_, target_base_address)
    build(stride, target_base_address)
    build(others, target_base_address)
    set_base_address(target_base_address, origin_base_address)

