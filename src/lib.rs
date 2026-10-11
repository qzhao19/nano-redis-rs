pub mod frame;
pub use frame::Frame;

/// Error returned by most functions.
///
/// When writing a real application, one might want to consider a specialized
/// error handling crate or defining an error type as an `enum` of causes.
/// However, for this example, using a boxed `std::error::Error` is sufficient.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for this crate's operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;
