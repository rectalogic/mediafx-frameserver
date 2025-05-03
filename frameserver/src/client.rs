use std::error::Error;

use shared_memory::ShmemConf;

use crate::{
    context::RenderContext,
    messages::{RenderAck, RenderFrame, RenderInitialize, receive_message, send_message},
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
        let render_initialize: RenderInitialize = receive_message(&mut stdin)?;
        let shmem = ShmemConf::new()
            .os_id(render_initialize.shmem_id())
            .open()?;
        send_message(RenderAck::default(), &mut stdout)?;
        let context = RenderContext::new(*render_initialize.size(), shmem);

        Ok(FrameClient {
            stdin,
            stdout,
            context,
        })
    }

    pub fn request_render(mut self) -> Result<RenderRequest, Box<dyn Error>> {
        let render_message: RenderFrame = receive_message(&mut self.stdin)?;
        match render_message {
            RenderFrame::Terminate => std::process::exit(0),
            RenderFrame::Render(time) => Ok(RenderRequest {
                frame_client: self,
                time,
            }),
        }
    }
}

pub struct RenderRequest {
    frame_client: FrameClient,
    time: f32,
}

impl RenderRequest {
    pub fn time(&self) -> f32 {
        self.time
    }

    pub fn get_source_frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        self.frame_client.context.frame(frame_num)
    }

    pub fn get_rendered_frame_mut(&mut self) -> &mut [u8] {
        self.frame_client.context.rendered_frame_mut()
    }

    pub fn render_complete(mut self) -> Result<FrameClient, Box<dyn Error>> {
        send_message(RenderAck::default(), &mut self.frame_client.stdout)?;
        Ok(self.frame_client)
    }
}
