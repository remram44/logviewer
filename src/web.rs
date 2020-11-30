use warp::Filter;
use warp::path;
use warp::reply::Reply;
use warp::reply::with::header;

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
        .or(path("bundle.js").and(path::end())
            .map(|| include_bytes!("../ui.dist/bundle.js") as &[u8])
            .with(header("Content-Type", "text/javascript")))
        // Log query
        .or(path("api").and(path("query")).and(path::end())
            .map(query))
    ;

    eprintln!("Starting server on {}:{}", host, port);
    warp::serve(routes).run((host, port)).await;
}

fn query() -> impl Reply {
    "{}"
}
