use frame::{Reason, StreamId};

use std::{error, fmt, io};

/// Errors that are received
#[derive(Debug)]
pub enum RecvError {
    Connection(Reason),
    Stream {
        id: StreamId,
        reason: Reason,
    },
    Io(io::Error),
}

/// Errors caused by sending a message
#[derive(Debug)]
pub enum SendError {
    /// User error
    User(UserError),

    /// I/O error
    Io(io::Error),
}

/// Errors caused by users of the library
#[derive(Debug)]
pub enum UserError {
    /// The stream ID is no longer accepting frames.
    InactiveStreamId,

    /// The stream is not currently expecting a frame of this type.
    UnexpectedFrameType,

    /// The payload size is too big
    PayloadTooBig,

    /// The application attempted to initiate too many streams to remote.
    Rejected,
}

// ===== impl RecvError =====

impl From<io::Error> for RecvError {
    fn from(src: io::Error) -> Self {
        RecvError::Io(src)
    }
}

impl error::Error for RecvError {
    fn description(&self) -> &str {
        use self::RecvError::*;

        match *self {
            Connection(ref reason) => reason.description(),
            Stream { ref reason, .. } => reason.description(),
            Io(ref e) => e.description(),
        }
    }
}

impl fmt::Display for RecvError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(fmt, "{}", self.description())
    }
}

// ===== impl SendError =====

impl error::Error for SendError {
    fn description(&self) -> &str {
        use self::SendError::*;

        match *self {
            User(ref e) => e.description(),
            Io(ref e) => e.description(),
        }
    }
}

impl fmt::Display for SendError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(fmt, "{}", self.description())
    }
}

impl From<io::Error> for SendError {
    fn from(src: io::Error) -> Self {
        SendError::Io(src)
    }
}

impl From<UserError> for SendError {
    fn from(src: UserError) -> Self {
        SendError::User(src)
    }
}

// ===== impl UserError =====

impl error::Error for UserError {
    fn description(&self) -> &str {
        use self::UserError::*;

        match *self {
            InactiveStreamId => "inactive stream",
            UnexpectedFrameType => "unexpected frame type",
            PayloadTooBig => "payload too big",
            Rejected => "rejected",
        }
    }
}

impl fmt::Display for UserError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(fmt, "{}", self.description())
    }
}
