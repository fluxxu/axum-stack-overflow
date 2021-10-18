use axum::{
    handler::{get, post},
};
use axum::routing::{BoxRoute, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(get_router().into_make_service())
        .await
        .unwrap();
}

fn get_router() ->  Router<BoxRoute> {
    let mut router = Router::new().boxed();

    for i in 0..282 {
        router = router.route(&format!("/get{}", i), axum::handler::get(move || {
            async move {
                format!("i = {}", i)
            }
        })).boxed();
    }

    router
}