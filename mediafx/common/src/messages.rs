// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    io::{Read, Write},
};

use bincode::{Decode, Encode};

use crate::context::RenderSize;

#[derive(Encode, Decode, Debug)]
pub struct RenderInitialize {
    size: RenderSize,
    shmem_id: String,
    config: String,
}

#[derive(Encode, Decode, Debug, Default)]
pub struct RenderAck {
    error: Option<String>,
}

pub type RenderData = (f64, f64, f64, f64);

#[derive(Encode, Decode, Debug)]
pub enum RenderFrame {
    Render(RenderData),
    Terminate,
}

impl RenderInitialize {
    pub fn new(size: RenderSize, shmem_id: String, config: String) -> Self {
        RenderInitialize {
            size,
            shmem_id,
            config,
        }
    }

    pub fn size(&self) -> &RenderSize {
        &self.size
    }

    pub fn shmem_id(&self) -> &str {
        &self.shmem_id
    }

    pub fn config(&self) -> &str {
        &self.config
    }
}

const BINCODE_CONFIG: bincode::config::Configuration = bincode::config::standard();

pub fn send_message<E: Encode, W: Write>(
    message: E,
    writer: &mut W,
) -> Result<usize, Box<dyn Error>> {
    let result = bincode::encode_into_std_write(message, writer, BINCODE_CONFIG)?;
    writer.flush()?;
    Ok(result)
}

pub fn receive_message<D: Decode<()>, R: Read>(
    reader: &mut R,
) -> Result<D, bincode::error::DecodeError> {
    bincode::decode_from_std_read(reader, BINCODE_CONFIG)
}
