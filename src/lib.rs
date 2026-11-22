pub mod clients;

pub mod exception;
pub use exception::Error;

pub mod frame;
pub use frame::Frame;

mod connection;
pub use connection::Connection;

mod parse;
use parse::Parse;

mod db;
use db::Db;
use db::DbDropGuard;

pub mod commands;
pub use commands::Command;

mod shutdown;
use shutdown::Shutdown;

/// Default port that a redis server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 6379;

pub mod server;
