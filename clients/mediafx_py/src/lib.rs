// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

use client_state::{ClientState, RenderData};
use pyo3::{buffer::PyBuffer, exceptions::PyRuntimeError, prelude::*, types::PySequence};

#[pyclass]
struct MediaFX {
    state: ClientState,
}

#[pymethods]
impl MediaFX {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(MediaFX {
            state: ClientState::new().map_err(|err| PyRuntimeError::new_err(err.to_string()))?,
        })
    }

    #[getter]
    fn get_config(&self) -> PyResult<&str> {
        Ok(self.state.config())
    }

    #[getter]
    fn get_frame_bytecount(&self) -> PyResult<usize> {
        Ok(self.state.frame_bytecount())
    }

    #[getter]
    fn get_frame_size(&self) -> PyResult<(u32, u32)> {
        Ok(self.state.frame_size())
    }

    #[getter]
    fn get_frame_count(&self) -> PyResult<usize> {
        Ok(self.state.frame_count())
    }

    #[pyo3(signature = (frames=None))]
    fn render_frame(&mut self, frames: Option<&Bound<PySequence>>) -> PyResult<RenderData> {
        let render_result = self.state.render_frame(frames, |render_frame, frames| {
            Python::with_gil(|py| -> Result<(), Box<dyn Error>> {
                for (frame_num, buffer) in frames.try_iter()?.enumerate() {
                    let frame: PyBuffer<u8> = PyBuffer::get(&buffer?)?;
                    let source = render_frame.get_source_frame(frame_num)?;
                    frame.copy_from_slice(py, source)?;
                }
                Ok(())
            })
        });
        match render_result {
            Ok(render_result) => Ok(render_result),
            Err(err) => match err.downcast::<PyErr>() {
                Ok(pyerr) => Err(*pyerr),
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            },
        }
    }

    fn render_commit(&mut self, frame: PyBuffer<u8>) -> PyResult<()> {
        let render_result = self.state.render_commit(frame, |frame, rendered_frame| {
            Python::with_gil(|py| -> Result<(), Box<dyn Error>> {
                frame.copy_to_slice(py, rendered_frame)?;
                Ok(())
            })
        });
        match render_result {
            Ok(_) => Ok(()),
            Err(err) => match err.downcast::<PyErr>() {
                Ok(pyerr) => Err(*pyerr),
                Err(err) => Err(PyRuntimeError::new_err(err.to_string())),
            },
        }
    }
}

#[pymodule]
fn _mediafx(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MediaFX>()?;
    Ok(())
}
