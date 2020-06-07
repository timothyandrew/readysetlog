use chrono::Local;

use warp::filters::path::FullPath;
use warp::reply::Reply;

fn extract_body(body: bytes::Bytes) -> String {
    // TODO: Avoid copying (`into_owned`)
    String::from_utf8_lossy(body.as_ref()).into_owned()
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
