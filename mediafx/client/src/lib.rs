// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

use shared_memory::ShmemConf;

pub use mediafx_common::context::{BYTES_PER_PIXEL, RenderSize};
pub use mediafx_common::messages::RenderData;
use mediafx_common::{
    context::RenderContext,
    messages::{RenderAck, RenderFrame, RenderInitialize, receive_message, send_message},
};

#[derive(Debug)]
pub struct MediaFXClient {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    context: RenderContext,
    config: String,
}

impl MediaFXClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let render_initialize: RenderInitialize = receive_message(&mut stdin)?;
        let shmem = ShmemConf::new()
            .os_id(render_initialize.shmem_id())
            .open()?;
        send_message(RenderAck::default(), &mut stdout)?;
        let context = RenderContext::new(*render_initialize.size(), shmem);

        Ok(MediaFXClient {
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
    pub fn request_render(mut self) -> Result<RenderRequest, (Self, Box<dyn Error>)> {
        match receive_message(&mut self.stdin) {
            Ok(RenderFrame::Terminate) => std::process::exit(0),
            Ok(RenderFrame::Render(render_data)) => Ok(RenderRequest {
                frame_client: self,
                render_data,
            }),
            Err(err) => Err((self, Box::new(err))),
        }
    }
}

#[derive(Debug)]
pub struct RenderRequest {
    frame_client: MediaFXClient,
    render_data: RenderData,
}

impl RenderRequest {
    pub fn render_data(&self) -> &RenderData {
        &self.render_data
    }

    pub fn config(&self) -> &str {
        &self.frame_client.config
    }

    pub fn render_size(&self) -> RenderSize {
        self.frame_client.context.render_size()
    }

    pub fn get_source_frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        self.frame_client.context.frame(frame_num)
    }

    pub fn get_rendered_frame_mut(&mut self) -> &mut [u8] {
        self.frame_client.context.rendered_frame_mut()
    }

    #[allow(clippy::result_large_err)]
    pub fn render_complete(mut self) -> Result<MediaFXClient, (Self, Box<dyn Error>)> {
        match send_message(RenderAck::default(), &mut self.frame_client.stdout) {
            Ok(_) => Ok(self.frame_client),
            Err(err) => Err((self, err)),
        }
    }
}
