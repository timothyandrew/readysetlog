use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port = match env::args().skip(1).next() {
        Some(port) => port.parse::<u16>().unwrap(),
        _ => 7750,
    };

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(readysetlog::process)) });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on port {} ✅", port);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }

    // TODO
    // - [ ] Log all incoming requests to the command line
    // - [ ] Display all incoming requests on a web page
    // - [ ] Allow grouping by endpoint
    // - [ ] Websockets + live updates
}
