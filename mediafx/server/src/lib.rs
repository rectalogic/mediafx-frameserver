// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
    error::Error,
    io::{self, PipeReader, PipeWriter},
    os::{
        fd::{AsRawFd, RawFd},
        unix::process::CommandExt,
    },
    process::{Child, Command, Stdio},
};

pub use mediafx_common::messages::RenderData;
use mediafx_common::{
    CLIENT_IN_FD, CLIENT_OUT_FD,
    context::{RenderContext, RenderSize},
    messages::{RenderAck, RenderFrame, RenderInitialize, receive_message, send_message},
};

use shared_memory::ShmemConf;

pub struct MediaFXServer {
    context: RenderContext,
    client: Child,
    client_input: PipeWriter,
    client_output: PipeReader,
}

impl MediaFXServer {
    pub fn new<C: Into<String>>(
        client_path: &str,
        config: C,
        width: u32,
        height: u32,
        count: usize,
    ) -> Result<Self, Box<dyn Error>> {
        let size = RenderSize::new(width, height, count);
        let (mut childout_rx, childout_tx) = io::pipe()?;
        let (childin_rx, mut childin_tx) = io::pipe()?;
        let (childin_tx_fd, childin_rx_fd) = (childin_tx.as_raw_fd(), childin_rx.as_raw_fd());
        let (childout_tx_fd, childout_rx_fd) = (childout_tx.as_raw_fd(), childout_rx.as_raw_fd());

        // XXX whitelist programs, support args?
        let mut command = Command::new(client_path);
        command.stdin(Stdio::null());

        fn dup_fd(src_fd: RawFd, dst_fd: RawFd) -> Result<(), io::Error> {
            // dup2 removes FD_CLOEXEC flag if fds are different,
            // if they are the same we remove it ourselves
            if src_fd == dst_fd {
                let flags = unsafe { libc::fcntl(dst_fd, libc::F_GETFD) };
                if flags == -1 {
                    return Err(io::Error::last_os_error());
                }
                if unsafe { libc::fcntl(dst_fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) } == -1 {
                    return Err(io::Error::last_os_error());
                }
            } else if unsafe { libc::dup2(src_fd, dst_fd) } == -1 {
                return Err(io::Error::last_os_error());
            }
            Ok(())
        }
        fn close_fd(fd: RawFd) {
            if fd != CLIENT_IN_FD && fd != CLIENT_OUT_FD {
                // Ignore error, FD_CLOEXEC is set on these fds anyway
                let _ = unsafe { libc::close(fd) };
            }
        }

        unsafe {
            command.pre_exec(move || {
                dup_fd(childin_rx_fd, CLIENT_IN_FD)?;
                dup_fd(childout_tx_fd, CLIENT_OUT_FD)?;

                close_fd(childin_rx_fd);
                close_fd(childin_tx_fd);
                close_fd(childout_tx_fd);
                close_fd(childout_rx_fd);

                Ok(())
            })
        };
        let client = command.spawn()?;
        drop(childout_tx);
        drop(childin_rx);

        let shmem = ShmemConf::new().size(size.memory_size()).create()?;
        let render_initialize =
            RenderInitialize::new(size, shmem.get_os_id().into(), config.into());
        let context = RenderContext::new(size, shmem);

        send_message(&render_initialize, &mut childin_tx)?;
        // XXX check for errors
        let _response: RenderAck = receive_message(&mut childout_rx)?;
        Ok(MediaFXServer {
            context,
            client,
            client_input: childin_tx,
            client_output: childout_rx,
        })
    }

    pub fn get_source_frame_mut(&mut self, frame_num: usize) -> Result<&mut [u8], Box<dyn Error>> {
        self.context.frame_mut(frame_num)
    }

    pub fn render(&mut self, render_data: RenderData) -> Result<&mut [u8], Box<dyn Error>> {
        send_message(RenderFrame::Render(render_data), &mut self.client_input)?;
        // XXX check for errors
        let _response: RenderAck = receive_message(&mut self.client_output)?;
        Ok(self.context.rendered_frame_mut())
    }
}

impl Drop for MediaFXServer {
    fn drop(&mut self) {
        if send_message(RenderFrame::Terminate, &mut self.client_input).is_err() {
            let _ = self.client.kill();
        } else {
            //XXX need to close this so it will exit
            // drop(self.client_input);
            let _ = self.client.wait();
        }
    }
}
