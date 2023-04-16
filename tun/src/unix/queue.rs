use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};

use crate::TunInterface;

pub struct TunQueue {
    socket: socket2::Socket,
}

impl From<TunInterface> for TunQueue {
    fn from(interface: TunInterface) -> TunQueue {
        TunQueue {
            socket: interface.socket,
        }
    }
}

impl AsRawFd for TunQueue {
    fn as_raw_fd(&self) -> RawFd {
        self.socket.as_raw_fd()
    }
}

impl IntoRawFd for TunQueue {
    fn into_raw_fd(self) -> RawFd {
        self.socket.into_raw_fd()
    }
}
