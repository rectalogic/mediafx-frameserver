// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use client_state::ClientState;
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(js_name = "MediaFX")]
pub struct MediaFX {
    state: ClientState,
}

#[napi]
impl MediaFX {
    #[napi(constructor)]
    pub fn new() -> napi::Result<Self> {
        Ok(MediaFX {
            state: ClientState::new().map_err(|err| napi::Error::from_reason(err.to_string()))?,
        })
    }

    #[napi(getter)]
    pub fn config(&self) -> &str {
        self.state.config()
    }

    #[napi(getter)]
    pub fn frame_bytecount(&self) -> u32 {
        self.state.frame_bytecount() as u32
    }

    #[napi(getter)]
    pub fn frame_size(&self) -> [u32; 2] {
        self.state.frame_size().into()
    }

    #[napi(getter)]
    pub fn frame_count(&self) -> u32 {
        self.state.frame_count() as u32
    }

    #[napi]
    pub fn render_frame(&mut self, frames: Option<Vec<Uint8Array>>) -> napi::Result<[f64; 4]> {
        let result = self.state.render_frame(frames, |render_frame, mut frames| {
            for (frame_num, frame) in frames.iter_mut().enumerate() {
                let source = render_frame.get_source_frame(frame_num)?;
                check_len(source, &frame)?;
                unsafe { frame.as_mut().copy_from_slice(source) };
            }
            Ok(())
        });
        match result {
            Ok(render_data) => Ok(render_data.into()),
            Err(err) => Err(napi::Error::from_reason(err.to_string())),
        }
    }

    #[napi]
    pub fn render_commit(&mut self, frame: Uint8Array) -> napi::Result<()> {
        self.state
            .render_commit(&frame, |frame, rendered_frame| {
                check_len(&rendered_frame, frame)?;
                rendered_frame.copy_from_slice(frame);
                Ok(())
            })
            .map_err(|err| napi::Error::from_reason(err.to_string()))
    }
}

fn check_len<S, D>(source: S, dest: D) -> std::result::Result<(), Box<dyn std::error::Error>>
where
    S: AsRef<[u8]>,
    D: AsRef<[u8]>,
{
    let source_len = source.as_ref().len();
    let dest_len = dest.as_ref().len();
    if source_len != dest_len {
        return Err(format!("Frame lengths do not match, {source_len} != {dest_len}").into());
    }
    Ok(())
}
