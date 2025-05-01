use std::error::Error;

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
        let stdout = std::io::stdout();
        let initialize_message: InitializeClient = serde_cbor::from_reader(&stdin)?;
        let shmem = ShmemConf::new()
            .os_id(initialize_message.shmem_id())
            .open()?;
        serde_cbor::to_writer(&stdout, &ClientResponse)?;

        Ok(FrameClient {
            stdin,
            stdout,
            frames: *initialize_message.frames(),
            shmem,
        })
    }

    //XXX use typestate so we can't access frames until render is called - similar in server
    pub fn render(&self) -> Result<(), Box<dyn Error>> {
        let render_message: RenderFrame = serde_cbor::from_reader(&self.stdin)?;
    }
}
