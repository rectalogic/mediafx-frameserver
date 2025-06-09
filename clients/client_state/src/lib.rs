// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

pub use mediafx::client::RenderData;
use mediafx::client::{BYTES_PER_PIXEL, MediaFXClient, RenderFrame, RenderSize};

enum State {
    Client(MediaFXClient),
    RenderFrame(RenderFrame),
}

pub struct ClientState {
    state: Option<State>,
}

impl ClientState {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            state: Some(State::Client(MediaFXClient::new()?)),
        })
    }

    fn with_state<F, R, E>(&mut self, f: F) -> Result<R, E>
    where
        F: FnOnce(State) -> Result<(State, R), (State, E)>,
    {
        let state = self.state.take().expect("State already taken");
        match f(state) {
            Ok((state, result)) => {
                self.state = Some(state);
                Ok(result)
            }
            Err((state, err)) => {
                self.state = Some(state);
                Err(err)
            }
        }
    }

    pub fn config(&self) -> &str {
        match self.state {
            Some(State::Client(ref client)) => client.config(),
            Some(State::RenderFrame(ref render_frame)) => render_frame.config(),
            None => unreachable!("Invalid state"),
        }
    }

    fn render_size(&self) -> RenderSize {
        match self.state {
            Some(State::Client(ref client)) => client.render_size(),
            Some(State::RenderFrame(ref render_frame)) => render_frame.render_size(),
            None => unreachable!("Invalid state"),
        }
    }

    pub fn frame_bytecount(&self) -> usize {
        let size = self.render_size();
        (size.width() * size.height()) as usize * BYTES_PER_PIXEL
    }

    pub fn frame_size(&self) -> (u32, u32) {
        let size = self.render_size();
        (size.width(), size.height())
    }

    pub fn frame_count(&self) -> usize {
        self.render_size().count()
    }

    #[allow(clippy::result_large_err)]
    pub fn render_frame<F, C>(
        &mut self,
        frames: Option<F>,
        copy_frames: C,
    ) -> Result<RenderData, Box<dyn Error>>
    where
        C: FnOnce(&RenderFrame, F) -> Result<(), Box<dyn Error>>,
    {
        self.with_state(|state| {
            let render_frame = match state {
                State::Client(client) => client
                    .render_frame()
                    .map_err(|(client, err)| (State::Client(client), err))?,
                State::RenderFrame(render_frame) => render_frame,
            };

            let render_data = *render_frame.render_data();

            if let Some(frames) = frames {
                if let Err(err) = copy_frames(&render_frame, frames) {
                    return Err((State::RenderFrame(render_frame), err));
                }
            }

            Ok((State::RenderFrame(render_frame), render_data))
        })
    }

    #[allow(clippy::result_large_err)]
    pub fn render_commit<F, C>(&mut self, frame: F, copy_frame: C) -> Result<(), Box<dyn Error>>
    where
        C: FnOnce(F, &mut [u8]) -> Result<(), Box<dyn Error>>,
    {
        self.with_state(|state| match state {
            State::RenderFrame(mut render_frame) => {
                let rendered_frame = render_frame.get_rendered_frame_mut();

                if let Err(err) = copy_frame(frame, rendered_frame) {
                    return Err((State::RenderFrame(render_frame), err));
                }
                match render_frame.commit() {
                    Ok(client) => Ok((State::Client(client), ())),
                    Err((render_frame, err)) => Err((State::RenderFrame(render_frame), err)),
                }
            }
            State::Client(client) => Err((
                State::Client(client),
                "Cannot commit in current state".into(),
            )),
        })
    }
}
