//! Test application demonstrating an architecture for combining `async`
//! runtime-independent and runtime-specific functionality.

use crate::{Channel, Receiver, Result, Runtime, Sender};

#[derive(Clone)]
pub struct App<R: Runtime> {
    cmd_tx: <<R as Runtime>::Channel as Channel>::Sender<Request<R>>,
    _runtime: std::marker::PhantomData<R>,
}

impl<R: Runtime> App<R> {
    /// Create the app with initial state.
    pub fn new<S: AsRef<str>>(state: S) -> (Self, AppDriver<R>) {
        let (cmd_tx, cmd_rx) = R::Channel::unbounded();
        (
            Self {
                cmd_tx,
                _runtime: Default::default(),
            },
            AppDriver {
                state: state.as_ref().to_owned(),
                cmd_rx,
                _runtime: Default::default(),
            },
        )
    }

    pub async fn get_state(&self) -> Result<String> {
        self.command(Command::GetState).await
    }

    pub async fn update_state<S: AsRef<str>>(&self, new_state: S) -> Result<String> {
        self.command(Command::UpdateState(new_state.as_ref().to_owned()))
            .await
    }

    pub async fn terminate(self) -> Result<String> {
        self.command(Command::Terminate).await
    }

    async fn command(&self, cmd: Command) -> Result<String> {
        let (reply_tx, mut reply_rx) = R::Channel::unbounded();
        self.cmd_tx
            .send(Request {
                cmd,
                reply_tx: reply_tx.clone(),
            })
            .await?;
        reply_rx.recv().await?.0
    }
}

pub struct AppDriver<R: Runtime> {
    state: String,
    cmd_rx: <<R as Runtime>::Channel as Channel>::Receiver<Request<R>>,
    _runtime: std::marker::PhantomData<R>,
}

// Runtime-independent code.
impl<R: Runtime> AppDriver<R> {
    pub fn new(
        state: String,
        cmd_rx: <<R as Runtime>::Channel as Channel>::Receiver<Request<R>>,
    ) -> AppDriver<R> {
        Self {
            state,
            cmd_rx,
            _runtime: Default::default(),
        }
    }

    async fn handle_request(&mut self, request: Request<R>) -> Result<()> {
        match request.cmd {
            Command::GetState => self.get_state(request.reply_tx).await,
            Command::UpdateState(new_state) => self.update_state(new_state, request.reply_tx).await,
            Command::Terminate => return self.terminate(request.reply_tx).await,
        }
    }

    async fn get_state(
        &self,
        reply_tx: <<R as Runtime>::Channel as Channel>::Sender<CommandResult>,
    ) -> Result<()> {
        reply_tx.send(CommandResult(Ok(self.state.clone()))).await
    }

    async fn update_state(
        &mut self,
        new_state: String,
        reply_tx: <<R as Runtime>::Channel as Channel>::Sender<CommandResult>,
    ) -> Result<()> {
        self.state = new_state;
        reply_tx.send(CommandResult(Ok(self.state.clone()))).await
    }

    async fn terminate(
        &mut self,
        reply_tx: <<R as Runtime>::Channel as Channel>::Sender<CommandResult>,
    ) -> Result<()> {
        reply_tx.send(CommandResult(Ok(self.state.clone()))).await?;
        Ok(())
    }
}

#[cfg(feature = "with-tokio")]
impl AppDriver<crate::Tokio> {
    pub async fn run(mut self) -> Result<()> {
        loop {
            tokio::select! {
                result = self.cmd_rx.recv() => self.handle_request(result?).await?,
            }
        }
    }
}

#[cfg(feature = "with-async-std")]
use futures::FutureExt;

#[cfg(feature = "with-async-std")]
impl AppDriver<crate::AsyncStd> {
    pub async fn run(mut self) -> Result<()> {
        loop {
            futures::select! {
                result = self.cmd_rx.recv().fuse() => self.handle_request(result?).await?,
            }
        }
    }
}

#[derive(Debug, Clone)]
enum Command {
    GetState,
    UpdateState(String),
    Terminate,
}

#[derive(Clone)]
struct CommandResult(Result<String>);

#[derive(Clone)]
struct Request<R: Runtime> {
    cmd: Command,
    reply_tx: <<R as Runtime>::Channel as Channel>::Sender<CommandResult>,
}
