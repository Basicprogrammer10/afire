//! Middleware is code that runs before and after the routes.
//! They can be used to Log Requests, Ratelimit Requests, add Analytics, etc.

use std::any::type_name;

use crate::{error::Result, Request, Response, Server};

/// Middleware `post` Responses
pub enum MiddleResponse {
    /// Dont affect the Response
    Continue,

    /// Change the Response and continue to run Middleware (if any)
    Add(Response),

    /// Send Response immediately
    Send(Response),
}

/// Middleware `pre` Responses
///
/// Works with the Request
pub enum MiddleRequest {
    /// Dont affect the Request
    Continue,

    /// Change the Request and continue to run Middleware (if any) then routes
    Add(Request),

    /// Send a Response immediately
    Send(Response),
}

/// Middleware
pub trait Middleware {
    /// Middleware to run Before Routes
    fn pre(&self, _req: &Result<Request>) -> MiddleRequest {
        MiddleRequest::Continue
    }

    /// Middleware to run After Routes
    fn post(&self, _req: &Result<Request>, _res: &Result<Response>) -> MiddleResponse {
        MiddleResponse::Continue
    }

    /// Middleware ot run after the response has been handled
    fn end(&self, _req: &Result<Request>, _res: &Response) {}

    /// Attatch Middleware to a Server
    fn attach<State>(self, server: &mut Server<State>)
    where
        Self: 'static + Send + Sync + Sized,
        State: 'static + Send + Sync,
    {
        trace!("📦 Adding Middleware {}", type_name::<Self>());

        server.middleware.push(Box::new(self));
    }
}
