use axum::{
    extract::{Path, WebSocketUpgrade},
    routing::get,
    ServiceExt,
};
use dioxus_fullstack::prelude::*;
use tower::ServiceExt as OtherServiceExt;
use tower_http::services::ServeDir;

pub fn launch() {
    const ADDR: &str = "[::]:8080";

    if dotenvy::dotenv().is_err() {
        log::warn!(".env file not found, continuing without loading")
    }

    log::info!("server launching");
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            log::info!("listening on {}", ADDR);
            axum::Server::bind(&ADDR.parse().unwrap())
                .serve(
                    axum::Router::new()
                        .nest_service("/", ServeDir::new("dist"))
                        .nest_service("/images", ServeDir::new("images"))
                        .register_server_fns("/api")
                        .map_response(|mut response| {
                            response.headers_mut().insert(
                                "Cross-Origin-Opener-Policy",
                                "same-origin".parse().unwrap(),
                            );
                            response.headers_mut().insert(
                                "Cross-Origin-Embedder-Policy",
                                "require-corp".parse().unwrap(),
                            );
                            response
                        })
                        .into_make_service(),
                )
                .await
                .unwrap()
        });
}