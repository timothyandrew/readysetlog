
use std::collections::{HashMap};
use std::convert::Infallible;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::ws;

use warp::Filter;

fn optional_raw_query_params() -> impl Filter<Extract = (String,), Error = Infallible> + Clone {
    warp::filters::query::raw()
        .or(warp::any().map(String::default))
        .unify()
}

fn http_server() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let html = warp::get().and(warp::path::end()).map(readysetlog::html);
    let js = warp::path("main.js").and(warp::path::end()).map(readysetlog::js);
    html.or(js)
}

fn api_server(
    state: readysetlog::State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::filters::path::full())
        .and(optional_raw_query_params())
        .and(warp::body::bytes())
        .and(warp::any().map(move || state.clone()))
        .and_then(readysetlog::api)
}

fn ws_server(
    state: readysetlog::State,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::ws())
        .and(warp::any().map(move || state.clone()))
        .map(|ws: ws::Ws, state: readysetlog::State| ws.on_upgrade(|ws| readysetlog::ws(ws, state)))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = match env::args().nth(1) {
        Some(port) => port.parse::<u16>().unwrap(),
        _ => 7750,
    };

    println!(" API server -> :{}", port);
    println!("HTTP server -> :{}", port + 1);
    println!("  WS server -> :{}", port + 2);

    let state: readysetlog::State = Arc::new(Mutex::new(HashMap::new()));

    tokio::join!(
        warp::serve(api_server(state.clone())).run(([127, 0, 0, 1], port)),
        warp::serve(http_server()).run(([127, 0, 0, 1], port + 1)),
        warp::serve(ws_server(state.clone())).run(([127, 0, 0, 1], port + 2))
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
