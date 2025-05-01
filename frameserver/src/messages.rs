use serde::{Deserialize, Serialize};

use crate::frames::Frames;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeClient {
    frames: Frames,
    shmem_id: String,
}

//XXX add optional error message?
#[derive(Serialize, Deserialize, Debug)]
pub struct ClientResponse;

#[derive(Serialize, Deserialize, Debug)]
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
