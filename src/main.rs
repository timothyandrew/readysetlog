use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::env;
use std::net::SocketAddr;
use futures::future::join_all;
use tokio::task::JoinHandle;
use std::error::Error;

fn api_server(port: u16) -> JoinHandle<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(readysetlog::process)) });

    let server = Server::bind(&addr).serve(make_svc);
    
    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("API server error: {}", e);
        }
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = match env::args().nth(1) {
        Some(port) => port.parse::<u16>().unwrap(),
        _ => 7750,
    };

    println!("API server listening on port {} ✅", port);

    let services = vec![ api_server(port)]; 
    join_all(services).await;

    Ok(())

    // TODO
    // - [x] Log all incoming requests to the command line
    // - [ ] Highlight a given string/JSON key/query param (color codes)
    // - [ ] Optionally pretty-print JSON bodies
    // - [ ] Display all incoming requests on a web page
    // - [ ] Allow grouping by endpoint
    // - [ ] Websockets + live updates
}
