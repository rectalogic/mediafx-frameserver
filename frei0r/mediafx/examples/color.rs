// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

// XXX need to forward params to client
const COLOR: [u8; 4] = [255, 0, 0, 255];

fn source_frame(frame_client: mediafx_client::MediaFXClient) -> mediafx_client::MediaFXClient {
    let size = frame_client.render_size();
    let mut request = frame_client.request_render().unwrap();

    let rendered_frame = request.get_rendered_frame_mut();
    for dy in 0..size.height() {
        for dx in 0..size.width() {
            let pixel_offset = ((dy * size.width() + dx) * 4) as usize;
            rendered_frame[pixel_offset] = COLOR[0];
            rendered_frame[pixel_offset + 1] = COLOR[1];
            rendered_frame[pixel_offset + 2] = COLOR[2];
            rendered_frame[pixel_offset + 3] = COLOR[3];
        }
    }

    request.render_complete().unwrap()
}

fn main() {
    let mut frame_client = mediafx_client::MediaFXClient::new().unwrap();
    loop {
        frame_client = source_frame(frame_client);
    }
}
