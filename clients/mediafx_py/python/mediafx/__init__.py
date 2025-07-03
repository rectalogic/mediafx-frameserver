# Copyright (C) 2025 Andrew Wason
# SPDX-License-Identifier: GPL-3.0-or-later

import sys

from ._mediafx import MediaFX as MediaFX

# Redirect stdout so it doesn't interfere with the anonymous pipes MediaFX uses
sys.stdout = sys.stderr
