use std::net::{SocketAddr, TcpListener};
use axum::{
    routing::get,
    Router
};
use tokio::task::JoinHandle;
use tower;


use tower_http::services::ServeDir;
use server::foo;

async fn bind_server() {
    let listener = TcpListener::bind("0.0.0.0:8080".parse::<SocketAddr>().unwrap()).unwrap();
    let addr = listener.local_addr().unwrap();

    tracing::debug!("listening on {}", addr);
    let app = server::app::get_app();

        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap()
}
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();


    println!("called");
    bind_server().await;
}

