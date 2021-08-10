/*!
# 🔥 afire <a href="https://github.com/Basicprogrammer10/afire/actions"><img src="https://img.shields.io/github/workflow/status/Basicprogrammer10/afire/CI?label=Tests"></a> <a href="https://www.codefactor.io/repository/github/basicprogrammer10/watertemp"><a href="#"><img src="https://img.shields.io/tokei/lines/github/Basicprogrammer10/afire?label=Total%20Lines"></a>
A blazing fast web framework for Rust

Work in progress :P
*/

use std::io::prelude::*;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;

/// Defines a server.
pub struct Server {
    /// Port to listen on.
    pub port: u16,

    /// Ip address to listen on.
    pub ip: [u8; 4],

    /// Routes to handle.
    pub routes: Vec<Route>,

    // Optional stuff
    /// Run server
    run: bool,

    /// Headders automatically added to every response.
    default_headers: Option<Vec<Header>>,
}

/// Defines a route.
pub struct Route {
    method: Method,
    path: String,
    handler: fn(Request) -> Response,
}

/// Methods for a request
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
    TRACE,

    /// Custom request
    CUSTOM(String),

    /// For routes that run on all methods
    ///
    /// Will not be use in a request
    ANY,
}

/// Http header
///
/// Has a name and a value.
pub struct Header {
    name: String,
    value: String,
}

/// Http Request
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Vec<Header>,
    pub body: Vec<u8>,
}

/// Http Responce
pub struct Response {
    pub status: u16,
    pub data: String,
    pub headers: Vec<Header>,
}

/// Implamantaions for Server
impl Server {
    /// Creates a new server.
    ///
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::Server;
    ///
    /// // Create a server for localhost on port 8080
    /// // Note: The server has not been started yet
    /// let mut server: Server = Server::new("localhost", 8080);
    /// ```
    pub fn new(mut raw_ip: &str, port: u16) -> Server {
        let mut ip: [u8; 4] = [0; 4];

        // If the ip is localhost, use the loopback ip
        if raw_ip == "localhost" {
            raw_ip = "127.0.0.1";
        }

        // Parse the ip to an array
        let splitted_ip: Vec<&str> = raw_ip.split('.').collect();

        if splitted_ip.len() != 4 {
            panic!("Invalid Server IP");
        }
        for i in 0..4 {
            let octet: u8 = splitted_ip[i].parse::<u8>().expect("Invalid Server IP");
            ip[i] = octet;
        }

        Server {
            port: port,
            ip: ip,
            routes: Vec::new(),
            run: true,
            default_headers: Some(vec![Header::new("Powerd-By", "afire")]),
        }
    }

    /// Start the server.
    ///
    /// Will be blocking.
    ///
    /// ## Example
    /// ```rust
    /// // Import Library
    /// use afire::{Server, Response, Header};
    ///
    /// // Starts a server for localhost on port 8080
    /// let mut server: Server = Server::new("localhost", 8080);
    ///
    /// // Define a route
    /// server.get("/", |req| {
    ///     Response::new(
    ///         200,
    ///         "N O S E",
    ///         vec![Header::new("Content-Type", "text/plain")],
    ///     )
    /// });
    ///
    /// // Starts the server
    /// // This is blocking
    /// # // Keep the server from strarting and blocking the main thread
    /// # server.set_run(false);
    /// server.start();
    /// ```
    pub fn start(&self) {
        // Exit if the server should not run
        if !self.run {
            return;
        }

        let listener = TcpListener::bind(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                self.ip[0], self.ip[1], self.ip[2], self.ip[3],
            )),
            self.port,
        ))
        .unwrap();

        for event in listener.incoming() {
            // Read stream into buffer
            let mut stream = event.unwrap();

            // Get the reponse from the handler
            // Uses the most recently defined route that matches the request
            let mut res = self.handle_connection(&stream);

            // Add default headers to response
            if self.default_headers.is_some() {
                for header in self.default_headers.as_ref().unwrap() {
                    res.headers.push(Header::copy(header));
                }
            }

            // Add content-length header to response
            res.headers
                .push(Header::new("Content-Length", &res.data.len().to_string()));

            // Convert the response to a string
            let response = format!(
                "HTTP/1.1 {} OK\r\n{}\r\n\r\n{}",
                res.status,
                headers_to_string(res.headers),
                res.data
            );

            // Send the response
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }

    /// Handel a connection to the server
    fn handle_connection(&self, mut stream: &TcpStream) -> Response {
        // Init Buffer
        let mut buffer = [0; 1024];

        // Read stream into buffer
        stream.read(&mut buffer).unwrap();

        let stream_string = str::from_utf8(&buffer).expect("Error parseing buffer data");

        // Loop through all routes and check if the request matches
        for route in self.routes.iter().rev() {
            let req_method = get_request_method(stream_string.to_string());
            let req_path = get_request_path(stream_string.to_string());
            if &req_method == &route.method || route.method == Method::ANY && req_path == route.path
            {
                // TODO: Send Header and Body here
                let req = Request::new(req_method, &req_path, Vec::new(), Vec::new());
                return (route.handler)(req);
            }
        }
        return Response::new(
            404,
            "Not Found",
            vec![Header::new("Content-Type", "text/plain")],
        );
    }

    /// Keep a server from starting
    ///
    /// Only used for testing
    ///
    /// It would be a really dumb idea to use
    pub fn set_run(&mut self, run: bool) {
        self.run = run;
    }

    /// Create a new route for get requests
    pub fn get(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.routes.push(Route {
            method: Method::GET,
            path: path.to_string(),
            handler: handler,
        });
    }

    /// Create a new route for any type of request
    pub fn any(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.routes.push(Route {
            method: Method::ANY,
            path: path.to_string(),
            handler: handler,
        });
    }
}

impl Response {
    /// Quick and easy way to create a response.
    pub fn new(status: u16, data: &str, headers: Vec<Header>) -> Response {
        Response {
            status,
            data: data.to_string(),
            headers: headers,
        }
    }
}

impl Request {
    /// Quick and easy way to create a request.
    pub fn new(method: Method, path: &str, headers: Vec<Header>, body: Vec<u8>) -> Request {
        Request {
            method,
            path: path.to_string(),
            headers,
            body,
        }
    }
}

impl PartialEq for Method {
    /// Allow compatring Method Enums
    ///
    /// EX: Method::GET == Method::GET
    ///
    /// > True
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl Header {
    /// Make a new header
    pub fn new(name: &str, value: &str) -> Header {
        Header {
            name: name.to_string(),
            value: value.to_string(),
        }
    }

    /// Convert a header ref to a header
    pub fn copy(header: &Header) -> Header {
        Header {
            name: header.name.clone(),
            value: header.value.clone(),
        }
    }

    /// Convert a header to a string
    ///
    /// `name: value`
    pub fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.value)
    }

    /// Convert a string to a header
    ///
    /// String must be in the format `name: value`
    pub fn from_string(header: &str) -> Option<Header> {
        let splitted_header: Vec<&str> = header.split(':').collect();
        if splitted_header.len() != 2 {
            return None;
        }
        Some(Header {
            name: splitted_header[0].trim().to_string(),
            value: splitted_header[1].trim().to_string(),
        })
    }
}

/// Stringify a Vec of headers
///
/// Each header is in the format `name: value`
///
/// Every header is separated by a newline (`\r\n`)
fn headers_to_string(headers: Vec<Header>) -> String {
    let headers_string: Vec<String> = headers.iter().map(|header| header.to_string()).collect();
    format!("{}", headers_string.join("\r\n"))
}

/// Get the request method of a raw HTTP request.
fn get_request_method(raw_data: String) -> Method {
    let method_str = raw_data
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .next()
        .unwrap()
        .to_string();

    return match &method_str[..] {
        "GET" => Method::GET,
        "POST" => Method::POST,
        "PUT" => Method::PUT,
        "DELETE" => Method::DELETE,
        "OPTIONS" => Method::OPTIONS,
        "HEAD" => Method::HEAD,
        "PATCH" => Method::PATCH,
        "TRACE" => Method::TRACE,
        _ => Method::CUSTOM(method_str),
    };
}

/// Get the path of a raw HTTP request.
fn get_request_path(raw_data: String) -> String {
    let path_str = raw_data.split(" ").collect::<Vec<&str>>();
    path_str[1].to_string()
}
