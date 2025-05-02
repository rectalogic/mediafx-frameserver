use bincode::{Decode, Encode};

use crate::frames::Frames;

#[derive(Encode, Decode, Debug)]
pub struct InitializeClient {
    frames: Frames,
    shmem_id: String,
}

#[derive(Encode, Decode, Debug, Default)]
pub struct ClientResponse {
    error: Option<String>,
}

#[derive(Encode, Decode, Debug)]
pub struct RenderFrame {
    pub time: f32,
}

impl InitializeClient {
    pub fn new(frame_info: Frames, shmem_id: String) -> Self {
        InitializeClient {
            frames: frame_info,
            shmem_id,
        }
    }

    pub fn frames(&self) -> &Frames {
        &self.frames
    }

    pub fn shmem_id(&self) -> &str {
        &self.shmem_id
    }
}
