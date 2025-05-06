// use frameserver::server;
use std::{ffi::CString, marker::PhantomData};

pub use frei0r_rs;

pub struct Source;
pub struct Filter;
pub struct Mixer2;
pub struct Mixer3;

#[derive(frei0r_rs::PluginBase)]
pub struct FrameServerPlugin<T> {
    #[frei0r(explain = c"Frameserver client executable path")]
    client_path: CString,
    #[internal]
    width: u32,
    #[internal]
    height: u32,
    #[internal]
    frame_server: Option<frameserver::server::FrameServer>,
    #[internal]
    _phantom: PhantomData<T>,
}

mod plugin {
    use super::*;

    pub(super) trait PluginDefault {
        fn info() -> frei0r_rs::PluginInfo;
    }

    impl<T> frei0r_rs::Plugin for super::FrameServerPlugin<T>
    where
        super::FrameServerPlugin<T>: PluginDefault,
    {
        fn info() -> frei0r_rs::PluginInfo {
            <super::FrameServerPlugin<T> as PluginDefault>::info()
        }

        fn new(width: usize, height: usize) -> Self {
            Self {
                width: width as u32,
                height: height as u32,
                client_path: c"".to_owned(),
                frame_server: None,
                _phantom: PhantomData,
            }
        }

        fn update(
            &self,
            _time: f64,
            _width: usize,
            _height: usize,
            _inframe: &[u32],
            _outframe: &mut [u32],
        ) {
        }

        fn update2(
            &self,
            _: f64,
            _width: usize,
            _height: usize,
            _inframe1: &[u32],
            _inframe2: &[u32],
            _inframe3: &[u32],
            _outframe: &mut [u32],
        ) {
            unreachable!()
        }
    }
}

fn plugin_info(plugin_type: frei0r_rs::PluginType) -> frei0r_rs::PluginInfo {
    let (name, explanation) = match plugin_type {
        frei0r_rs::PluginType::Source => (c"Frameserver source", c"Handles source plugin clients"),
        frei0r_rs::PluginType::Filter => (c"Frameserver filter", c"Handles filter plugin clients"),
        frei0r_rs::PluginType::Mixer2 => (c"Frameserver mixer2", c"Handles mixer2 plugin clients"),
        frei0r_rs::PluginType::Mixer3 => (c"Frameserver mixer3", c"Handles mixer3 plugin clients"),
    };
    frei0r_rs::PluginInfo {
        name,
        author: c"Andrew Wason",
        plugin_type,
        color_model: frei0r_rs::ColorModel::RGBA8888,
        major_version: 1,
        minor_version: 0,
        explanation,
    }
}

impl plugin::PluginDefault for FrameServerPlugin<Source> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Source)
    }
}

impl plugin::PluginDefault for FrameServerPlugin<Filter> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Filter)
    }
}

impl plugin::PluginDefault for FrameServerPlugin<Mixer2> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Mixer2)
    }
}

impl plugin::PluginDefault for FrameServerPlugin<Mixer3> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Mixer3)
    }
}
