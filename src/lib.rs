use chrono::Local;
use futures::{SinkExt, StreamExt};



use futures::stream::SplitSink;
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::filters::path::FullPath;
use warp::filters::ws::Message;
use warp::filters::ws::WebSocket;
use warp::reply::html;
use warp::reply::Reply;

static ID_GENERATOR: AtomicUsize = AtomicUsize::new(1);

#[allow(dead_code)]
static HTML_BODY: &str = include_str!("../html/index.html");

// Can't use a `HashSet` because `SplitSink` doesn't implement `std::hash::Hash`
pub type State = Arc<Mutex<HashMap<usize, SplitSink<WebSocket, Message>>>>;

fn extract_body(body: bytes::Bytes) -> String {
    // TODO: Avoid copying (`into_owned`)
    String::from_utf8_lossy(body.as_ref()).into_owned()
}

#[cfg(debug_assertions)]
fn web_response() -> String {
    let mut f = File::open("html/index.html").unwrap();
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).unwrap();
    buffer
}

#[cfg(not(debug_assertions))]
fn web_response() -> String {
    HTML_BODY.to_string()
}

pub fn web() -> impl Reply {
    html(web_response())
}

pub async fn ws(ws: warp::ws::WebSocket, state: State) {
    let (mut tx, mut rx) = ws.split();

    let ws_id = ID_GENERATOR.fetch_add(1, Ordering::Relaxed);
    tx.send(Message::text("Connected!")).await.unwrap();

    {
        let mut state = state.lock().await;
        (*state).insert(ws_id, tx);
    }

    while let Some(_) = rx.next().await {}

    let mut state = state.lock().await;
    (*state).remove(&ws_id);
}

pub async fn api(
    endpoint: FullPath,
    query: String,
    body: bytes::Bytes,
    state: State,
) -> Result<impl Reply, warp::Rejection> {
    let mut serialized = String::new();
    let timestamp = Local::now();

    serialized.push_str(&format!("{} {}\n", timestamp, endpoint.as_str()));

    if !query.trim().is_empty() {
        serialized.push_str(&format!("{} ┗━> Query: {}\n", timestamp, query));
    }

    let body = extract_body(body);
    if !body.trim().is_empty() {
        serialized.push_str(&format!("{} ┗━>  Body: {}\n", timestamp, body));
    }

    let mut state = state.lock().await;
    for (_, tx) in (*state).iter_mut() {
        tx.send(Message::text(serialized.as_str())).await.unwrap();
    }

    println!("{}", serialized);

    Ok("OK!")
}
