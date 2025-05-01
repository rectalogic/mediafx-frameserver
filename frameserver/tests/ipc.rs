use frameserver::{client, server};
use std::{env, iter::zip};

fn fill_frame(frame: &mut [u8], value: u8) {
    frame.fill(value);
}

fn frame_server(client_path: &str) {
    let mut frame_server = server::FrameServer::new(client_path, 1024, 768, 2).unwrap();
    fill_frame(frame_server.get_source_frame_mut(0).unwrap(), 1);
    fill_frame(frame_server.get_source_frame_mut(1).unwrap(), 2);
    let mut result = frame_server.render(0.0).unwrap();
    let expected = vec![3; 1024 * 768 * 4];
    assert_eq!(result.get_rendered_frame(), &expected);
    result.finish();
}

fn frame_client() {
    let frame_client = client::FrameClient::new().unwrap();
    let prepare = frame_client.render_prepare().unwrap();
    let frame0 = prepare.get_source_frame(0).unwrap();
    let frame1 = prepare.get_source_frame(1).unwrap();
    let rendered_frame = zip(frame0, frame1)
        .map(|(frame1, frame2)| frame1 + frame2)
        .collect::<Vec<u8>>();
    let mut render = prepare.render();
    render
        .get_rendered_frame_mut()
        .copy_from_slice(&rendered_frame);
    render.finish().unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            frame_server(args[0].as_str());
        }
        2 => {
            frame_client();
        }
        _ => {
            panic!("Invalid arguments");
        }
    }
}
