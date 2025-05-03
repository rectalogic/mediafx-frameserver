use super::FrameServer;
use std::ffi::CStr;
use std::os::raw::c_char;

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn mediafx_create_server(
    client_path: *const c_char,
    width: u32,
    height: u32,
    count: usize,
) -> *mut FrameServer {
    if !client_path.is_null() {
        let client_path = unsafe { CStr::from_ptr(client_path) };
        match client_path.to_str() {
            Ok(client_path) => match FrameServer::new(client_path, width, height, count) {
                Ok(frame_server) => Box::into_raw(Box::new(frame_server)),
                Err(e) => {
                    eprintln!("mediafx_initialize: {}", e);
                    std::ptr::null_mut::<FrameServer>()
                }
            },
            Err(e) => {
                eprintln!("mediafx_initialize client path: {}", e);
                std::ptr::null_mut::<FrameServer>()
            }
        }
    } else {
        eprintln!("client_path null pointer");
        std::ptr::null_mut::<FrameServer>()
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn mediafx_get_frame(frame_server: *mut FrameServer, frame_num: usize) -> *mut u8 {
    match unsafe { frame_server.as_mut() } {
        Some(frame_server) => match frame_server.get_source_frame_mut(frame_num) {
            Ok(frame) => frame.as_mut_ptr(),
            Err(e) => {
                eprintln!("mediafx_get_frame: {}", e);
                std::ptr::null_mut::<u8>()
            }
        },
        _ => {
            eprintln!("mediafx_get_frame invalid state");
            std::ptr::null_mut::<u8>()
        }
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn mediafx_render(frame_server: *mut FrameServer, time: f32) -> *mut u8 {
    match unsafe { frame_server.as_mut() } {
        Some(frame_server) => match frame_server.render(time) {
            Ok(rendered_frame) => rendered_frame.as_mut_ptr(),
            Err(e) => {
                eprintln!("mediafx_render: {}", e);
                std::ptr::null_mut::<u8>()
            }
        },
        None => {
            eprintln!("mediafx_render null frame_server");
            std::ptr::null_mut::<u8>()
        }
    }
}

#[unsafe(no_mangle)]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn mediafx_destroy(frame_server: *mut FrameServer) {
    if frame_server.is_null() {
        return;
    }
    unsafe {
        drop(Box::from_raw(frame_server));
    }
}
