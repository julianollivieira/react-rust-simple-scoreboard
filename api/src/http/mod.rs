mod controllers;
pub mod error;
mod requests;

use crate::env::EnvConfig;
use anyhow::{Context, Result};
use axum::{
    http::{header, HeaderValue, Method},
    Extension, Server,
};
use controllers::players;
use sqlx::{Pool, Postgres};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

// Context with a database connection pool to be shared with all handlers.
#[derive(Clone)]
pub struct ApiContext {
    pool: Pool<Postgres>,
}

pub async fn serve(config: EnvConfig, pool: Pool<Postgres>) -> Result<()> {
    // Create a SocketAddr to bind the HTTP server to.
    let ip_addr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));
    let socket_addr = SocketAddr::new(ip_addr, config.api_port);

    // Create context for our API.
    let api_context = ApiContext { pool };

    // Create app url HeaderValue for CORS.
    let app_url_header = HeaderValue::from_str(&config.app_url)?;

    // Set CORS headers for all responses.
    let cors_layer = CorsLayer::new()
        .allow_origin(app_url_header)
        .allow_methods(vec![
            Method::POST,
            Method::GET,
            Method::DELETE,
            Method::OPTIONS,
            Method::PATCH,
        ])
        .allow_headers(vec![header::CONTENT_TYPE]);

    // Initialize the routes and share the context with all handlers.
    let app = players::router()
        .layer(cors_layer)
        .layer(ServiceBuilder::new().layer(Extension(api_context)));

    // Bind the server to the socket address.
    Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await
        .context("Failed to bind to socket")?;

    Ok(())
}
