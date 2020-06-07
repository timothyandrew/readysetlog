use futures::future::join_all;
use std::convert::Infallible;
use std::env;
use std::error::Error;
use warp::ws;

use warp::Filter;

fn optional_raw_query_params() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::filters::query::raw()
        .or(warp::any().map(String::default))
        .unify()
}

fn api_server() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::filters::path::full())
        .and(optional_raw_query_params())
        .and(warp::body::bytes())
        .map(readysetlog::process)
}

fn ws_server() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::ws())
        .map(|ws: ws::Ws| ws.on_upgrade(readysetlog::process_ws))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = match env::args().nth(1) {
        Some(port) => port.parse::<u16>().unwrap(),
        _ => 7750,
    };

    println!("API server listening on port {} ✅", port);
    println!("WS server listening on port {} ✅", port + 1);

    tokio::join!(
        warp::serve(api_server()).run(([127, 0, 0, 1], port)),
        warp::serve(ws_server()).run(([127, 0, 0, 1], port + 1))
    );

    Ok(())

    // TODO
    // - [x] Log all incoming requests to the command line
    // - [ ] Highlight a given string/JSON key/query param (color codes)
    // - [ ] Optionally pretty-print JSON bodies
    // - [ ] Display all incoming requests on a web page
    // - [ ] Allow grouping by endpoint
    // - [ ] Websockets + live updates
}
