# 🔥 afire <a href="https://github.com/Basicprogrammer10/afire/actions"><img src="https://github.com/Basicprogrammer10/afire/actions/workflows/rust.yml/badge.svg"></a> <a href="https://crates.io/crates/afire"><img alt="Crates.io" src="https://img.shields.io/crates/v/afire"> <img src="https://img.shields.io/crates/d/afire?label=Downloads"></a>

afire is a _blazingly fast_ web server micro framework for rust.

## 💠 Install

Just add the following to your `Cargo.toml`:

```toml
[dependencies]
afire = "1.3.0"
```

## 📄 Info

afire is a simple synchronous multithreaded express.js inspired rust web micro framework.
wow that was long.
It comes with some built extensions in for Static File Serving, Rate limiting, more.

Below you can fine links to some afire related resources.

- [Homepage](https://connorcode.com/writing/afire)
- [Crates.io page](https://crates.io/crates/afire)
- [API Docs](https://docs.rs/afire/latest/afire/)

## 💥 Example

For more examples see the examples directory [here](https://github.com/Basicprogrammer10/afire/tree/main/examples).

Below is a super simple example so you can see the basics of aire syntax.

```rust no_run
// Import Lib
use afire::{Server, Method, Response, Header, Content};

// Create Server
let mut server = Server::<()>::new("localhost", 8080);

// Add a route
server.route(Method::GET, "/greet/{name}", |req| {
  let name = req.param("name").unwrap();

  Response::new()
    .text(format!("Hello, {}", name))
    .content(Content::TXT)
});

// Start the server
// This is blocking
server.start().unwrap();
```

## 💼 License

afire is licensed under the MIT license so you are free to do basically whatever you want with it as long as you add a copyright notice.
You can read the full license text [here](https://github.com/Basicprogrammer10/afire/blob/main/LICENSE).
