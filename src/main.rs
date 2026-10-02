use tokio::main;

mod db;

#[main]
async fn main() {
    let _ = db::init_db_connection().await;
}
