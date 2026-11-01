use std::fmt;
use std::io;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a frame from the socket. More
    /// data may still arrive, so this is recoverable: the caller waits for
    /// more bytes (see `Connection::parse_frame`, which maps this to
    /// `Ok(None)`).
    Incomplete,

    /// A command's array frame was fully consumed before all of the command's
    /// fields were read. The frame itself was received in its entirety, so
    /// waiting for more data cannot help: this is a malformed command and the
    /// connection must be terminated.
    EndOfStream,

    /// An error occurred on the underlying transport.
    Io(io::Error),

    /// Invalid message encoding
    Other(String),
}

impl From<String> for Error {
    fn from(src: String) -> Error {
        Error::Other(src)
    }
}

impl From<&str> for Error {
    fn from(src: &str) -> Error {
        Error::Other(src.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(_src: FromUtf8Error) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<TryFromIntError> for Error {
    fn from(_src: TryFromIntError) -> Error {
        "protocol error; invalid frame format".into()
    }
}

impl From<io::Error> for Error {
    fn from(src: io::Error) -> Error {
        Error::Io(src)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Incomplete => "stream ended early".fmt(fmt),
            Error::EndOfStream => "protocol error; unexpected end of stream".fmt(fmt),
            Error::Io(err) => err.fmt(fmt),
            Error::Other(msg) => msg.fmt(fmt),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}
