#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;

use axum::{
    body::Body,
    handler::{get, post},
    http::{header, HeaderValue},
    routing::BoxRoute,
    AddExtensionLayer, Router,
};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::{set_header::SetResponseHeaderLayer, trace::TraceLayer};

mod error;
mod extractors;
mod handlers;
mod model;
mod sql;
mod utils;

pub mod config;

pub fn app(pg_pool: PgPool) -> Router<BoxRoute> {
    let middleware_stack = ServiceBuilder::new()
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::<_, Body>::overriding(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::overriding(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("GET, HEAD, POST, OPTIONS"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::overriding(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        ))
        .layer(SetResponseHeaderLayer::<_, Body>::overriding(
            header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        ))
        .layer(AddExtensionLayer::new(pg_pool))
        .into_inner();

    Router::new()
        .route("/login", post(handlers::login))
        .route("/register", post(handlers::register))
        .route("/authorize", get(handlers::authorize))
        .layer(middleware_stack)
        .boxed()
}
