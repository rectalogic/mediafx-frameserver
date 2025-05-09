// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::{FrameServerPlugin, PluginType, frei0r_rs};

struct Filter;

impl PluginType for Filter {
    const PLUGIN_TYPE: frei0r_rs::PluginType = frei0r_rs::PluginType::Filter;
}

frei0r_rs::plugin!(FrameServerPlugin<Filter>);
