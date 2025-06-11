// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    ffi::{CStr, CString},
    marker::PhantomData,
};

pub use frei0r_rs2;
use mediafx::server::RenderData;

pub struct FrameServerPlugin<K: frei0r_rs2::PluginKind> {
    client_path: CString,
    config: CString,
    param1: f64,
    param2: f64,
    param3: f64,
    width: u32,
    height: u32,
    frame_count: usize,
    frame_server: Option<mediafx::server::FrameServer>,
    frame_server_initialized: bool,
    _phantom: PhantomData<K>,
}

impl<K> FrameServerPlugin<K>
where
    K: frei0r_rs2::PluginKind,
{
    pub fn new(width: u32, height: u32, frame_count: usize) -> Self {
        Self {
            width,
            height,
            client_path: c"".to_owned(),
            config: c"".to_owned(),
            param1: 0.,
            param2: 0.,
            param3: 0.,
            frame_count,
            frame_server: None,
            frame_server_initialized: false,
            _phantom: PhantomData,
        }
    }

    fn params(&self, time: f64) -> RenderData {
        (time, self.param1, self.param2, self.param3)
    }

    fn frame_server(&mut self) -> Option<&mut mediafx::server::FrameServer> {
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
                let client_config = match self.config.to_str() {
                    Ok(client_config) => client_config,
                    Err(e) => {
                        debug_assert!(false, "Failed to parse client_config: {}", e);
                        eprintln!("Failed to parse client_config: {}", e);
                        return None;
                    }
                };
                match mediafx::server::FrameServer::new(
                    client_path,
                    client_config,
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
        let params = self.params(time);
        if let Some(frame_server) = self.frame_server() {
            let rendered_frame = frame_server.render(params)?;
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
        let params = self.params(time);
        if let Some(frame_server) = self.frame_server() {
            frame_server.get_source_frames_mut::<1>()?[0].copy_from_slice(slice_to_bytes(inframe));
            let rendered_frame = frame_server.render(params)?;
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
        let params = self.params(time);
        if let Some(frame_server) = self.frame_server() {
            let source_frames = frame_server.get_source_frames_mut::<2>()?;
            source_frames[0].copy_from_slice(slice_to_bytes(inframe1));
            source_frames[1].copy_from_slice(slice_to_bytes(inframe2));
            let rendered_frame = frame_server.render(params)?;
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
        let params = self.params(time);
        if let Some(frame_server) = self.frame_server() {
            let source_frames = frame_server.get_source_frames_mut::<3>()?;
            source_frames[0].copy_from_slice(slice_to_bytes(inframe1));
            source_frames[1].copy_from_slice(slice_to_bytes(inframe2));
            source_frames[2].copy_from_slice(slice_to_bytes(inframe3));
            let rendered_frame = frame_server.render(params)?;
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

impl ServerType for frei0r_rs2::KindSource {
    const NAME: &'static CStr = c"Frameserver source";
    const EXPLANATION: &'static CStr = c"Handles source plugin clients";
    const FRAME_COUNT: usize = 0;
}

impl ServerType for frei0r_rs2::KindFilter {
    const NAME: &'static CStr = c"Frameserver filter";
    const EXPLANATION: &'static CStr = c"Handles filter plugin clients";
    const FRAME_COUNT: usize = 1;
}

impl ServerType for frei0r_rs2::KindMixer2 {
    const NAME: &'static CStr = c"Frameserver mixer2";
    const EXPLANATION: &'static CStr = c"Handles mixer2 plugin clients";
    const FRAME_COUNT: usize = 2;
}

impl ServerType for frei0r_rs2::KindMixer3 {
    const NAME: &'static CStr = c"Frameserver mixer3";
    const EXPLANATION: &'static CStr = c"Handles mixer3 plugin clients";
    const FRAME_COUNT: usize = 3;
}

impl<K> frei0r_rs2::Plugin for FrameServerPlugin<K>
where
    K: frei0r_rs2::PluginKind + ServerType + 'static,
{
    type Kind = K;

    const PARAMS: &'static [frei0r_rs2::ParamInfo<Self>] = &[
        frei0r_rs2::ParamInfo::new_string(
            c"client_path",
            c"Frameserver client executable path",
            |plugin| plugin.client_path.as_c_str(),
            |plugin, value| plugin.client_path = value.to_owned(),
        ),
        frei0r_rs2::ParamInfo::new_string(
            c"config",
            c"Frameserver client configuration data",
            |plugin| plugin.config.as_c_str(),
            |plugin, value| plugin.config = value.to_owned(),
        ),
        frei0r_rs2::ParamInfo::new_double(
            c"param1",
            c"Frameserver client specific parameter 1",
            |plugin| plugin.param1,
            |plugin, value| plugin.param1 = value,
        ),
        frei0r_rs2::ParamInfo::new_double(
            c"param2",
            c"Frameserver client specific parameter 2",
            |plugin| plugin.param2,
            |plugin, value| plugin.param2 = value,
        ),
        frei0r_rs2::ParamInfo::new_double(
            c"param3",
            c"Frameserver client specific parameter 3",
            |plugin| plugin.param3,
            |plugin, value| plugin.param3 = value,
        ),
    ];

    fn info() -> frei0r_rs2::PluginInfo {
        frei0r_rs2::PluginInfo {
            name: K::NAME,
            author: c"Andrew Wason",
            color_model: frei0r_rs2::ColorModel::RGBA8888,
            major_version: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
            minor_version: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
            explanation: Some(K::EXPLANATION),
        }
    }

    fn new(width: usize, height: usize) -> Self {
        FrameServerPlugin::new(width as u32, height as u32, K::FRAME_COUNT)
    }
}

impl frei0r_rs2::FilterPlugin for FrameServerPlugin<frei0r_rs2::KindFilter> {
    fn update_filter(&mut self, time: f64, inframe: &[u32], outframe: &mut [u32]) {
        if let Err(e) = self.filter(time, inframe, outframe) {
            debug_assert!(false, "Failed to filter frame: {}", e);
            eprintln!("Failed to filter frame: {}", e);
            self.frame_server.take();
        }
    }
}

impl frei0r_rs2::SourcePlugin for FrameServerPlugin<frei0r_rs2::KindSource> {
    fn update_source(&mut self, time: f64, outframe: &mut [u32]) {
        if let Err(e) = self.source(time, outframe) {
            debug_assert!(false, "Failed to source frame: {}", e);
            eprintln!("Failed to source frame: {}", e);
            self.frame_server.take();
        }
    }
}

impl frei0r_rs2::Mixer2Plugin for FrameServerPlugin<frei0r_rs2::KindMixer2> {
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

impl frei0r_rs2::Mixer3Plugin for FrameServerPlugin<frei0r_rs2::KindMixer3> {
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
