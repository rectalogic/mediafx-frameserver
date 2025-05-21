// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

fn source_frame(frame_client: mediafx_client::MediaFXClient) -> mediafx_client::MediaFXClient {
    let size = frame_client.render_size();
    let mut request = frame_client.request_render().unwrap();

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

    request.render_complete().unwrap()
}

fn main() {
    let mut frame_client = mediafx_client::MediaFXClient::new().unwrap();
    loop {
        frame_client = source_frame(frame_client);
    }
}
