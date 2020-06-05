use std::convert::Infallible;
use std::error::Error;

use chrono::Local;
use hyper::{Body, Request, Response};

async fn extract_body(request: Request<Body>) -> Result<String, Box<dyn Error>> {
    let body = request.into_body();
    let data = hyper::body::to_bytes(body).await?;
    // TODO: Avoid copying (`into_owned`)
    let data = String::from_utf8_lossy(data.as_ref()).into_owned();
    Ok(data)
}

pub async fn process(request: Request<Body>) -> Result<Response<Body>, Infallible> {
    let timestamp = Local::now();

    let endpoint = request.uri().path();

    println!("{} {}", timestamp, endpoint);

    if let Some(query) = request.uri().query() {
        println!("{} ┗━> Query: {}", timestamp, query);
    };

    if let Ok(body) = extract_body(request).await {
        if !body.trim().is_empty() {
            println!("{} ┗━> Body: {}", timestamp, body);
        }
    };

    println!();

    Ok(Response::new("OK".into()))
}
