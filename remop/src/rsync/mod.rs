//! Synchronization primitives for use in asynchronous and remote contexts.
//!
//! This module is modelled after [tokio::sync] and follows its principles.

use crate::chmux;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{error::Error, fmt};

pub mod broadcast;
mod interlock;
pub mod lazy;
pub mod lr;
pub mod mpsc;
pub mod oneshot;
pub mod raw;
pub mod remote;
pub mod rw_lock;
pub mod watch;

/// Error connecting a remote channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectError {
    /// The corresponding sender or receiver has been dropped.
    Dropped,
    /// Error initiating chmux connection.
    Connect(chmux::ConnectError),
    /// Error listening for or accepting chmux connection.
    Listen(chmux::ListenerError),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConnectError::Dropped => write!(f, "other part was dropped"),
            ConnectError::Connect(err) => write!(f, "connect error: {}", err),
            ConnectError::Listen(err) => write!(f, "listen error: {}", err),
        }
    }
}

impl Error for ConnectError {}

/// Object that is remote sendable.
pub trait RemoteSend: Send + Serialize + DeserializeOwned + 'static {}

impl<T> RemoteSend for T where T: Send + Serialize + DeserializeOwned + 'static {}

pub(crate) const BACKCHANNEL_MSG_CLOSE: u8 = 0x01;
pub(crate) const BACKCHANNEL_MSG_ERROR: u8 = 0x02;

#[derive(Clone)]
pub(crate) enum RemoteSendError {
    Send(remote::SendErrorKind),
    Connect(chmux::ConnectError),
    Listen(chmux::ListenerError),
    Forward,
}