// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::{FrameServerPlugin, PluginType, frei0r_rs};

struct Source;

impl PluginType for Source {
    const PLUGIN_TYPE: frei0r_rs::PluginType = frei0r_rs::PluginType::Source;
}

frei0r_rs::plugin!(FrameServerPlugin<Source>);
