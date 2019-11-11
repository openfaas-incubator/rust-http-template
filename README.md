# rust-http-template

An OpenFaaS of-watchdog template written in Rust.

This repository contains two Rust templates for OpenFaaS, one of which gives additional control over the HTTP request and response. They will both handle higher throughput than the classic watchdog due to the process being kept warm.

```sh
$ faas template pull https://github.com/openfaas-incubator/rust-http-template
$ faas new --list
Languages available as templates:
- rust
- rust-http
```

## rust-http-template/rust

This template takes body serialized to `Vec` of bytes as an input.

```Rust
type Error = Box<dyn std::error::Error>;

const PHRASE: &str = "Hello, World!";

pub fn handle(_body: Vec<u8>) -> Result<Vec<u8>, Error> {
    Ok(PHRASE.as_bytes().to_vec())
}

// Returns:
// Hello, World!
```

You can return custom errors using `Result`.

```Rust
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError{details: msg.to_string()}
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for MyError {
    fn description(&self) -> &str {
        &self.details
    }
}

type BoxError = Box<dyn Error>;

pub fn handle(_body: Vec<u8>) -> Result<Vec<u8>, BoxError> {
    let error = MyError::new("my error");
    Err(Box::new(error))
}


// Returns:
// {"status": "500 Internal Server Error", "description":"my error"}
```

## rust-http-template/rust-http

This template gives you more control over handling function input and output.

```Rust
use hyper::{Body, Request, Response};

const PHRASE: &str = "Hello, World!";

pub fn handle(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(PHRASE))
}

// Returns:
// Hello, World!
```

You can return custom errors using `hyper::Response`.

```Rust
use hyper::{Body, Request, Response, StatusCode};

pub fn handle(_req: Request<Body>) -> Response<Body> {
    Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("my error"))
        .unwrap()
}

// Returns:
// my error
```
