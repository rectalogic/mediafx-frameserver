// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use frameserver::client::{FrameClient, RenderRequest};
use pyo3::{buffer::PyBuffer, exceptions::PyRuntimeError, prelude::*, types::PyList};

enum State {
    FrameClient(FrameClient),
    RenderRequest(RenderRequest),
    Interim,
}

#[pyclass]
struct MediaFX {
    state: State,
}

#[pymethods]
impl MediaFX {
    #[new]
    fn new() -> PyResult<Self> {
        let state = match FrameClient::new() {
            Ok(client) => State::FrameClient(client),
            Err(err) => return Err(PyRuntimeError::new_err(err.to_string())),
        };
        Ok(MediaFX { state })
    }

    #[getter]
    fn get_frame_size(&self) -> PyResult<(u32, u32)> {
        let size = match self.state {
            State::FrameClient(ref client) => client.render_size(),
            State::RenderRequest(ref client) => client.render_size(),
            State::Interim => unreachable!(),
        };
        Ok((size.width(), size.height()))
    }

    #[getter]
    fn get_frame_count(&self) -> PyResult<usize> {
        let size = match self.state {
            State::FrameClient(ref client) => client.render_size(),
            State::RenderRequest(ref client) => client.render_size(),
            State::Interim => unreachable!(),
        };
        Ok(size.count())
    }

    fn render(&mut self, buffers: Bound<PyList>) -> PyResult<()> {
        let current_state = std::mem::replace(&mut self.state, State::Interim);
        match current_state {
            State::FrameClient(client) => match client.request_render() {
                Ok(render_request) => {
                    self.state = State::RenderRequest(render_request);
                    for buffer in buffers {
                        let frame: PyBuffer<u8> = PyBuffer::get(&buffer)?;
                        //XXX copy frame slice into buffer
                        println!("Buffer length: {}", frame.len_bytes());
                    }
                    Ok(())
                }
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            },
            State::RenderRequest(_) => Err(PyRuntimeError::new_err("Incorrect state")),
            State::Interim => unreachable!(),
        }
    }
}

#[pymodule]
fn mediafx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MediaFX>()?;
    Ok(())
}
