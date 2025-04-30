use serde::{Deserialize, Serialize};
use std::{error::Error, ops::Range};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub(crate) struct FrameInfo {
    width: u32,
    height: u32,
    count: usize,
}

impl FrameInfo {
    pub fn new(width: u32, height: u32, count: usize) -> Self {
        FrameInfo {
            width,
            height,
            count,
        }
    }

    pub fn memory_size(&self) -> usize {
        (self.width * self.height) as usize * self.count
    }

    pub fn frame_range(&self, frame_num: usize) -> Result<Range<usize>, Box<dyn Error>> {
        if frame_num >= self.count {
            return Err("frame number out of range".into());
        }

        let frame_size = (self.width * self.height) as usize;
        let frame_offset = frame_num * frame_size;
        Ok(frame_offset..frame_offset + frame_size)
    }
}
