# 1.2.0

June 24, 2022

- oh windows,,,
- Fix Path Traversal on windows
- Use AsRef<str> more instead of Display
- Add a serve path to Serve Static
- Serve index from serve path
- Remove the `ignore_trailing_path_slash` feature
- Redo Internal Error handling system
- Middleware use references to Requests and Responses and stuff
- Improve built-in serve_static middleware
- Re organize extension stuff
- RateLimit use RwLock
- Add Request ID Middleware
- Server Wide State
- Add Cache Middleware
- Remove insane build script
- When building http response only add Content-Length and default headers if they are not already present
- Add server state syste
- Improved Request Parsing
- Redo Error system
- Remove the requests raw_data feild
- Remove Request::body_string in favor of String::from_utf8()
- Fix HTTP parseing and genatation issues

# 1.1.0

Apr 10, 2022

- Update Path Matcher to support AnyAfter segments (\*\*)
- Remove Test Example
- Add Paste Bin App Example
- Add SocketHandler struct to hold socket interacting functions
- Fix Path Traversal Exploit O_O

# 1.0.0!

Mar 14, 2022

- Add ThreadPool Back!
- Tracing Feature
- Remove Middleware Interior Mutability by Default
- Make remove_address_port usable without Feature
- Add _end_ middleware to run after sending a request
- Make use of end middleware on logger

# 0.4.0

Feb 19, 2022

- Make serve static types public
- Fix File Uploading stuff
- Add a Prelude for afire essentials
- Optimize HTTP parser _(saving nanoseconds!)_
- More unit tests
- Middleware Error Handling!
- Make Route and Error Handler functions take closures
- Remove deprecated .all function (deprecated in 0.3.0)
- Remove deprecated .ip_string function (deprecated in 0.3.0)
- Finally remove deprecated .any function (deprecated in 0.1.5)
- Route take AsRef<str> instead of any Displayable Type
- Now .header takes in a key and value rather than a Header struct

# 0.3.0

Jan 25, 2022 s

- Add content types
- Update Logger syntax
- Allow changing socket buffer size
- Switch Server to more of a Builder
- Add Path parameters
- Redo Path Handling
- Update README Example
- Deprecate `.all` routes, Use .route(Method::Any, "\*", ...)
- Make `content_type` a built-in (not an extension)
- Rename 'path pram' to Path Parameters
- Update Data Example with Path Parameters
- Remove Threading Example
- Update Rate limit to be a Builder
- Update Rate limit Example
- Remove thread pool from project (for now)
- Make Custom Content Type use &str not String
- Make VERSION Public
- Add External Unit Testsremove_address_port
- Use the built-in `IpAddr` enum for server Ip
- Remove `.ip_string()` for `.ip.to_string()`
- Add `Response.close()` for closing a stream with no response
- Completely Redo Middleware, Now it can modify Requests / Responses
- Removed Server.middleware()
- Export Internal Functions
- Supply the Request to the Middleware Post Function

# 0.2.2

Dec 04, 2021

- Remove Debug Print Left in...
- Small changes to features
- This is mostly about the Debug Print

# 0.2.1

Dec 04, 2021

- Only Build common::remove_address_port if logger or rate-limiter are enabled
- Make Header name / value Public
- Serve Static Middleware
- Make Routes use Closures
- Remove Threadpool (for now)
- Make Error handler use a closure
- Rename `set_error_handler` to `error_handler`
- Rename `set_socket_timeout` to `socket_timeout`
- Update Serve Static Example to use Middleware
- Allow for Manually setting the reason phrase
- Support URL encoded cookies
- Rename `add_default_header` to `default_header`
- Store Raw Request data and Request body as `Vec<u8>`
- Fix Panic Handler feature compile problems
- Dont use an Option for Vec of default headers
- Fix Header Parsing
- Add a `header` method on Request to get headers

# 0.2.0

Nov 04, 2021

- Response Overhaul, Now more like a Response Builder
- Update _every_ example with new syntax...
- Small improvement to Query parsing
- Update SetCookie Function Names
- Update Cookie Example
- Add a Build Script to write the Readme from the docstring in lib.rs
- Use Generics for more functions that take &str / Strings to just impl Display
- Rename .every to .middleware
- Update Readme in /examples
- Add a dynamic buffer resize feature
- Update Logger Middleware to be a builder
- Add Path parameter Example in 04_data

# 0.1.7

Oct 27, 2021

- Add Panic Message to Error Handel
- Add http.rs to move raw http parsing out of server.rs
- Start / Start Threaded returns Option
- Add .unwrap to all server.starts in examples
- Add http.rs to move raw http parsing out of server.rs
- Dont give up on cookie parsing if cookie header is malformed
- Add optional Socket Timeout
- Add Socket Timeout Docs

# 0.1.6

Oct 20, 2021

- Add Example for Logging
- Add Example for Rate Limiter
- Improve Rate limiter
- Add More Function Docs
- Show Query data in Info Logger
- Ignore extra slashes in path
- Remove nose.txt... don't know how that got there :P
- Don't unwrap stream.read, ignore errors like a good programmer
- Fix Routing Issue
- Ignore Case in Method String
- Add different Reason Phrase for the status codes
- Update Server Header to add Version
- Cleanup Raw HTTP Parsing
- Fix / Update some examples
- Update Logger Middleware

# 0.1.5

Sep 17, 2021

- Add a route error handler
- Add `set_error_handler` fn to set the error handler
- Implement clone for more structs
- Add More Examples
- Put default headers after route headers
- Auto decode URL encoded Queries
- Update Readme
- Proper Spelling In Description
- Add support for Request Cookies
- Add Cookies to a Feature
- Add Support for Response Cookies
- Deprecate .any routes (Use `.route(Method::ANY...)` instead)
- Make Built in middleware less garbage

# 0.1.4

Sep 05, 2021

- Allow responding with bytes, not just strings
- Add Serving Favoricon as example thing

# 0.1.3

Sep 03, 2021

- Add Support for query strings in paths
- Add More docs for Query

# 0.1.2

Sep 01, 2021

- Fix a bug where '.any' routes were not working

# 0.1.1

Aug 31, 2021

- Add Optional Rate limiter _beta_
- Update readme abit
- Copy new readme to lib.rs
- Add internal docs to Ratelimit
- Add Optional Logger _beta_
- Update Keywords in Cargo.toml
- Update Version number in Cargo.toml
- Make a function to add default headers to a server
- Don't let Logger Crash in debug mode if there are no headers
- Code Cleanup

# 0.1.0

Aug 21, 2021

- Base Code
