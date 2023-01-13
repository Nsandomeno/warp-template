use warp::Filter;

async fn hello(param: String) -> Result<impl warp::Reply, warp::Rejection> {
    Ok(format!("Hello {}", param))
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::get()
        .and(warp::path("hello"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and_then(hello);

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
