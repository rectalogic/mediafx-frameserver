// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::frameserver::client;

// XXX need to forward params to client
const XSHIFT: f32 = 0.5;
const YSHIFT: f32 = 0.25;

fn filter_frame(frame_client: client::FrameClient) -> client::FrameClient {
    let size = frame_client.render_size();
    let mut request = frame_client.request_render().unwrap();

    let xshift = (XSHIFT * size.width() as f32) as u32;
    let yshift = (YSHIFT * size.height() as f32) as u32;
    for dy in 0..size.height() {
        for dx in 0..size.width() {
            let sy = (dy + yshift) % size.height();
            let sx = (dx + xshift) % size.width();
            let dest_index = (dy * size.width() + dx) as usize * 4;
            let source_index = (sy * size.width() + sx) as usize * 4;
            for channel in 0..4 {
                request.get_rendered_frame_mut()[dest_index + channel] =
                    request.get_source_frame(0).unwrap()[source_index + channel];
            }
        }
    }

    request.render_complete().unwrap()
}

fn main() {
    let mut frame_client = client::FrameClient::new().unwrap();
    loop {
        frame_client = filter_frame(frame_client);
    }
}
