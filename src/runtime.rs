//! Abstractions for runtime-independent channels.

use crate::result::Result;
use async_trait::async_trait;

pub trait Runtime: Clone + Send {
    type Channel: Channel;
}

pub trait Channel: Clone + Send {
    type Sender<T: Clone + Send>: Sender<T>;
    type Receiver<T: Send>: Receiver<T>;

    fn unbounded<T: Clone + Send>() -> (Self::Sender<T>, Self::Receiver<T>);
}

#[async_trait]
pub trait Sender<T: Clone + Send>: Clone + Send {
    async fn send(&self, value: T) -> Result<()>;
}

#[async_trait]
pub trait Receiver<T: Send> {
    async fn recv(&mut self) -> Result<T>;
}

#[cfg(feature = "with-tokio")]
pub mod with_tokio {
    use super::*;
    use crate::result::Error;

    #[derive(Clone)]
    pub struct Tokio;

    impl Runtime for Tokio {
        type Channel = TokioChannel;
    }

    #[derive(Clone)]
    pub struct TokioChannel;

    impl Channel for TokioChannel {
        type Sender<T: Clone + Send> = TokioSender<T>;
        type Receiver<T: Send> = TokioReceiver<T>;

        fn unbounded<T: Clone + Send>() -> (Self::Sender<T>, Self::Receiver<T>) {
            let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
            (TokioSender(tx), TokioReceiver(rx))
        }
    }

    #[derive(Clone)]
    pub struct TokioSender<T>(tokio::sync::mpsc::UnboundedSender<T>);

    #[async_trait]
    impl<T: Clone + Send> Sender<T> for TokioSender<T> {
        async fn send(&self, value: T) -> Result<()> {
            self.0
                .send(value)
                .map_err(|e| Error::ChannelSend(e.to_string()))
        }
    }

    pub struct TokioReceiver<T>(tokio::sync::mpsc::UnboundedReceiver<T>);

    #[async_trait]
    impl<T: Send> Receiver<T> for TokioReceiver<T> {
        async fn recv(&mut self) -> Result<T> {
            self.0.recv().await.ok_or(Error::ChannelSenderClosed)
        }
    }
}

#[cfg(feature = "with-async-std")]
pub mod with_async_std {
    use super::*;
    use crate::result::Error;

    #[derive(Clone)]
    pub struct AsyncStd;

    impl Runtime for AsyncStd {
        type Channel = AsyncStdChannel;
    }

    #[derive(Clone)]
    pub struct AsyncStdChannel;

    impl Channel for AsyncStdChannel {
        type Sender<T: Clone + Send> = AsyncStdSender<T>;
        type Receiver<T: Send> = AsyncStdReceiver<T>;

        fn unbounded<T: Clone + Send>() -> (Self::Sender<T>, Self::Receiver<T>) {
            let (tx, rx) = async_channel::unbounded();
            (AsyncStdSender(tx), AsyncStdReceiver(rx))
        }
    }

    #[derive(Clone)]
    pub struct AsyncStdSender<T>(async_channel::Sender<T>);

    #[async_trait]
    impl<T: Clone + Send> Sender<T> for AsyncStdSender<T> {
        async fn send(&self, value: T) -> Result<()> {
            self.0
                .send(value)
                .await
                .map_err(|e| Error::ChannelSend(e.to_string()))
        }
    }

    pub struct AsyncStdReceiver<T>(async_channel::Receiver<T>);

    #[async_trait]
    impl<T: Send> Receiver<T> for AsyncStdReceiver<T> {
        async fn recv(&mut self) -> Result<T> {
            self.0
                .recv()
                .await
                .map_err(|e| Error::ChannelRecv(e.to_string()))
        }
    }
}
