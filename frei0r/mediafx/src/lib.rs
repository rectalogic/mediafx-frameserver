use std::{ffi::CString, marker::PhantomData};

use frameserver::server::FrameServer;
pub use frei0r_rs;

//XXX can we define a trait and move these into each plugin, and store this in FrameServerPlugin instead of phantom, so we can invoke it?
pub struct Source;
pub struct Filter;
pub struct Mixer2;
pub struct Mixer3;

#[derive(frei0r_rs::PluginBase)]
pub struct FrameServerPlugin<T> {
    #[frei0r(explain = c"Frameserver client executable path")]
    client_path: CString,
    width: u32,
    height: u32,
    frame_server: Option<frameserver::server::FrameServer>,
    frame_server_initialized: bool,
    _phantom: PhantomData<T>,
}

trait PluginDefault {
    fn info() -> frei0r_rs::PluginInfo;
}

impl<T> FrameServerPlugin<T>
where
    FrameServerPlugin<T>: PluginDefault,
{
    fn frame_server(&mut self) -> Option<&mut frameserver::server::FrameServer> {
        if self.frame_server_initialized {
            return self.frame_server.as_mut();
        }
        self.frame_server_initialized = true;
        match self.frame_server {
            None => {
                let client_path = match self.client_path.to_str() {
                    Ok(client_path) => client_path,
                    Err(e) => {
                        eprintln!("Failed to parse client_path: {}", e);
                        return None;
                    }
                };

                let count: usize = match <FrameServerPlugin<T> as PluginDefault>::info().plugin_type
                {
                    frei0r_rs::PluginType::Source => 0,
                    frei0r_rs::PluginType::Filter => 1,
                    frei0r_rs::PluginType::Mixer2 => 2,
                    frei0r_rs::PluginType::Mixer3 => 3,
                };
                match FrameServer::new(client_path, self.width, self.height, count) {
                    Ok(frame_server) => {
                        self.frame_server = Some(frame_server);
                        self.frame_server.as_mut()
                    }
                    Err(e) => {
                        eprintln!("Failed to create frame server: {}", e);
                        None
                    }
                }
            }
            Some(_) => self.frame_server.as_mut(),
        }
    }
}

fn slice_to_bytes_mut(slice: &mut [u32]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast::<u8>(), size_of_val(slice)) }
}

fn slice_to_bytes(slice: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), size_of_val(slice)) }
}

impl<T> frei0r_rs::Plugin for FrameServerPlugin<T>
where
    FrameServerPlugin<T>: PluginDefault,
{
    fn info() -> frei0r_rs::PluginInfo {
        <FrameServerPlugin<T> as PluginDefault>::info()
    }

    fn new(width: usize, height: usize) -> Self {
        Self {
            width: width as u32,
            height: height as u32,
            client_path: c"".to_owned(),
            frame_server: None,
            frame_server_initialized: false,
            _phantom: PhantomData,
        }
    }

    fn source_update(&mut self, time: f64, outframe: &mut [u32]) {
        if let Some(frame_server) = self.frame_server() {
            let rendered_frame = frame_server.render(time).unwrap();
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
    }

    fn filter_update(&mut self, time: f64, inframe: &[u32], outframe: &mut [u32]) {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe));
            let rendered_frame = frame_server.render(time).unwrap();
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
    }

    fn mixer2_update(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        outframe: &mut [u32],
    ) {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe1));
            frame_server
                .get_source_frame_mut(1)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe2));
            let rendered_frame = frame_server.render(time).unwrap();
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
    }

    fn mixer3_update(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        inframe3: &[u32],
        outframe: &mut [u32],
    ) {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe1));
            frame_server
                .get_source_frame_mut(1)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe2));
            frame_server
                .get_source_frame_mut(2)
                .unwrap()
                .copy_from_slice(slice_to_bytes(inframe3));
            let rendered_frame = frame_server.render(time).unwrap();
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
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

impl PluginDefault for FrameServerPlugin<Source> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Source)
    }
}

impl PluginDefault for FrameServerPlugin<Filter> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Filter)
    }
}

impl PluginDefault for FrameServerPlugin<Mixer2> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Mixer2)
    }
}

impl PluginDefault for FrameServerPlugin<Mixer3> {
    fn info() -> frei0r_rs::PluginInfo {
        plugin_info(frei0r_rs::PluginType::Mixer3)
    }
}
