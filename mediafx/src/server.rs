// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

pub use crate::messages::RenderData;
use crate::{
    context::{RenderContext, RenderSize},
    messages::{RenderAck, RenderFrame, RenderInitialize, receive_message, send_message},
};

use shared_memory::ShmemConf;

pub struct MediaFXServer {
    context: RenderContext,
    client: Child,
    client_stdin: ChildStdin,
    client_stdout: ChildStdout,
}

impl MediaFXServer {
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
            RenderInitialize::new(size, shmem.get_os_id().into(), config.into());
        let context = RenderContext::new(size, shmem);

        send_message(&render_initialize, &mut client_stdin)?;
        // XXX check for errors
        let _response: RenderAck = receive_message(&mut client_stdout)?;
        Ok(MediaFXServer {
            context,
            client,
            client_stdin,
            client_stdout,
        })
    }

    pub fn get_source_frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        self.context.frame_mut(frame_num)
    }

    pub fn render(&mut self, render_data: RenderData) -> Result<&mut [u8], Box<dyn Error>> {
        send_message(RenderFrame::Render(render_data), &mut self.client_stdin)?;
        // XXX check for errors
        let _response: RenderAck = receive_message(&mut self.client_stdout)?;
        Ok(self.context.rendered_frame_mut())
    }
}

impl Drop for MediaFXServer {
    fn drop(&mut self) {
        if send_message(RenderFrame::Terminate, &mut self.client_stdin).is_err() {
            let _ = self.client.kill();
        } else {
            let _ = self.client.wait();
        }
    }
}
