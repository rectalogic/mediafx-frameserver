// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use crate::context::{RenderContext, RenderSize};
use crate::message;
pub use message::RenderData;
use shared_memory::ShmemConf;

pub struct FrameServer {
    context: RenderContext,
    client: Child,
    client_stdin: ChildStdin,
    client_stdout: ChildStdout,
}

impl FrameServer {
    pub fn new<C: Into<String>>(
        client_path: &str,
        config: C,
        width: u32,
        height: u32,
        count: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let size = RenderSize::new(width, height, count);
        // XXX whitelist programs, support args?
        let mut client = Command::new(client_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let mut client_stdin = client
            .stdin
            .take()
            .ok_or("frame client stdin is not available")?;
        let mut client_stdout = client
            .stdout
            .take()
            .ok_or("frame client stdout is not available")?;
        let shmem = ShmemConf::new().size(size.memory_size()).create()?;
        let render_initialize =
            message::RenderInitialize::new(size, shmem.get_os_id().into(), config.into());
        let context = RenderContext::new(size, shmem);

        message::send(&render_initialize, &mut client_stdin)?;
        // XXX check for errors
        let _response: message::RenderAck = message::receive(&mut client_stdout)?;
        Ok(FrameServer {
            context,
            client,
            client_stdin,
            client_stdout,
        })
    }

    pub fn get_source_frames_mut<const N: usize>(
        &mut self,
    ) -> Result<[&mut [u8]; N], Box<dyn Error>> {
        self.context.frames_mut()
    }

    pub fn render(
        &mut self,
        render_data: message::RenderData,
    ) -> Result<&mut [u8], Box<dyn Error>> {
        message::send(
            message::RenderFrame::Render(render_data),
            &mut self.client_stdin,
        )?;
        // XXX check for errors
        let _response: message::RenderAck = message::receive(&mut self.client_stdout)?;
        Ok(self.context.rendered_frame_mut())
    }
}

impl Drop for FrameServer {
    fn drop(&mut self) {
        if message::send(message::RenderFrame::Terminate, &mut self.client_stdin).is_err() {
            let _ = self.client.kill();
        } else {
            let _ = self.client.wait();
        }
    }
}
