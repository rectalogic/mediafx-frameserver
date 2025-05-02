use std::{
    error::Error,
    ffi::OsStr,
    io::Write,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use shared_memory::{Shmem, ShmemConf};

use crate::{
    frames::Frames,
    messages::{ClientResponse, InitializeClient, RenderFrame, receive_message, send_message},
};

pub struct FrameServer {
    frames: Frames,
    client: Child,
    client_stdin: ChildStdin,
    client_stdout: ChildStdout,
    shmem: Shmem,
}

impl FrameServer {
    pub fn new<S: AsRef<OsStr>>(
        client_path: S,
        width: u32,
        height: u32,
        count: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let frames = Frames::new(width, height, count);
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
        let shmem = ShmemConf::new().size(frames.memory_size()).create()?;
        let initialize_message = InitializeClient::new(frames, shmem.get_os_id().into());

        send_message(&initialize_message, &mut client_stdin)?;
        client_stdin.flush()?;
        let response: ClientResponse = receive_message(&mut client_stdout)?;
        Ok(FrameServer {
            frames,
            client,
            client_stdin,
            client_stdout,
            shmem,
        })
    }

    pub fn get_source_frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        let range = self.frames.frame_range(frame_num)?;
        let bytes = unsafe { self.shmem.as_slice_mut() };
        Ok(&mut bytes[range])
    }

    pub fn render(mut self, time: f32) -> Result<RenderResult, Box<dyn Error>> {
        send_message(&RenderFrame { time }, &mut self.client_stdin)?;
        self.client_stdin.flush()?;
        let response: ClientResponse = receive_message(&mut self.client_stdout)?;
        Ok(RenderResult { frame_server: self })
    }
}

impl Drop for FrameServer {
    fn drop(&mut self) {
        // Drop/close stdin
        self.client.stdin.take();
        self.client.wait().ok();
    }
}

pub struct RenderResult {
    frame_server: FrameServer,
}

impl RenderResult {
    pub fn get_rendered_frame(&mut self) -> &mut [u8] {
        let bytes = unsafe { self.frame_server.shmem.as_slice_mut() };
        &mut bytes[self.frame_server.frames.rendered_frame_range()]
    }

    pub fn finish(self) -> FrameServer {
        self.frame_server
    }
}
