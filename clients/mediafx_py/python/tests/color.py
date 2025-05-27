#!/usr/bin/env python
# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

import struct

from mediafx import MediaFX


def main() -> None:
    client = MediaFX()
    # width, height = client.frame_size
    rendered_frame = bytearray(client.frame_bytecount)

    while True:
        (_, r, g, b) = client.render_begin()
        r = int(r * 255)
        g = int(g * 255)
        b = int(b * 255)
        pixel = struct.pack("=4B", r, g, b, 255)
        for outpixel in range(0, len(rendered_frame), 4):
            rendered_frame[outpixel : outpixel + 4] = pixel
        client.render_finish(rendered_frame)


if __name__ == "__main__":
    main()
