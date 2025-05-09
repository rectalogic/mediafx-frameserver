// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use frameserver::{client, server};
use std::{env, iter::zip};

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn fill_frame(frame: &mut [u8], value: u8) {
    frame.fill(value);
}

fn server_render_frame(frame_server: &mut server::FrameServer, num: u8) {
    fill_frame(frame_server.get_source_frame_mut(0).unwrap(), num);
    fill_frame(frame_server.get_source_frame_mut(1).unwrap(), num + 1);
    let rendered_frame = frame_server.render(0.0).unwrap();
    let expected_frame = vec![num + num + 1; (WIDTH * HEIGHT * 4) as usize];
    assert_eq!(rendered_frame, &expected_frame);
}

fn frame_server(client_path: &str) {
    let mut frame_server = server::FrameServer::new(client_path, WIDTH, HEIGHT, 2).unwrap();
    for num in 1..10 {
        server_render_frame(&mut frame_server, num);
    }
}

fn client_render_frame(frame_client: client::FrameClient) -> client::FrameClient {
    let mut request = frame_client.request_render().unwrap();
    let frame0 = request.get_source_frame(0).unwrap();
    let frame1 = request.get_source_frame(1).unwrap();
    let rendered_frame = zip(frame0, frame1)
        .map(|(frame1, frame2)| frame1 + frame2)
        .collect::<Vec<u8>>();

    request
        .get_rendered_frame_mut()
        .copy_from_slice(&rendered_frame);
    request.render_complete().unwrap()
}

fn frame_client() {
    let mut frame_client = client::FrameClient::new().unwrap();
    loop {
        frame_client = client_render_frame(frame_client);
    }
}

fn main() {
    if env::var("IPC_CLIENT").is_ok() {
        frame_client();
    } else {
        unsafe { env::set_var("IPC_CLIENT", "1") };
        frame_server(env::args().next().unwrap().as_str());
    }
}
