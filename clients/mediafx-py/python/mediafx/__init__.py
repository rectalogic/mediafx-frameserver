# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

from collections.abc import Buffer
from . import _mediafx


class MediaFX:
    def __init__(self):
        self.client = _mediafx.MediaFX()
        width, height = self.client.frame_size
        byte_count = width * height * 4
        self.frames = [bytearray(byte_count) for _ in range(self.client.frame_count)]

    def render_begin(self) -> float:
        return self.client.render_begin(self.frames)

    def render_finish(self, frame: Buffer) -> None:
        self.client.render_finish(frame)
