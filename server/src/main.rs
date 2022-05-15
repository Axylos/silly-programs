use std::net::SocketAddr;
use axum::{
    routing::get,
    Router
};

use server::foo;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    foo::bar();


    let app = server::app::get_app();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

