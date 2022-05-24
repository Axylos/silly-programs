use std::net::{SocketAddr, TcpListener};

async fn bind_server() {
    let listener = TcpListener::bind("0.0.0.0:8081".parse::<SocketAddr>().unwrap()).unwrap();
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
