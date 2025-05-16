#!/usr/bin/env python
# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

import struct
import sys
from mediafx import MediaFX

XSHIFT = 0.5
YSHIFT = 0.25

def main():
    client = MediaFX()
    width, height = client.frame_size
    pixels = width * height
    rendered_frame = bytearray(client.frame_bytecount)
    frames = [bytearray(client.frame_bytecount)]

    while True:
        time = client.render_begin(frames)

        xshift = int(XSHIFT * width);
        yshift = int(YSHIFT * height);
        for dy in range(0, height):
            for dx in range(0, width):
                sy = (dy + yshift) % height
                sx = (dx + xshift) % width
                dest_index = (dy * width + dx) * 4
                source_index = (sy * width + sx) * 4
                for channel in range(0, 4):
                    rendered_frame[dest_index + channel] = frames[0][source_index + channel]

        client.render_finish(rendered_frame)

if __name__ == "__main__":
    main()
