use bincode::{Decode, Encode};
use std::{error::Error, ops::Range};

const BYTES_PER_PIXEL: usize = 4;

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub(crate) struct RenderContext {
    width: u32,
    height: u32,
    /// Number of source frame images, does not count the rendered frame
    count: usize,
}

impl RenderContext {
    pub fn new(width: u32, height: u32, count: usize) -> Self {
        RenderContext {
            width,
            height,
            count,
        }
    }

    pub fn memory_size(&self) -> usize {
        (self.width * self.height) as usize * (self.count + 1) * BYTES_PER_PIXEL
    }

    fn frame_range_unchecked(&self, frame_num: usize) -> Range<usize> {
        let frame_size = (self.width * self.height) as usize * BYTES_PER_PIXEL;
        let frame_offset = frame_num * frame_size;
        frame_offset..frame_offset + frame_size
    }

    pub fn frame_range(&self, frame_num: usize) -> Result<Range<usize>, Box<dyn Error>> {
        if frame_num >= self.count {
            return Err("frame number out of range".into());
        }
        Ok(self.frame_range_unchecked(frame_num))
    }

    pub fn rendered_frame_range(&self) -> Range<usize> {
        self.frame_range_unchecked(self.count)
    }
}
