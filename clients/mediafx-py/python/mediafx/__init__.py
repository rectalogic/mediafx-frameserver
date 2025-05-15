# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

from . import mediafx

class MediaFX:
    def __init__(self):
        self.client = mediafx.MediaFX()
        width, height = self.client.frame_size
        byte_count = width * height * 4
        self.frames = [bytearray(byte_count) for _ in range(self.client.frame_count)]

    def render(self) -> float:
        return self.client.render(self.frames)
