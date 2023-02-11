//! Errors that can occur in the process of connecting to clients, parsing HTTP and handling requests.

use std::{rc::Rc, result};

use crate::{Method, Request};

/// Easy way to use a Result<T, Error>
pub type Result<T> = result::Result<T, Error>;

/// Errors that can occur at startup or in the process of connecting to clients, parsing HTTP and handling requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Error while starting the server
    Startup(StartupError),

    /// Stream error
    Stream(StreamError),

    /// Error while handling a Request
    Handle(Box<HandleError>),

    /// Error while parsing request HTTP
    Parse(ParseError),

    /// IO Errors
    Io(String),

    /// Response does not exist (probably because of an error with the request)
    None,
}

/// Errors that can occur while starting the server
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartupError {
    /// The IP address specified is invalid
    InvalidIp,

    /// No state was specified, but a route requires it
    NoState,

    /// The socket timeout specified is invalid (must be greater than 0)
    InvalidSocketTimeout,
}

/// Errors that can arise while handling a request
#[derive(Debug, Clone)]
pub enum HandleError {
    /// Route matching request path not found
    NotFound(Method, String),

    /// A route or middleware panicked while running
    Panic(Box<Result<Rc<Request>>>, String),
}

/// Error that can occur while parsing the HTTP of a request
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// No `\r\n\r\n` found in request to separate metadata from body
    NoSeparator,

    /// No Method found in request HTTP
    NoMethod,

    /// No Path found in request HTTP
    NoPath,

    /// No Version found in request HTTP
    NoVersion,

    /// No Request Line found in HTTP
    NoRequestLine,

    /// Invalid Query in Path
    InvalidQuery,

    /// Invalid Method in Request HTTP
    InvalidMethod,

    /// Invalid Header in Request HTTP
    InvalidHeader,
}

/// Error that can occur while reading or writing to a stream
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamError {
    /// The stream ended unexpectedly
    UnexpectedEof,
}

impl From<StartupError> for Error {
    fn from(e: StartupError) -> Self {
        Error::Startup(e)
    }
}

impl From<StreamError> for Error {
    fn from(e: StreamError) -> Self {
        Error::Stream(e)
    }
}

impl From<ParseError> for Error {
    fn from(e: ParseError) -> Self {
        Error::Parse(e)
    }
}

impl From<HandleError> for Error {
    fn from(e: HandleError) -> Self {
        Error::Handle(Box::new(e))
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.to_string())
    }
}

impl Eq for HandleError {}
impl PartialEq for HandleError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HandleError::NotFound(m1, p1), HandleError::NotFound(m2, p2)) => m1 == m2 && p1 == p2,
            (HandleError::Panic(_, s1), HandleError::Panic(_, s2)) => s1 == s2,
            _ => false,
        }
    }
}
