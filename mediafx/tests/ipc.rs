// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use mediafx::client::Metadata;
use std::env;

const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

fn fill_frame(frame: &mut [u8], value: u8) {
    frame.fill(value);
}

fn server_render_frame(frame_server: &mut mediafx::server::FrameServer, num: u8) {
    let source_frames = frame_server.get_source_frames_mut::<2>().unwrap();
    fill_frame(source_frames[0], num);
    fill_frame(source_frames[1], num + 1);
    let rendered_frame = frame_server.render((0.0, 0.0, 0.0, 0.0)).unwrap();
    let expected_frame = vec![num + num + 1; (WIDTH * HEIGHT * 4) as usize];
    assert_eq!(rendered_frame, &expected_frame);
}

fn frame_server(client_path: &str) {
    let mut frame_server =
        mediafx::server::FrameServer::new(client_path, "config", WIDTH, HEIGHT, 2).unwrap();
    for num in 1..10 {
        server_render_frame(&mut frame_server, num);
    }
}

fn client_render_frame(frame_client: mediafx::client::FrameClient) -> mediafx::client::FrameClient {
    let mut request = frame_client.render_frame().unwrap();
    let (frames, rendered_frame) = request.get_frames_with_rendered_frame_mut::<2>().unwrap();
    for (i, b) in rendered_frame.iter_mut().enumerate() {
        *b = frames[0][i] + frames[1][i];
    }
    request.commit().unwrap()
}

fn frame_client() {
    let mut frame_client = mediafx::client::FrameClient::new().unwrap();
    assert_eq!(frame_client.config(), "config");
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
