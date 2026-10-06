use chrono::{DateTime, Duration, Utc};
use sqlx::{SqlitePool, query_as};

pub struct Session {
    id: i64,
    valid_until: DateTime<Utc>,
}

impl Session {
    pub async fn new(pool: &SqlitePool) -> Self {
        let next_week = Utc::now() + Duration::weeks(1);

        match query_as!(
            Session,
            "INSERT INTO sessions (valid_until) VALUES (?) RETURNING id,valid_until as \"valid_until: _\"",
            next_week
        )
        .fetch_one(pool)
        .await {
            Ok(session) => session,
            Err(e) => panic!("Failed to create new session in database: {}", e),
        }
    }

    pub async fn exists(pool: &SqlitePool, id: i64) -> bool {
        match query_as!(
            Session,
            "SELECT id,valid_until as \"valid_until: _\" FROM sessions WHERE id = ?",
            id
        )
        .fetch_optional(pool)
        .await
        {
            Ok(Some(session)) => session.valid_until >= Utc::now(),
            _ => false,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }
}
