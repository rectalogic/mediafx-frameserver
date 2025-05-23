// Copyright (C) 2025 Andrew Wason
// SPDX-License-Identifier: GPL-3.0-or-later

pub mod context;
pub mod messages;
use std::os::fd::RawFd;

pub use bincode;

pub const CLIENT_IN_FD: RawFd = 3;
pub const CLIENT_OUT_FD: RawFd = 4;
