//! Results and error handling.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("channel send error: {0}")]
    ChannelSend(String),

    #[error("channel receive error: {0}")]
    ChannelRecv(String),

    #[error("sending end of channel closed")]
    ChannelSenderClosed,
}
