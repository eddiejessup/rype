extern crate hyper;
extern crate futures;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode, Chunk, Body};
use std::ascii::AsciiExt;
use futures::Stream;
use futures::stream::Map;

struct Echo;

fn to_uppercase(chunk: Chunk) -> Chunk {
    Chunk::from(chunk
        .iter()
        .map(|byte| byte.to_ascii_uppercase())
        .collect::<Vec<u8>>())
}

impl Service for Echo {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response<Map<Body, fn(Chunk) -> Chunk>>;
    type Error = hyper::Error;
    // The future representing the eventual Response your call will
    // resolve to. This can change to whatever Future you need.
    type Future = futures::future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body("Try POSTing data to /echo");
            },
            (&Method::Post, "/echo") => {
                response.set_body(req.body().map(to_uppercase as _));
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };
        futures::future::ok(response)
    }
}

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let server = Http::new().bind(&addr, || Ok(Echo)).unwrap();
    server.run().unwrap();
}
