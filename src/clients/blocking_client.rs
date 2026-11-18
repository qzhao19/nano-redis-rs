//! Minimal blocking Redis client implementation
//!
//! Provides a blocking connect and methods for issuing the supported commands.

use bytes::Bytes;
use std::time::Duration;
use tokio::net::ToSocketAddrs;
use tokio::runtime::Runtime;

pub use crate::clients::Message;

/// Established connection with a Redis server.
///
/// Backed by a single `TcpStream`, `BlockingClient` provides basic network
/// client functionality (no pooling, retrying, ...). Connections are
/// established using the [`connect`](fn@connect) function.
///
/// Requests are issued using the various methods of `Client`.
pub struct BlockingClient {
    /// The asynchronous `Client`.
    inner: crate::clients::Client,

    /// A `current_thread` runtime for executing operations on the asynchronous
    /// client in a blocking manner.
    rt: Runtime,
}

/// A client that has entered pub/sub mode.
///
/// Once clients subscribe to a channel, they may only perform pub/sub related
/// commands. The `BlockingClient` type is transitioned to a
/// `BlockingSubscriber` type in order to prevent non-pub/sub methods from being
/// called.
pub struct BlockingSubscriber {
    /// The asynchronous `Subscriber`.
    inner: crate::clients::Subscriber,

    /// A `current_thread` runtime for executing operations on the asynchronous
    /// `Subscriber` in a blocking manner.
    rt: Runtime,
}
