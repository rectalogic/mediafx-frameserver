// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

use shared_memory::ShmemConf;

use crate::context::RenderContext;
pub use crate::context::{BYTES_PER_PIXEL, RenderSize};
use crate::message;
pub use message::RenderData;

#[derive(Debug)]
pub struct FrameClient {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    context: RenderContext,
    config: String,
}

impl FrameClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let render_initialize: message::RenderInitialize = message::receive(&mut stdin)?;
        let shmem = ShmemConf::new()
            .os_id(render_initialize.shmem_id())
            .open()?;
        message::send(message::RenderAck::default(), &mut stdout)?;
        let context = RenderContext::new(*render_initialize.size(), shmem);

        Ok(FrameClient {
            stdin,
            stdout,
            context,
            config: render_initialize.config().into(),
        })
    }

    pub fn config(&self) -> &str {
        &self.config
    }

    pub fn render_size(&self) -> RenderSize {
        self.context.render_size()
    }

    #[allow(clippy::result_large_err)]
    pub fn render_frame(mut self) -> Result<RenderFrame, (Self, Box<dyn Error>)> {
        match message::receive(&mut self.stdin) {
            Ok(message::RenderFrame::Terminate) => std::process::exit(0),
            Ok(message::RenderFrame::Render(render_data)) => Ok(RenderFrame {
                client: self,
                render_data,
            }),
            Err(err) => Err((self, Box::new(err))),
        }
    }
}

#[derive(Debug)]
pub struct RenderFrame {
    client: FrameClient,
    render_data: message::RenderData,
}

impl RenderFrame {
    pub fn render_data(&self) -> &message::RenderData {
        &self.render_data
    }

    pub fn config(&self) -> &str {
        &self.client.config
    }

    pub fn render_size(&self) -> RenderSize {
        self.client.context.render_size()
    }

    pub fn get_source_frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        self.client.context.frame(frame_num)
    }

    pub fn get_source_frames<const N: usize>(&self) -> Result<[&[u8]; N], Box<dyn Error>> {
        self.client.context.frames()
    }

    pub fn get_rendered_frame_mut(&mut self) -> &mut [u8] {
        self.client.context.rendered_frame_mut()
    }

    #[allow(clippy::type_complexity)]
    pub fn get_frames_with_rendered_frame_mut<const N: usize>(
        &mut self,
    ) -> Result<([&[u8]; N], &mut [u8]), Box<dyn Error>> {
        self.client.context.frames_with_rendered_frame_mut()
    }

    #[allow(clippy::result_large_err)]
    pub fn commit(mut self) -> Result<FrameClient, (Self, Box<dyn Error>)> {
        match message::send(message::RenderAck::default(), &mut self.client.stdout) {
            Ok(_) => Ok(self.client),
            Err(err) => Err((self, err)),
        }
    }
}
