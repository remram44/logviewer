use warp::Filter;
use warp::path;
use warp::reply::Reply;

pub async fn serve(
    host: std::net::IpAddr,
    port: u16,
) {
    let routes =
        // Index, show interface
        path::end().map(index)
        // Log query
        .or(path("api").and(path("query")).and(path::end())
            .map(query))
    ;

    eprintln!("Starting server on {}:{}", host, port);
    warp::serve(routes).run((host, port)).await;
}

fn index() -> impl Reply {
    "hello"
}

fn query() -> impl Reply {
    "{}"
}
