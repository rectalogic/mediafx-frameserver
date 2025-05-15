// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use bincode::{Decode, Encode};
use shared_memory::Shmem;
use std::{error::Error, ops::Range};

const BYTES_PER_PIXEL: usize = 4;

pub(crate) struct RenderContext {
    size: RenderSize,
    shmem: Shmem,
}

#[derive(Debug, Clone, Copy, Encode, Decode)]
pub struct RenderSize {
    width: u32,
    height: u32,
    /// Number of source frame images, does not count the rendered frame
    count: usize,
}

impl std::fmt::Debug for RenderContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RenderContext")
            .field("size", &self.size)
            .finish()
    }
}

impl RenderContext {
    pub fn new(size: RenderSize, shmem: Shmem) -> Self {
        RenderContext { size, shmem }
    }

    pub fn render_size(&self) -> RenderSize {
        self.size
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

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn memory_size(&self) -> usize {
        (self.width * self.height) as usize * (self.count + 1) * BYTES_PER_PIXEL
    }
}

// Safety: borrow checker shuld enforce mutable references on RenderContext access to Shmem
unsafe impl Sync for RenderContext {}
// Safety: Shmem is only accessed in RenderContext
unsafe impl Send for RenderContext {}

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
