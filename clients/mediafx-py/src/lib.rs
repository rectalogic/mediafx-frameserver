// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx_client::{BYTES_PER_PIXEL, MediaFXClient, RenderData, RenderRequest, RenderSize};
use pyo3::{buffer::PyBuffer, exceptions::PyRuntimeError, prelude::*, types::PySequence};

enum State {
    MediaFXClient(MediaFXClient),
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
        Ok(MediaFX {
            state: Some(State::MediaFXClient(
                MediaFXClient::new().map_err(|err| PyRuntimeError::new_err(err.to_string()))?,
            )),
        })
    }

    #[getter]
    fn get_config(&self) -> PyResult<&str> {
        Ok(self.get_config_internal())
    }

    #[getter]
    fn get_frame_bytecount(&self) -> PyResult<usize> {
        let size = self.get_size();
        Ok((size.width() * size.height()) as usize * BYTES_PER_PIXEL)
    }

    #[getter]
    fn get_frame_size(&self) -> PyResult<(u32, u32)> {
        let size = self.get_size();
        Ok((size.width(), size.height()))
    }

    #[getter]
    fn get_frame_count(&self) -> PyResult<usize> {
        Ok(self.get_size().count())
    }

    #[pyo3(signature = (frames=None))]
    fn render_begin(&mut self, frames: Option<&Bound<PySequence>>) -> PyResult<RenderData> {
        let current_state = self.state.take().expect("Invalid internal state");
        match current_state {
            State::MediaFXClient(client) => {
                let render_request = client.request_render().map_err(|(client, err)| {
                    self.state = Some(State::MediaFXClient(client));
                    PyRuntimeError::new_err(err.to_string())
                })?;
                let render_data = *render_request.render_data();
                if let Some(buffers) = frames {
                    self.copy_source_frames(&render_request, buffers)?;
                }
                self.state = Some(State::RenderRequest(render_request));
                Ok(render_data)
            }
            State::RenderRequest(render_request) => {
                let time = *render_request.render_data();
                if let Some(buffers) = frames {
                    self.copy_source_frames(&render_request, buffers)?;
                }
                self.state = Some(State::RenderRequest(render_request));
                Ok(time)
            }
        }
    }

    fn render_finish(&mut self, frame: PyBuffer<u8>) -> PyResult<()> {
        let current_state = self.state.take().expect("Invalid internal state");
        match current_state {
            State::RenderRequest(mut render_request) => {
                let rendered_frame = render_request.get_rendered_frame_mut();
                let result = Python::with_gil(|py| -> PyResult<()> {
                    frame.copy_to_slice(py, rendered_frame)
                });
                if let Err(err) = result {
                    self.state = Some(State::RenderRequest(render_request));
                    return Err(PyRuntimeError::new_err(err.to_string()));
                }

                let client =
                    render_request
                        .render_complete()
                        .map_err(|(render_request, err)| {
                            self.state = Some(State::RenderRequest(render_request));
                            PyRuntimeError::new_err(err.to_string())
                        })?;
                self.state = Some(State::MediaFXClient(client));
                Ok(())
            }
            State::MediaFXClient(client) => {
                self.state = Some(State::MediaFXClient(client));
                Err(PyRuntimeError::new_err("Invalid state"))
            }
        }
    }
}

impl MediaFX {
    fn get_size(&self) -> RenderSize {
        match self.state {
            Some(State::MediaFXClient(ref client)) => client.render_size(),
            Some(State::RenderRequest(ref client)) => client.render_size(),
            None => unreachable!(),
        }
    }

    fn get_config_internal(&self) -> &str {
        match self.state {
            Some(State::MediaFXClient(ref client)) => client.config(),
            Some(State::RenderRequest(ref client)) => client.config(),
            None => unreachable!(),
        }
    }

    fn copy_source_frames(
        &self,
        render_request: &RenderRequest,
        frames: &Bound<PySequence>,
    ) -> PyResult<()> {
        Python::with_gil(|py| -> PyResult<()> {
            for (frame_num, buffer) in frames.try_iter()?.enumerate() {
                let frame: PyBuffer<u8> = PyBuffer::get(&buffer?)?;
                let source = render_request
                    .get_source_frame(frame_num)
                    .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
                frame.copy_from_slice(py, source)?;
            }
            Ok(())
        })
    }
}

#[pymodule]
fn _mediafx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MediaFX>()?;
    Ok(())
}
