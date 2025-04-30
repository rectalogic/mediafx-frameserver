use std::{
    error::Error,
    ffi::OsStr,
    process::{Child, ChildStdin, ChildStdout, Command, Stdio},
};

use shared_memory::{Shmem, ShmemConf};

use crate::{
    frame::FrameInfo,
    messages::{ClientResponse, InitializeClient, RenderFrame},
};

pub struct FrameServer {
    frame_info: FrameInfo,
    client: Child,
    client_stdin: ChildStdin,
    client_stdout: ChildStdout,
    shmem: Shmem,
}

impl FrameServer {
    pub fn new<S: AsRef<OsStr>>(
        client_path: S,
        frame_info: FrameInfo,
    ) -> Result<Self, Box<dyn Error>> {
        let mut client = Command::new(client_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;
        let client_stdin = client
            .stdin
            .take()
            .ok_or("frame client stdin is not available")?;
        let mut client_stdout = client
            .stdout
            .take()
            .ok_or("frame client stdout is not available")?;
        let shmem = ShmemConf::new().size(frame_info.memory_size()).create()?;
        let initialize_message = InitializeClient::new(frame_info, shmem.get_os_id().into());

        serde_cbor::to_writer(&client_stdin, &initialize_message)?;
        let response: ClientResponse = serde_cbor::from_reader(&mut client_stdout)?;

        Ok(FrameServer {
            frame_info,
            client,
            client_stdin,
            client_stdout,
            shmem,
        })
    }

    pub fn get_frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        let range = self.frame_info.frame_range(frame_num)?;
        let mem = unsafe { self.shmem.as_slice_mut() };
        Ok(&mut mem[range])
    }

    pub fn render(&mut self, time: f32) -> Result<&mut [u8], Box<dyn Error>> {
        serde_cbor::to_writer(&self.client_stdin, &RenderFrame { time })?;
        let response: ClientResponse = serde_cbor::from_reader(&mut self.client_stdout)?;
        let mem = unsafe { self.shmem.as_slice_mut() };
        Ok(&mut mem[self.frame_info.frame_range(0)?])
    }
}

impl Drop for FrameServer {
    fn drop(&mut self) {
        // Drop/close stdin
        self.client.stdin.take();
        self.client.wait().ok();
    }
}
