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

if __name__ == '__main__':
    origin_base_address = 0x0
    target_base_address = 0x80400000
    set_base_address(origin_base_address, target_base_address)
    os.system('cargo build --release')
    set_base_address(target_base_address, origin_base_address)

