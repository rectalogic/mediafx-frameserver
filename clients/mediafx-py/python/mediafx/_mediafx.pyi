# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

from collections.abc import Buffer

class MediaFX:
    @property
    def frame_size(self) -> tuple[int, int]: ...

    @property
    def frame_count(self) -> int: ...

    def render(self, list[Buffer]) -> float: ...
