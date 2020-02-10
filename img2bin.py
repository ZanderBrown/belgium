#!/usr/bin/python3

import sys

with open(sys.argv[1], 'r') as img:
    with open(sys.argv[1] + '.bin', 'wb') as bin:
        line = img.readline()
        if line != 'v2.0 raw':
            print('Unexpected format!')
        line = img.readline()
        while line:
            byte = bytes.fromhex(line)
            bin.write(byte)
            line = img.readline()