#![allow(incomplete_features)]
#![feature(generic_associated_types)]

mod app;
mod result;
mod runtime;

pub use app::{App, AppDriver};
pub use result::{Error, Result};
pub use runtime::{Channel, Receiver, Runtime, Sender};

#[cfg(feature = "with-tokio")]
pub use runtime::with_tokio::{Tokio, TokioChannel, TokioReceiver, TokioSender};

#[cfg(feature = "with-async-std")]
pub use runtime::with_async_std::{AsyncStd, AsyncStdChannel, AsyncStdReceiver, AsyncStdSender};
