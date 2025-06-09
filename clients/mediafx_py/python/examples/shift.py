#!/usr/bin/env python
# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

from mediafx import MediaFX


def main() -> None:
    client = MediaFX()
    width, height = client.frame_size
    rendered_frame = bytearray(client.frame_bytecount)
    frames = [bytearray(client.frame_bytecount)]

    while True:
        (_, xshift, yshift, _) = client.render_frame(frames)

        xshift = int(xshift * width)
        yshift = int(yshift * height)
        for dy in range(0, height):
            for dx in range(0, width):
                sy = (dy + yshift) % height
                sx = (dx + xshift) % width
                dest_index = (dy * width + dx) * 4
                source_index = (sy * width + sx) * 4
                for channel in range(0, 4):
                    rendered_frame[dest_index + channel] = frames[0][source_index + channel]

        client.render_commit(rendered_frame)


if __name__ == "__main__":
    main()
