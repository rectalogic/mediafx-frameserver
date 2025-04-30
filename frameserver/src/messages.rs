use serde::{Deserialize, Serialize};

use crate::frame::FrameInfo;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitializeClient {
    frame_info: FrameInfo,
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
    pub fn new(frame_info: FrameInfo, shmem_id: String) -> Self {
        InitializeClient {
            frame_info,
            shmem_id,
        }
    }

    pub fn frame_info(&self) -> &FrameInfo {
        &self.frame_info
    }

    pub fn shmem_id(&self) -> &str {
        &self.shmem_id
    }
}
