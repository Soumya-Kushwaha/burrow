use fehler::throws;
use mio::{event, Interest, Registry, Token};
use std::{
    io,
    io::Error,
    os::fd::AsRawFd,
    pin::Pin,
    task::{Context, Poll},
};
use tokio::io::{AsyncRead, AsyncWrite};
use tokio_core::reactor::PollEvented;

#[cfg(unix)]
use mio::unix::SourceFd;

struct TunIo<T>
where
    T: AsRawFd,
{
    inner: T,
}

impl<T> event::Source for TunIo<T>
where
    T: AsRawFd,
{
    #[throws]
    fn register(&mut self, registry: &Registry, token: Token, interests: Interest) {
        SourceFd(&self.inner.as_raw_fd()).register(registry, token, interests)?
    }

    #[throws]
    fn reregister(&mut self, registry: &Registry, token: Token, interests: Interest) {
        SourceFd(&self.inner.as_raw_fd()).reregister(registry, token, interests)
    }

    #[throws]
    fn deregister(&mut self, registry: &Registry) {
        SourceFd(&self.inner.as_raw_fd()).deregister(registry)
    }
}

pub struct TunQueue {
    io: PollEvented<TunIo<tun::TunQueue>>,
}

impl AsyncWrite for TunQueue {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        self.io.poll_write(cx, buf)?
    }

    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        todo!()
    }

    fn is_write_vectored(&self) -> bool {
        true
    }

    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        // tcp flush is a no-op
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        // self.shutdown_std(std::net::Shutdown::Write)?;
        Poll::Ready(Ok(()))
    }
}
