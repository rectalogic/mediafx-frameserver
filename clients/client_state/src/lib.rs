// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::error::Error;

use mediafx_client::{BYTES_PER_PIXEL, MediaFXClient, RenderData, RenderRequest, RenderSize};

enum State {
    MediaFXClient(MediaFXClient),
    RenderRequest(RenderRequest),
}

pub struct ClientState {
    state: Option<State>,
}

impl ClientState {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            state: Some(State::MediaFXClient(MediaFXClient::new()?)),
        })
    }

    pub fn config(&self) -> &str {
        match self.state {
            Some(State::MediaFXClient(ref client)) => client.config(),
            Some(State::RenderRequest(ref client)) => client.config(),
            None => unreachable!("Invalid state"),
        }
    }

    fn render_size(&self) -> RenderSize {
        match self.state {
            Some(State::MediaFXClient(ref client)) => client.render_size(),
            Some(State::RenderRequest(ref client)) => client.render_size(),
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

    fn handle_render_request<F, C>(
        &mut self,
        render_request: RenderRequest,
        frames: Option<F>,
        copy_frames: C,
    ) -> Result<RenderData, Box<dyn Error>>
    where
        C: FnOnce(&RenderRequest, F) -> Result<(), Box<dyn Error>>,
    {
        let render_data = *render_request.render_data();
        if let Some(frames) = frames {
            let result = copy_frames(&render_request, frames);
            self.state = Some(State::RenderRequest(render_request));
            result?;
            Ok(render_data)
        } else {
            self.state = Some(State::RenderRequest(render_request));
            Ok(render_data)
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn render_begin<F, C>(
        &mut self,
        frames: Option<F>,
        copy_frames: C,
    ) -> Result<RenderData, Box<dyn Error>>
    where
        C: FnOnce(&RenderRequest, F) -> Result<(), Box<dyn Error>>,
    {
        let state = self.state.take().expect("Invalid internal state");
        match state {
            State::MediaFXClient(client) => {
                let render_request = client.request_render().map_err(|(client, err)| {
                    self.state = Some(State::MediaFXClient(client));
                    err
                })?;
                self.handle_render_request(render_request, frames, copy_frames)
            }
            State::RenderRequest(render_request) => {
                self.handle_render_request(render_request, frames, copy_frames)
            }
        }
    }

    #[allow(clippy::result_large_err)]
    pub fn render_finish<F, C>(&mut self, frame: F, copy_frame: C) -> Result<(), Box<dyn Error>>
    where
        C: FnOnce(F, &mut [u8]) -> Result<(), Box<dyn Error>>,
    {
        let state = self.state.take().expect("Invalid internal state");
        match state {
            State::RenderRequest(mut render_request) => {
                let rendered_frame = render_request.get_rendered_frame_mut();
                if let Err(err) = copy_frame(frame, rendered_frame) {
                    self.state = Some(State::RenderRequest(render_request));
                    return Err(err);
                }
                match render_request.render_complete() {
                    Ok(client) => {
                        self.state = Some(State::MediaFXClient(client));
                        Ok(())
                    }
                    Err((render_request, err)) => {
                        self.state = Some(State::RenderRequest(render_request));
                        Err(err)
                    }
                }
            }
            State::MediaFXClient(client) => {
                self.state = Some(State::MediaFXClient(client));
                Err("Invalid state".into())
            }
        }
    }
}
