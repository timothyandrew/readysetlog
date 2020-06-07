use chrono::Local;
use futures::{FutureExt, StreamExt};
use warp::Filter;

use warp::filters::path::FullPath;
use warp::reply::Reply;

fn extract_body(body: bytes::Bytes) -> String {
    // TODO: Avoid copying (`into_owned`)
    String::from_utf8_lossy(body.as_ref()).into_owned()
}

pub async fn process_ws(ws: warp::ws::WebSocket) {
    println!("WS conn established!");

    let (tx, rx) = ws.split();
    if let Err(_) = rx.take(5).forward(tx).await {
        panic!("WS connection failed!")
    }

    println!("Done!")
}

pub fn process(endpoint: FullPath, query: String, body: bytes::Bytes) -> impl Reply {
    let timestamp = Local::now();

    println!("{} {}", timestamp, endpoint.as_str());

    if !query.trim().is_empty() {
        println!("{} ┗━> Query: {}", timestamp, query);
    }

    let body = extract_body(body);
    if !body.trim().is_empty() {
        println!("{} ┗━>  Body: {}", timestamp, body);
    }

    println!();

    "OK!"
}
