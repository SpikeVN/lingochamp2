use axum::response::{Html, IntoResponse};
use axum::routing::get;
use axum::Server;
use socketioxide::SocketIo;
use std::net::SocketAddr;
use tokio::runtime::Builder;

use crate::networking::handlers;

async fn serve_html() -> impl IntoResponse {
    Html(include_str!("../../templates/login.html"))
}

pub async fn sws(addr: &SocketAddr) {
    let (layer, io) = SocketIo::new_layer();
    io.ns("/", handlers::handler);

    let app = axum::Router::new()
        .route("/", get(serve_html))
        .layer(layer)
        .with_state(io);

    let _ = Server::bind(addr).serve(app.into_make_service()).await;
}

pub fn start_web_service(addr: &SocketAddr) {
    let runtime = Builder::new_current_thread().enable_all().build().unwrap();

    runtime.block_on(sws(addr));
}
