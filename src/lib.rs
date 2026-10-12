pub mod exception;
pub use exception::Error;

pub mod frame;
pub use frame::Frame;

mod connection;
pub use connection::Connection;

// A specialized `Result` type for this crate's operations.
// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;
