use crate::{
    AppState, PASSWORD_HASH, USERNAME,
    ext::{Flash, Session},
    services,
    template::{LoginPage, LogoutPage},
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect, Response},
};
use serde::Deserialize;
use std::sync::Arc;

pub async fn get_login(State(state): State<Arc<AppState>>, flash: Flash) -> Response {
    LoginPage::show(&state.tera, flash.message())
}

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn post_login(
    flash: Flash,
    butler: crate::services::Butler,
    Form(form): Form<LoginForm>,
) -> Response {
    let global_username = USERNAME
        .get()
        .expect("Global username variable must be accessible during runtime");
    let global_password_hash = PASSWORD_HASH
        .get()
        .expect("Global password_hash variable must be accessible during runtime");

    if form.username != *global_username {
        flash.set("Username was not correct.".to_string());
        return Redirect::to("/login").into_response();
    }

    let parsed_password_hash = PasswordHash::new(global_password_hash)
        .expect("Global password_hash variable must be a valid argon2id hash");

    let login_correct = Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_password_hash)
        .is_ok();

    if !login_correct {
        flash.set("Password was not correct.".to_string());
        return Redirect::to("/login").into_response();
    }

    if let Err(e) = butler.save_new_session().await {
        #[cfg(debug_assertions)]
        eprintln!("{:?}", e);
        flash.set("Could not save session to browser.".to_string());
    };

    Redirect::to("/").into_response()
}

pub async fn get_logout(
    State(state): State<Arc<AppState>>,
    _session: Session,
    butler: services::Butler,
) -> Response {
    if let Err(e) = butler.remove_session_from_client() {
        #[cfg(debug_assertions)]
        eprintln!("{:?}", e);
    }
    LogoutPage::show(&state.tera)
}
