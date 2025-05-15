// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use frameserver::client::{FrameClient, RenderRequest, RenderSize};
use pyo3::{buffer::PyBuffer, exceptions::PyRuntimeError, prelude::*, types::PyList};

enum State {
    FrameClient(FrameClient),
    RenderRequest(RenderRequest),
}

#[pyclass]
struct MediaFX {
    state: Option<State>,
}

#[pymethods]
impl MediaFX {
    #[new]
    fn new() -> PyResult<Self> {
        let state = match FrameClient::new() {
            Ok(client) => State::FrameClient(client),
            Err(err) => return Err(PyRuntimeError::new_err(err.to_string())),
        };
        Ok(MediaFX { state: Some(state) })
    }

    #[getter]
    fn get_frame_size(&self) -> PyResult<(u32, u32)> {
        let size = self.get_size();
        Ok((size.width(), size.height()))
    }

    #[getter]
    fn get_frame_count(&self) -> PyResult<usize> {
        let size = self.get_size();
        Ok(size.count())
    }

    fn render(&mut self, buffers: Bound<PyList>) -> PyResult<()> {
        let current_state = self
            .state
            .take()
            .ok_or_else(|| PyRuntimeError::new_err("Invalid internal state"))?;
        match current_state {
            State::FrameClient(client) => match client.request_render() {
                Ok(render_request) => {
                    self.state = Some(State::RenderRequest(render_request));
                    for buffer in buffers {
                        let frame: PyBuffer<u8> = PyBuffer::get(&buffer)?;
                        //XXX copy frame slice into buffer
                        println!("Buffer length: {}", frame.len_bytes());
                    }
                    Ok(())
                }
                Err((client, err)) => {
                    self.state = Some(State::FrameClient(client));
                    Err(PyRuntimeError::new_err(err.to_string()))
                }
            },
            State::RenderRequest(render_request) => {
                self.state = Some(State::RenderRequest(render_request));
                Err(PyRuntimeError::new_err("Incorrect state"))
            }
        }
    }
}

impl MediaFX {
    fn get_size(&self) -> RenderSize {
        match self.state {
            Some(State::FrameClient(ref client)) => client.render_size(),
            Some(State::RenderRequest(ref client)) => client.render_size(),
            None => unreachable!(),
        }
    }
}

#[pymodule]
fn mediafx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MediaFX>()?;
    Ok(())
}
