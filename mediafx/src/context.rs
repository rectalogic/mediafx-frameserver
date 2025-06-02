// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use bincode::{Decode, Encode};
use shared_memory::Shmem;
use std::{array, error::Error, ops::Range};

pub const BYTES_PER_PIXEL: usize = 4;

pub(super) struct RenderContext {
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
    pub(super) fn new(size: RenderSize, shmem: Shmem) -> Self {
        RenderContext { size, shmem }
    }

    pub(super) fn render_size(&self) -> RenderSize {
        self.size
    }
}

impl RenderSize {
    pub(super) fn new(width: u32, height: u32, count: usize) -> Self {
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

fn split<const N: usize>(bytes: &[u8], bytecount: usize) -> [&[u8]; N] {
    array::from_fn::<&[u8], N, _>(|i| &bytes[(i * bytecount)..((i + 1) * bytecount)])
}

fn split_mut<const N: usize>(bytes: &mut [u8], bytecount: usize) -> [&mut [u8]; N] {
    let indices =
        array::from_fn::<std::ops::Range<usize>, N, _>(|i| (i * bytecount)..((i + 1) * bytecount));

    bytes.get_disjoint_mut(indices).unwrap()
}

impl RenderContext {
    fn get_bytes(&self) -> &[u8] {
        unsafe { self.shmem.as_slice() }
    }

    fn get_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { self.shmem.as_slice_mut() }
    }

    pub(super) fn frames<const N: usize>(&self) -> Result<[&[u8]; N], Box<dyn Error>> {
        if N > self.size.count {
            return Err("frame number out of range".into());
        }
        let bytes = self.get_bytes();
        let bytecount = self.frame_bytecount();
        Ok(split(bytes, bytecount))
    }

    pub(super) fn frames_mut<const N: usize>(&mut self) -> Result<[&mut [u8]; N], Box<dyn Error>> {
        if N > self.size.count {
            return Err("frame number out of range".into());
        }
        let bytecount = self.frame_bytecount();
        let bytes = self.get_bytes_mut();
        Ok(split_mut(bytes, bytecount))
    }

    pub(super) fn frame(&self, frame_num: usize) -> Result<&[u8], Box<dyn Error>> {
        self.check_frame(frame_num)?;
        let range = self.frame_range(frame_num);
        let bytes = self.get_bytes();
        Ok(&bytes[range])
    }

    pub(super) fn rendered_frame_mut(&mut self) -> &mut [u8] {
        let range = self.frame_range(self.size.count);
        let bytes = self.get_bytes_mut();
        &mut bytes[range]
    }

    #[allow(clippy::type_complexity)]
    pub(super) fn frames_with_rendered_frame_mut<const N: usize>(
        &mut self,
    ) -> Result<([&[u8]; N], &mut [u8]), Box<dyn Error>> {
        if N > self.size.count {
            return Err("frame number out of range".into());
        }
        let bytecount = self.frame_bytecount();
        let bytes = self.get_bytes_mut();
        let (frames, rendered_frame) = bytes.split_at_mut(bytes.len() - bytecount);
        Ok((split(frames, bytecount), rendered_frame))
    }

    fn frame_bytecount(&self) -> usize {
        (self.size.width * self.size.height) as usize * BYTES_PER_PIXEL
    }

    fn frame_range(&self, frame_num: usize) -> Range<usize> {
        let frame_size = self.frame_bytecount();
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
