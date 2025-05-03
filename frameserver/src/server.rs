use std::{
    error::Error,
    ffi::OsStr,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use shared_memory::ShmemConf;

use crate::{
    context::{RenderContext, RenderSize},
    messages::{RenderAck, RenderFrame, RenderInitialize, receive_message, send_message},
};

pub struct FrameServer {
    context: RenderContext,
    client: Child,
    client_stdin: ChildStdin,
    client_stdout: ChildStdout,
}

impl FrameServer {
    pub fn new<S: AsRef<OsStr>>(
        client_path: S,
        width: u32,
        height: u32,
        count: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let size = RenderSize::new(width, height, count);
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
        let render_initialize = RenderInitialize::new(size, shmem.get_os_id().into());
        let context = RenderContext::new(size, shmem);

        send_message(&render_initialize, &mut client_stdin)?;
        let response: RenderAck = receive_message(&mut client_stdout)?;
        Ok(FrameServer {
            context,
            client,
            client_stdin,
            client_stdout,
        })
    }

    pub fn get_source_frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        self.context.frame_mut(frame_num)
    }

    pub fn render(mut self, time: f32) -> Result<RenderResult, Box<dyn Error>> {
        send_message(RenderFrame::Render(time), &mut self.client_stdin)?;
        let response: RenderAck = receive_message(&mut self.client_stdout)?;
        Ok(RenderResult { frame_server: self })
    }
}

impl Drop for FrameServer {
    fn drop(&mut self) {
        if send_message(RenderFrame::Terminate, &mut self.client_stdin).is_err() {
            let _ = self.client.kill();
        } else {
            let _ = self.client.wait();
        }
    }
}

pub struct RenderResult {
    frame_server: FrameServer,
}

impl RenderResult {
    pub fn get_rendered_frame(&mut self) -> &mut [u8] {
        self.frame_server.context.rendered_frame_mut()
    }

    pub fn finish(self) -> FrameServer {
        self.frame_server
    }
}
