use crate::COOKIE_KEY;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use tower_cookies::{
    Cookie, Cookies,
    cookie::{SameSite, time::Duration},
};

pub struct Crumble {
    jar: Cookies,
}

pub enum CrumbleKind {
    Session,
    Flash,
}

impl CrumbleKind {
    fn name(self) -> &'static str {
        match self {
            Self::Session => "session",
            Self::Flash => "flash",
        }
    }
    fn cookie(self, data: Option<String>) -> Cookie<'static> {
        let value = match data {
            Some(value) => value,
            None => "".to_string(),
        };

        Cookie::build((self.name(), value))
            .domain("fin.siru.ink")
            .path("/")
            .http_only(true)
            .max_age(Duration::days(7))
            .secure(true)
            .same_site(SameSite::Strict)
            .build()
    }
}

impl Crumble {
    pub fn set(&self, kind: CrumbleKind, data: String) {
        let key = COOKIE_KEY
            .get()
            .expect("Global static cookiekey variable must always be accessible");

        let crypto_jar = self.jar.private(key);

        let cookie = kind.cookie(Some(data));

        crypto_jar.add(cookie);
    }

    pub fn get(&self, kind: CrumbleKind) -> String {
        let key = COOKIE_KEY
            .get()
            .expect("Global static cookiekey variable must always be accessible");

        let crypto_jar = self.jar.private(key);

        let cookie = crypto_jar.get(kind.name());

        match cookie {
            Some(cookie) => cookie.value().to_string(),
            None => "".to_string(),
        }
    }

    pub fn del(&self, kind: CrumbleKind) {
        let key = COOKIE_KEY
            .get()
            .expect("Global static cookiekey variable must always be accessible");

        let crypto_jar = self.jar.private(key);

        let cookie = kind.cookie(None);

        // This does not delete the cookie, but rather sets it's contents to be empty.
        crypto_jar.add(cookie);
    }
}

impl<S> FromRequestParts<S> for Crumble
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match Cookies::from_request_parts(parts, state).await {
            Ok(jar) => Ok(Crumble { jar }),
            Err(_) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not extract cookies from client.",
            )),
        }
    }
}
