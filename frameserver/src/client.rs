use std::{error::Error, io::Write};

use shared_memory::{Shmem, ShmemConf};

use crate::{
    frames::Frames,
    messages::{ClientResponse, InitializeClient, RenderFrame},
};

pub struct FrameClient {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    frames: Frames,
    shmem: Shmem,
}

impl FrameClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let initialize_message: InitializeClient = ciborium::from_reader(&stdin)?;
        let shmem = ShmemConf::new()
            .os_id(initialize_message.shmem_id())
            .open()?;
        ciborium::into_writer(&ClientResponse, &stdout)?;
        stdout.flush()?;

        Ok(FrameClient {
            stdin,
            stdout,
            frames: *initialize_message.frames(),
            shmem,
        })
    }

    pub fn render_prepare(self) -> Result<RenderPrepare, Box<dyn Error>> {
        let render_message: RenderFrame = ciborium::from_reader(&self.stdin)?;
        Ok(RenderPrepare {
            frame_client: self,
            time: render_message.time,
        })
    }
}

pub struct RenderPrepare {
    frame_client: FrameClient,
    time: f32,
}

impl RenderPrepare {
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn get_source_frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        let range = self.frame_client.frames.frame_range(frame_num)?;
        let bytes = unsafe { self.frame_client.shmem.as_slice() };
        Ok(&bytes[range])
    }

    pub fn render(self) -> RenderResult {
        RenderResult {
            frame_client: self.frame_client,
        }
    }
}

pub struct RenderResult {
    frame_client: FrameClient,
}

impl RenderResult {
    pub fn get_rendered_frame_mut(&mut self) -> &mut [u8] {
        let bytes = unsafe { self.frame_client.shmem.as_slice_mut() };
        &mut bytes[self.frame_client.frames.rendered_frame_range()]
    }

    pub fn finish(mut self) -> Result<FrameClient, Box<dyn Error>> {
        ciborium::into_writer(&ClientResponse, &self.frame_client.stdout)?;
        self.frame_client.stdout.flush()?;
        Ok(self.frame_client)
    }
}
