// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::client::Metadata;

fn source_frame(frame_client: mediafx::client::FrameClient) -> mediafx::client::FrameClient {
    let size = frame_client.render_size();
    let mut request = frame_client.render_frame().unwrap();

    let (_, r, g, b) = *request.render_data();
    let rendered_frame = request.get_rendered_frame_mut();
    for dy in 0..size.height() {
        for dx in 0..size.width() {
            let pixel_offset = ((dy * size.width() + dx) * 4) as usize;
            rendered_frame[pixel_offset] = (r * 255.) as u8;
            rendered_frame[pixel_offset + 1] = (g * 255.) as u8;
            rendered_frame[pixel_offset + 2] = (b * 255.) as u8;
            rendered_frame[pixel_offset + 3] = 255;
        }
    }

    request.commit().unwrap()
}

fn main() {
    let mut frame_client = mediafx::client::FrameClient::new().unwrap();
    loop {
        frame_client = source_frame(frame_client);
    }
}
