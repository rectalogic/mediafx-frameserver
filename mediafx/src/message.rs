// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    io::{Read, Write},
};

use bincode::{Decode, Encode};

use crate::context::RenderSize;

#[derive(Encode, Decode, Debug)]
pub(super) struct RenderInitialize {
    size: RenderSize,
    shmem_id: String,
    config: String,
}

#[derive(Encode, Decode, Debug, Default)]
pub(super) struct RenderAck {
    error: Option<String>,
}

pub type RenderData = (f64, f64, f64, f64);

#[derive(Encode, Decode, Debug)]
pub(super) enum RenderFrame {
    Render(RenderData),
    Terminate,
}

impl RenderInitialize {
    pub(super) fn new(size: RenderSize, shmem_id: String, config: String) -> Self {
        RenderInitialize {
            size,
            shmem_id,
            config,
        }
    }

    pub(super) fn size(&self) -> &RenderSize {
        &self.size
    }

    pub(super) fn shmem_id(&self) -> &str {
        &self.shmem_id
    }

    pub(super) fn config(&self) -> &str {
        &self.config
    }
}

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

pub(super) fn send<E: Encode, W: Write>(
    message: E,
    writer: &mut W,
) -> Result<usize, Box<dyn Error>> {
    let result = bincode::encode_into_std_write(message, writer, BINCODE_CONFIG)?;
    writer.flush()?;
    Ok(result)
}

pub(super) fn receive<D: Decode<()>, R: Read>(
    reader: &mut R,
) -> Result<D, bincode::error::DecodeError> {
    bincode::decode_from_std_read(reader, BINCODE_CONFIG)
}
