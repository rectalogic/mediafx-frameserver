use mediafx::{FrameServerPlugin, PluginType, frei0r_rs};

struct Mixer3;

impl PluginType for Mixer3 {
    const PLUGIN_TYPE: frei0r_rs::PluginType = frei0r_rs::PluginType::Mixer3;
}

frei0r_rs::plugin!(FrameServerPlugin<Mixer3>);
