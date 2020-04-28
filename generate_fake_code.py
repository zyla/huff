import random

for _ in range(256):
    nbits = random.randint(0, 256);
    s = "{ %d, { " % nbits
    for i in range(4):
        r = random.randint(0, 0xffffffffffffffff)

        if nbits <= 0:
            r = 0
        elif nbits < 64:
            r &= (1 << nbits) - 1

        s += "0x%016x, " % r
        nbits -= 64

    s += " } },"
    print(s)
