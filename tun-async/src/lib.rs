#[cfg(feature = "tokio")]
#[path = "tokio.rs"]
pub(crate) mod imp;

pub use imp::TunQueue;
