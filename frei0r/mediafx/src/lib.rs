// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    ffi::{CStr, CString},
    marker::PhantomData,
};

pub use frei0r_rs;
pub use mediafx_server;

#[derive(frei0r_rs::PluginBase)]
pub struct MediaFXServerPlugin<K: frei0r_rs::PluginKind> {
    #[frei0r(explain = c"Frameserver client executable path")]
    client_path: CString,
    width: u32,
    height: u32,
    frame_count: usize,
    frame_server: Option<mediafx_server::MediaFXServer>,
    frame_server_initialized: bool,
    _phantom: PhantomData<K>,
}

impl<K> MediaFXServerPlugin<K>
where
    K: frei0r_rs::PluginKind,
{
    pub fn new(width: u32, height: u32, frame_count: usize) -> Self {
        Self {
            width,
            height,
            client_path: c"".to_owned(),
            frame_count,
            frame_server: None,
            frame_server_initialized: false,
            _phantom: PhantomData,
        }
    }

    fn frame_server(&mut self) -> Option<&mut mediafx_server::MediaFXServer> {
        if self.frame_server_initialized {
            return self.frame_server.as_mut();
        }
        self.frame_server_initialized = true;
        match self.frame_server {
            None => {
                let client_path = match self.client_path.to_str() {
                    Ok(client_path) => client_path,
                    Err(e) => {
                        debug_assert!(false, "Failed to parse client_path: {}", e);
                        eprintln!("Failed to parse client_path: {}", e);
                        return None;
                    }
                };

                match mediafx_server::MediaFXServer::new(
                    client_path,
                    self.width,
                    self.height,
                    self.frame_count,
                ) {
                    Ok(frame_server) => {
                        self.frame_server = Some(frame_server);
                        self.frame_server.as_mut()
                    }
                    Err(e) => {
                        debug_assert!(false, "Failed to create frame server: {}", e);
                        eprintln!("Failed to create frame server: {}", e);
                        None
                    }
                }
            }
            Some(_) => self.frame_server.as_mut(),
        }
    }

    pub fn source(&mut self, time: f64, outframe: &mut [u32]) -> Result<(), Box<dyn Error>> {
        if let Some(frame_server) = self.frame_server() {
            let rendered_frame = frame_server.render(time)?;
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
        Ok(())
    }

    pub fn filter(
        &mut self,
        time: f64,
        inframe: &[u32],
        outframe: &mut [u32],
    ) -> Result<(), Box<dyn Error>> {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)?
                .copy_from_slice(slice_to_bytes(inframe));
            let rendered_frame = frame_server.render(time)?;
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
        Ok(())
    }

    pub fn mixer2(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        outframe: &mut [u32],
    ) -> Result<(), Box<dyn Error>> {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)?
                .copy_from_slice(slice_to_bytes(inframe1));
            frame_server
                .get_source_frame_mut(1)?
                .copy_from_slice(slice_to_bytes(inframe2));
            let rendered_frame = frame_server.render(time)?;
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
        Ok(())
    }

    pub fn mixer3(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        inframe3: &[u32],
        outframe: &mut [u32],
    ) -> Result<(), Box<dyn Error>> {
        if let Some(frame_server) = self.frame_server() {
            frame_server
                .get_source_frame_mut(0)?
                .copy_from_slice(slice_to_bytes(inframe1));
            frame_server
                .get_source_frame_mut(1)?
                .copy_from_slice(slice_to_bytes(inframe2));
            frame_server
                .get_source_frame_mut(2)?
                .copy_from_slice(slice_to_bytes(inframe3));
            let rendered_frame = frame_server.render(time)?;
            slice_to_bytes_mut(outframe).copy_from_slice(rendered_frame);
        }
        Ok(())
    }
}

fn slice_to_bytes_mut(slice: &mut [u32]) -> &mut [u8] {
    unsafe { std::slice::from_raw_parts_mut(slice.as_mut_ptr().cast::<u8>(), size_of_val(slice)) }
}

fn slice_to_bytes(slice: &[u32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(slice.as_ptr().cast::<u8>(), size_of_val(slice)) }
}

trait ServerType {
    const NAME: &'static CStr;
    const EXPLANATION: &'static CStr;
    const FRAME_COUNT: usize;
}

impl ServerType for frei0r_rs::KindSource {
    const NAME: &'static CStr = c"Frameserver source";
    const EXPLANATION: &'static CStr = c"Handles source plugin clients";
    const FRAME_COUNT: usize = 0;
}

impl ServerType for frei0r_rs::KindFilter {
    const NAME: &'static CStr = c"Frameserver filter";
    const EXPLANATION: &'static CStr = c"Handles filter plugin clients";
    const FRAME_COUNT: usize = 1;
}

impl ServerType for frei0r_rs::KindMixer2 {
    const NAME: &'static CStr = c"Frameserver mixer2";
    const EXPLANATION: &'static CStr = c"Handles mixer2 plugin clients";
    const FRAME_COUNT: usize = 2;
}

impl ServerType for frei0r_rs::KindMixer3 {
    const NAME: &'static CStr = c"Frameserver mixer3";
    const EXPLANATION: &'static CStr = c"Handles mixer3 plugin clients";
    const FRAME_COUNT: usize = 3;
}

impl<K> frei0r_rs::Plugin for MediaFXServerPlugin<K>
where
    K: frei0r_rs::PluginKind + ServerType,
{
    type Kind = K;

    fn info() -> frei0r_rs::PluginInfo {
        frei0r_rs::PluginInfo {
            name: K::NAME,
            author: c"Andrew Wason",
            color_model: frei0r_rs::ColorModel::RGBA8888,
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            explanation: K::EXPLANATION,
        }
    }

    fn new(width: usize, height: usize) -> Self {
        MediaFXServerPlugin::new(width as u32, height as u32, K::FRAME_COUNT)
    }
}

impl frei0r_rs::FilterPlugin for MediaFXServerPlugin<frei0r_rs::KindFilter> {
    fn update_filter(&mut self, time: f64, inframe: &[u32], outframe: &mut [u32]) {
        if let Err(e) = self.filter(time, inframe, outframe) {
            debug_assert!(false, "Failed to filter frame: {}", e);
            eprintln!("Failed to filter frame: {}", e);
            self.frame_server.take();
        }
    }
}

impl frei0r_rs::SourcePlugin for MediaFXServerPlugin<frei0r_rs::KindSource> {
    fn update_source(&mut self, time: f64, outframe: &mut [u32]) {
        if let Err(e) = self.source(time, outframe) {
            debug_assert!(false, "Failed to source frame: {}", e);
            eprintln!("Failed to source frame: {}", e);
            self.frame_server.take();
        }
    }
}

impl frei0r_rs::Mixer2Plugin for MediaFXServerPlugin<frei0r_rs::KindMixer2> {
    fn update_mixer2(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        outframe: &mut [u32],
    ) {
        if let Err(e) = self.mixer2(time, inframe1, inframe2, outframe) {
            debug_assert!(false, "Failed to mixer2 frame: {}", e);
            eprintln!("Failed to mixer2 frame: {}", e);
            self.frame_server.take();
        }
    }
}

impl frei0r_rs::Mixer3Plugin for MediaFXServerPlugin<frei0r_rs::KindMixer3> {
    fn update_mixer3(
        &mut self,
        time: f64,
        inframe1: &[u32],
        inframe2: &[u32],
        inframe3: &[u32],
        outframe: &mut [u32],
    ) {
        if let Err(e) = self.mixer3(time, inframe1, inframe2, inframe3, outframe) {
            debug_assert!(false, "Failed to mixer3 frame: {}", e);
            eprintln!("Failed to mixer3 frame: {}", e);
            self.frame_server.take();
        }
    }
}
