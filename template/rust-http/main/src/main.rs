use futures::future;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Request, Response, Server};

use handler;

type BoxFuture = Box<dyn Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn handler_service(req: Request<Body>) -> BoxFuture {
    let resp = handler::handle(req);
    Box::new(future::ok(resp))
}

fn main() {
    let addr = ([0, 0, 0, 0], 3000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(handler_service))
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
