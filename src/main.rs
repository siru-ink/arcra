use crate::env::EnvConfiguration;
use axum::{
    extract::Request,
    middleware::{Next, from_fn},
    response::Response,
    serve,
};
use sqlx::SqlitePool;
use std::sync::{Arc, OnceLock};
use tera::Tera;
use tokio::{main, net::TcpListener};
use tower::ServiceBuilder;
use tower_cookies::{CookieManagerLayer, Key};

mod crumble;
mod db;
mod env;
mod routing;

struct AppState {
    pool: SqlitePool,
    tera: Tera,
}

static USERNAME: OnceLock<String> = OnceLock::new();
static PASSWORD_HASH: OnceLock<String> = OnceLock::new();
static COOKIE_KEY: OnceLock<Key> = OnceLock::new();

#[main]
async fn main() {
    let oracle = EnvConfiguration::get();

    USERNAME
        .set(oracle.username)
        .expect("Static username variable should be settable");

    PASSWORD_HASH
        .set(oracle.password_hash)
        .expect("Static password_hash variable should be settable");

    COOKIE_KEY
        .set(Key::from(oracle.cookie_key.as_bytes()))
        .expect("Static cookie_key variable should be settable");

    let pool = db::init_db_connection().await;

    let mut tera = Tera::default();
    tera.load_from_glob("templates/**/*.html")
        .expect("Tera templates should exist in $(pwd)/templates directory");

    let appstate = Arc::new(AppState {
        pool: pool,
        tera: tera,
    });

    let router = routing::get()
        .layer(
            ServiceBuilder::new()
                .layer(from_fn(logger))
                .layer(CookieManagerLayer::new()),
        )
        .with_state(appstate);

    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Port 80 should allow binding a new listener");

    serve(listener, router)
        .await
        .expect("Server crashed unexpectedly");
}

async fn logger(request: Request, next: Next) -> Response {
    println!("Serving {}", request.uri().to_string());
    next.run(request).await
}
