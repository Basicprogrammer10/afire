#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// Current version of afire
#[doc(hidden)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Contains all the constants used in afire.
/// These may be in the future moved into the [`Server`] struct.
mod consts {
    /// The initial buffer allocation for the request.
    pub const BUFF_SIZE: usize = 256;

    /// Max chunk size for chunked transfer encoding.
    pub const CHUNK_SIZE: usize = 16 * 1024;
}

// Export Internal Functions
pub mod internal;

// Import Internal Functions
mod thread_pool;
use http::*;
use internal::{encoding, handle, path};

#[macro_use]
pub mod trace;
pub mod error;
mod http;
pub mod middleware;
mod request;
mod response;
mod route;
mod server;
pub use self::{
    content_type::Content,
    cookie::{Cookie, SetCookie},
    error::Error,
    header::{Header, HeaderType},
    http::header,
    http::multipart,
    method::Method,
    middleware::Middleware,
    query::Query,
    request::Request,
    response::Response,
    route::Route,
    server::Server,
    status::Status,
};

/// The Prelude is a collection of very commonly used *things* in afire.
/// Unless you are using middleware, extensions or internal lower level stuff this should be all you need!
pub mod prelude {
    pub use crate::{
        error::{self, Error},
        middleware::{MiddleResult, Middleware},
        Content, Cookie, Header, HeaderType, Method, Query, Request, Response, Server, SetCookie,
        Status,
    };
}

// Extra Features
#[cfg(feature = "extensions")]
mod extensions;
#[cfg(feature = "extensions")]
pub mod extension {
    //! Useful extensions to the base afire.
    //! Includes helpful middleware like Serve Static, Rate Limit and Logger.
    //!
    //! ## All Feature
    //! | Name            | Description                                           |
    //! | --------------- | ----------------------------------------------------- |
    //! | [`ServeStatic`] | Serve static files from a dir.                        |
    //! | [`Date`]        | Add the Date header to responses. Required by HTTP.   |
    //! | [`RateLimiter`] | Limit how many requests can be handled from a source. |
    //! | [`Logger`]      | Log incoming requests to the console / file.          |
    //! | [`RequestId`]   | Add a Request-Id header to all requests.              |
    pub use crate::extensions::{
        date::{self, Date},
        logger::{self, Logger},
        ratelimit::RateLimiter,
        request_id::RequestId,
        serve_static::{self, ServeStatic},
    };
}
