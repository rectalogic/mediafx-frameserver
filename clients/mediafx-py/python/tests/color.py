#!/usr/bin/env python
# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

import struct
import sys
from mediafx import MediaFX


def main():
    client = MediaFX()
    width, height = client.frame_size
    pixels = width * height
    rendered_frame = bytearray(client.frame_bytecount)
    # frames = [bytearray(client.frame_bytecount) for _ in range(client.frame_count)]

    while True:
        time = client.render_begin()
        r = int(time % 255)
        b = int((time + 128) % 255)
        pixel = struct.pack("=4B", r, 0, b, 255)
        for outpixel in range(0, len(rendered_frame), 4):
            rendered_frame[outpixel:outpixel+4] = pixel
        client.render_finish(rendered_frame)

if __name__ == "__main__":
    main()
