use crate::{Header, Response, SetCookie};

#[test]
fn response_new() {
    let response = Response::new();

    assert_eq!(
        response,
        Response {
            status: 200,
            data: vec![79, 75],
            headers: vec![],
            reason: None,
            close: false
        }
    );
}

#[test]
fn response_default() {
    assert_eq!(Response::new(), Response::default());
}

#[test]
fn response_status() {
    let response = Response::new().status(100);

    assert_eq!(response.status, 100);
}

#[test]
fn response_reason() {
    let response = Response::new().reason("Good");

    assert_eq!(response.reason, Some("Good".to_owned()));
}

#[test]
fn response_text() {
    let response = Response::new().text("Hello World");

    assert_eq!(
        response.data,
        vec![72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
    );
}

#[test]
fn response_bytes() {
    let response = Response::new().bytes(vec![100]);

    assert_eq!(response.data, vec![100]);
}

#[test]
fn response_header() {
    let response = Response::new()
        .header("Name", "Value")
        .header("Hello", "World");

    assert_eq!(
        response.headers,
        vec![Header::new("Name", "Value"), Header::new("Hello", "World")]
    );
}

#[test]
fn response_headers() {
    let response = Response::new().headers(vec![
        Header::new("Name", "Value"),
        Header::new("Hello", "World"),
    ]);

    assert_eq!(
        response.headers,
        vec![Header::new("Name", "Value"), Header::new("Hello", "World")]
    );
}

#[test]
fn response_close() {
    let response = Response::new().close();

    assert!(response.close);
}

#[test]
fn response_cookie() {
    let response = Response::new().cookie(SetCookie::new("Name", "Value"));

    assert_eq!(
        response.headers,
        vec![Header::new("Set-Cookie", "Name=Value;")]
    );
}

#[test]
fn response_cookies() {
    let response = Response::new().cookies(vec![
        SetCookie::new("Name", "Value"),
        SetCookie::new("Hello", "World"),
    ]);

    assert_eq!(
        response.headers,
        vec![
            Header::new("Set-Cookie", "Name=Value;"),
            Header::new("Set-Cookie", "Hello=World;")
        ]
    );
}
