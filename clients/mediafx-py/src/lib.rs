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
                    self.copy_source_frames(&render_request, buffers)?;
                    self.state = Some(State::RenderRequest(render_request));
                    Ok(())
                }
                Err((client, err)) => {
                    self.state = Some(State::FrameClient(client));
                    Err(PyRuntimeError::new_err(err.to_string()))
                }
            },
            State::RenderRequest(render_request) => {
                self.copy_source_frames(&render_request, buffers)?;
                self.state = Some(State::RenderRequest(render_request));
                Ok(())
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

    fn copy_source_frames(
        &self,
        render_request: &RenderRequest,
        buffers: Bound<PyList>,
    ) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            for (frame_num, buffer) in buffers.iter().enumerate() {
                let frame: PyBuffer<u8> = PyBuffer::get(&buffer)?;
                match render_request.get_source_frame(frame_num) {
                    Ok(source) => frame.copy_from_slice(py, source)?,
                    Err(err) => return Err(PyRuntimeError::new_err(err.to_string())),
                }
            }
            Ok(())
        })
    }
}

#[pymodule]
fn mediafx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MediaFX>()?;
    Ok(())
}
