use crate::{
    AppState, PASSWORD_HASH, USERNAME,
    crumble::{Flash, Session},
    template::LoginPage,
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

pub async fn post_login(flash: Flash, session: Session, Form(form): Form<LoginForm>) -> Response {
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

    session.set_new().await;

    Redirect::to("/").into_response()
}
