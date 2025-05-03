use std::{error::Error, io::Write};

use shared_memory::ShmemConf;

use crate::{
    context::RenderContext,
    messages::{ClientResponse, InitializeClient, RenderFrame, receive_message, send_message},
};

pub struct FrameClient {
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    context: RenderContext,
}

impl FrameClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let initialize_message: InitializeClient = receive_message(&mut stdin)?;
        let shmem = ShmemConf::new()
            .os_id(initialize_message.shmem_id())
            .open()?;
        send_message(ClientResponse::default(), &mut stdout)?;
        stdout.flush()?;
        let context = RenderContext::new(*initialize_message.size(), shmem);

        Ok(FrameClient {
            stdin,
            stdout,
            context,
        })
    }

    pub fn render_prepare(mut self) -> Result<RenderPrepare, Box<dyn Error>> {
        let render_message: RenderFrame = receive_message(&mut self.stdin)?;
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
        self.frame_client.context.frame(frame_num)
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
        self.frame_client.context.rendered_frame_mut()
    }

    pub fn finish(mut self) -> Result<FrameClient, Box<dyn Error>> {
        send_message(ClientResponse::default(), &mut self.frame_client.stdout)?;
        self.frame_client.stdout.flush()?;
        Ok(self.frame_client)
    }
}
