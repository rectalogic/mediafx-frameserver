use bincode::{Decode, Encode};
use shared_memory::Shmem;
use std::{error::Error, ops::Range};

const BYTES_PER_PIXEL: usize = 4;

pub(crate) struct RenderContext {
    size: RenderSize,
    shmem: Shmem,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub(crate) struct RenderSize {
    width: u32,
    height: u32,
    /// Number of source frame images, does not count the rendered frame
    count: usize,
}

impl RenderContext {
    pub fn new(size: RenderSize, shmem: Shmem) -> Self {
        RenderContext { size, shmem }
    }
}

impl RenderSize {
    pub fn new(width: u32, height: u32, count: usize) -> Self {
        RenderSize {
            width,
            height,
            count,
        }
    }

    pub fn memory_size(&self) -> usize {
        (self.width * self.height) as usize * (self.count + 1) * BYTES_PER_PIXEL
    }
}

impl RenderContext {
    pub fn frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        self.check_frame(frame_num)?;
        let range = self.frame_range(frame_num);
        let bytes = unsafe { self.shmem.as_slice() };
        Ok(&bytes[range])
    }

    pub fn frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        self.check_frame(frame_num)?;
        let range = self.frame_range(frame_num);
        let bytes = unsafe { self.shmem.as_slice_mut() };
        Ok(&mut bytes[range])
    }

    pub fn rendered_frame_mut(&mut self) -> &mut [u8] {
        let range = self.frame_range(self.size.count);
        let bytes = unsafe { self.shmem.as_slice_mut() };
        &mut bytes[range]
    }

    fn frame_range(&self, frame_num: usize) -> Range<usize> {
        let frame_size = (self.size.width * self.size.height) as usize * BYTES_PER_PIXEL;
        let frame_offset = frame_num * frame_size;
        frame_offset..frame_offset + frame_size
    }

    fn check_frame(&self, frame_num: usize) -> Result<(), Box<dyn Error>> {
        if frame_num >= self.size.count {
            return Err("frame number out of range".into());
        }
        Ok(())
    }
}
