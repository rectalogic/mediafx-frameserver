use std::{
    error::Error,
    io::{Read, Write},
};

use bincode::{Decode, Encode};

use crate::context::RenderSize;

#[derive(Encode, Decode, Debug)]
pub struct InitializeClient {
    size: RenderSize,
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
    pub fn new(size: RenderSize, shmem_id: String) -> Self {
        InitializeClient { size, shmem_id }
    }

    pub fn size(&self) -> &RenderSize {
        &self.size
    }

    pub fn shmem_id(&self) -> &str {
        &self.shmem_id
    }
}

const CONFIG: bincode::config::Configuration = bincode::config::standard();

pub(crate) fn send_message<E: Encode, W: Write>(
    message: E,
    writer: &mut W,
) -> Result<usize, Box<dyn Error>> {
    let result = bincode::encode_into_std_write(message, writer, CONFIG)?;
    writer.flush()?;
    Ok(result)
}

pub(crate) fn receive_message<D: Decode<()>, R: Read>(
    reader: &mut R,
) -> Result<D, bincode::error::DecodeError> {
    bincode::decode_from_std_read(reader, CONFIG)
}
