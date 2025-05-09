// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::{FrameServerPlugin, PluginType, frei0r_rs};

struct Mixer2;

impl PluginType for Mixer2 {
    const PLUGIN_TYPE: frei0r_rs::PluginType = frei0r_rs::PluginType::Mixer2;
}

frei0r_rs::plugin!(FrameServerPlugin<Mixer2>);
