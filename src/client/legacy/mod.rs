#[cfg(any(feature = "http1", feature = "http2"))]
mod client;
#[cfg(any(feature = "http1", feature = "http2"))]
pub use client::{Builder, Client, Error};

pub mod connect;
#[doc(hidden)]
// Publicly available, but just for legacy purposes. A better pool will be
// designed.
pub mod pool;
