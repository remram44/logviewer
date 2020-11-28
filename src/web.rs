use warp::Filter;
use warp::path;
use warp::reply::Reply;

fn main() {
    let mut runtime = tokio::runtime::Builder::new()
        .basic_scheduler()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(serve([127, 0, 0, 1].into(), 8000));
}

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
