use serde_derive::Serialize;
use std::convert::Infallible;
use warp::{Filter, Rejection};
use warp::path;
use warp::http::StatusCode;
use warp::reply::{Reply, json};
use warp::reply::with::header;

use crate::filters::View;
use crate::readers::LogFile;
use crate::{Record, process};

pub async fn serve(
    host: std::net::IpAddr,
    port: u16,
) {
    let routes =
        // Index, show interface
        path::end()
            .map(|| include_bytes!("../ui.dist/index.html") as &[u8])
            .with(header("Content-Type", "text/html; charset=utf-8"))
        // Javascript
        .or(
            path("bundle.js").and(path::end())
                .map(|| include_bytes!("../ui.dist/bundle.js") as &[u8])
                .with(header("Content-Type", "text/javascript"))
        )
        // CSS
        .or(
            path("index.css").and(path::end())
                .map(|| include_bytes!("../ui.dist/index.css") as &[u8])
                .with(header("Content-Type", "text/css"))
        )
        // API
        .or(
            path("api").and(
                // Log query
                warp::post().and(path("query")).and(path::end())
                    .and(warp::body::json())
                    .and_then(query)
            ).recover(api_error)
        );

    eprintln!("Starting server on {}:{}", host, port);
    warp::serve(routes).run((host, port)).await;
}

#[derive(Debug)]
enum QueryError {
    InternalError(Box<dyn std::error::Error + Send + Sync>),
}

impl warp::reject::Reject for QueryError {}

#[derive(Serialize)]
struct ErrorMessage<'a> {
    code: u16,
    message: &'a str,
}

async fn api_error(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message: std::borrow::Cow<str>;
    if let Some(QueryError::InternalError(e)) = err.find() {
        code = StatusCode::BAD_REQUEST;
        message = format!("Error: {}", e).into();
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        code = StatusCode::BAD_REQUEST;
        message = format!("{}", e).into();
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "Wrong method".into();
    } else if let Some(_) = err.find::<warp::reject::UnsupportedMediaType>() {
        code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        message = "Input is not JSON".into();
    } else {
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "Internal error".into();
    }
    Ok(warp::reply::with_status(
        json(&ErrorMessage {
            code: code.as_u16(),
            message: message.as_ref(),
        }),
        code,
    ))
}

#[derive(Serialize)]
struct Response {
    records: Vec<Record>,
}

async fn query(view: View) -> Result<impl Reply, Rejection> {
    let mut records = Vec::new();
    let file = LogFile::open("test.log").expect("Test file");
    for record in process(file, view) {
        let record = record
            .map_err(|e| warp::reject::custom(QueryError::InternalError(Box::new(e))))?;
        records.push(record);
    }
    Ok(json(&Response { records }))
}
