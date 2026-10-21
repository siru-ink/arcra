use crate::env::EnvConfiguration;
use axum::{
    extract::Request,
    middleware::{Next, from_fn},
    response::{IntoResponse, Response},
    serve,
};
use sqlx::SqlitePool;
use std::sync::{Arc, OnceLock};
use tera::Tera;
use tokio::{main, net::TcpListener};
use tower_cookies::{CookieManagerLayer, Key};

mod db;
mod domain;
mod env;
mod ext;
mod routing;
mod services;
mod template;

struct AppState {
    pool: SqlitePool,
    tera: Arc<Tera>,
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
        .set(Key::from(&oracle.cookie_key[..]))
        .expect("Static cookie_key variable should be settable");

    #[cfg(debug_assertions)]
    println!(
        "{} | {} | {:?}",
        USERNAME.get().unwrap(),
        PASSWORD_HASH.get().unwrap(),
        COOKIE_KEY.get().unwrap().master()
    );

    let pool = db::init_db_connection().await;

    let mut tera = Tera::default();
    tera.register_filter("money", money_format_template);
    tera.load_from_glob("templates/**/*.html")
        .expect("Tera templates should exist in $(pwd)/templates directory");

    let appstate = Arc::new(AppState {
        pool: pool,
        tera: Arc::new(tera),
    });

    let router = routing::get().fallback(error_handler_404);

    #[cfg(debug_assertions)]
    let router = router.layer(from_fn(logger));

    let router = router.layer(CookieManagerLayer::new()).with_state(appstate);

    #[cfg(not(debug_assertions))]
    let listener = TcpListener::bind("0.0.0.0:80")
        .await
        .expect("Port 80 should allow binding a new listener");

    #[cfg(debug_assertions)]
    let listener = TcpListener::bind("127.0.0.1:8160")
        .await
        .expect("Port 8160 should allow binding a new listener");

    serve(listener, router)
        .with_graceful_shutdown(check_interupt())
        .await
        .expect("Server crashed unexpectedly");
}

#[cfg(debug_assertions)]
async fn logger(request: Request, next: Next) -> Response {
    println!("Serving {}", request.uri());
    next.run(request).await
}

async fn check_interupt() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to launch ctrl-c handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

async fn error_handler_404(flash: crate::ext::Flash, uri: axum::http::Uri) -> Response {
    let path = uri.path();
    let ignored_paths = ["/favicon.ico", "/robots.txt", "/sitemap.xml"];
    if !ignored_paths.contains(&path) {
        flash.set("The requested webpage could not be found (Error 404 Not Found). You have been redirected to the homepage instead.".to_string());
    }
    axum::response::Redirect::to("/").into_response()
}

fn money_format_template(value: i64, _: tera::Kwargs, _: &tera::State) -> String {
    let sign = if value < 0 { "-" } else { "" };
    let value = value.abs();
    let euros = value / 100;
    let cents = value % 100;
    format!("{}{}.{:02}", sign, euros, cents)
}
