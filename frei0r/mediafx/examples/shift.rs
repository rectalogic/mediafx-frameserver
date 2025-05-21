// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

fn filter_frame(frame_client: mediafx_client::MediaFXClient) -> mediafx_client::MediaFXClient {
    let size = frame_client.render_size();
    let mut request = frame_client.request_render().unwrap();

    let (_, xshift, yshift, _) = *request.render_data();
    let xshift = (xshift * size.width() as f64) as u32;
    let yshift = (yshift * size.height() as f64) as u32;
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
    let mut frame_client = mediafx_client::MediaFXClient::new().unwrap();
    loop {
        frame_client = filter_frame(frame_client);
    }
}
