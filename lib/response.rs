use std::fmt::Display;

#[cfg(feature = "cookies")]
use super::cookie::SetCookie;
use super::header::Header;

/// Http Response
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Response {
    /// Response status code
    pub status: u16,

    /// Response Data as Bytes
    pub data: Vec<u8>,

    /// Response Headers
    pub headers: Vec<Header>,

    /// Response Reason
    pub reason: Option<String>,

    /// Force Close Connection
    pub close: bool,
}

impl Response {
    /// Create a new Blank Response
    ///
    /// Default data is as follows
    /// - Status: 200
    ///
    /// - Data: OK
    ///
    /// - Headers: Vec::new()
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Response, Header};
    /// // Create Response
    /// let response = Response::new();
    /// ```
    pub fn new() -> Self {
        Self {
            status: 200,
            data: vec![79, 75],
            headers: Vec::new(),
            reason: None,
            close: false,
        }
    }

    /// Add a status to a Response
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Response, Header};
    ///
    /// // Create Response
    /// let response = Response::new()
    ///    .status(200); // <- Here it is
    /// ```
    pub fn status(self, code: u16) -> Self {
        Self {
            status: code,
            ..self
        }
    }

    /// Manually set the Reason Phrase
    /// ```rust
    /// // Import Library
    /// use afire::{Response, Header};
    ///
    /// // Create Response
    /// let response = Response::new()
    ///    .reason("OK");
    /// ```
    pub fn reason<T>(self, reason: T) -> Self
    where
        T: AsRef<str>,
    {
        Self {
            reason: Some(reason.as_ref().to_owned()),
            ..self
        }
    }

    /// Add text as data to a Response
    ///
    /// Will accept any type that implements Display
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::Response;
    ///
    /// // Create Response
    /// let response = Response::new()
    ///    .text("Hi :P");
    /// ```
    pub fn text<T>(self, text: T) -> Self
    where
        T: Display,
    {
        Self {
            data: text.to_string().as_bytes().to_vec(),
            ..self
        }
    }

    /// Add raw bytes as data to a Response
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::Response;
    ///
    /// // Create Response
    /// let response = Response::new()
    ///   .bytes(vec![79, 75]);
    /// ```
    pub fn bytes(self, bytes: Vec<u8>) -> Self {
        Self {
            data: bytes,
            ..self
        }
    }

    /// Add a Header to a Response
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Response, Header};
    ///
    /// // Create Response
    /// let response = Response::new()
    ///    .header("Content-Type", "text/html");
    /// ```
    pub fn header<T, K>(self, key: T, value: K) -> Self
    where
        T: AsRef<str>,
        K: AsRef<str>,
    {
        let mut new_headers = self.headers;
        new_headers.push(Header::new(key.as_ref(), value.as_ref()));

        Self {
            headers: new_headers,
            ..self
        }
    }

    /// Add a Vec of Headers to a Response
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Response, Header};
    ///
    /// // Create Response
    /// let response = Response::new()
    ///   .headers(vec![Header::new("Content-Type", "text/html")]);
    /// ```
    pub fn headers(self, headers: Vec<Header>) -> Self {
        let mut new_headers = self.headers;
        let mut headers = headers;
        new_headers.append(&mut headers);

        Self {
            headers: new_headers,
            ..self
        }
    }

    /// Close the connection without sendng a Response
    ///
    /// Will ignore any other options defined on the Response
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Response};
    ///
    /// // Create Response
    /// let response = Response::new()
    ///   .close();
    /// ```
    pub fn close(self) -> Self {
        Self {
            close: true,
            ..self
        }
    }

    /// Add a cookie to a response.
    /// ## Example
    /// ```
    /// // Import Library
    /// use afire::{Response, SetCookie};
    ///
    /// // Create Response and add cookie
    /// let response = Response::new()
    ///     .cookie(SetCookie::new("name", "value"));
    /// ```
    #[cfg(feature = "cookies")]
    pub fn cookie(self, cookie: SetCookie) -> Self {
        let mut new = self;
        new.headers
            .push(Header::new("Set-Cookie", &cookie.to_string()));
        new
    }

    /// Add a vec of cookies to a response.
    /// ## Example
    /// ```
    /// // Import Library
    /// use afire::{Response, SetCookie};
    ///
    /// // Create Response and add cookie
    /// let response = Response::new()
    ///     .cookies(vec![SetCookie::new("name", "value")]);
    /// ```
    #[cfg(feature = "cookies")]
    pub fn cookies(self, cookie: Vec<SetCookie>) -> Self {
        let mut new = Vec::new();

        for c in cookie {
            new.push(Header::new("Set-Cookie", &c.to_string()));
        }

        self.headers(new)
    }

    /// Set a Content Type on a Response
    /// ## Example
    /// ```
    /// // Import Library
    /// use afire::{Response, Content};
    ///
    /// // Create Response and type
    /// let response = Response::new()
    ///     .content(Content::HTML);
    /// ```
    pub fn content(self, content_type: crate::Content) -> Self {
        let mut new_headers = self.headers;
        new_headers.push(Header::new("Content-Type", content_type.as_type()));

        Self {
            headers: new_headers,
            ..self
        }
    }
}

// Impl Default for Response
impl Default for Response {
    fn default() -> Response {
        Response::new()
    }
}
