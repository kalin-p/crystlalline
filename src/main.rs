use dotenvy::dotenv;
use std::env;

mod orgroam;
use orgroam::sqlite_con::*;

use sqlx::sqlite::SqlitePool;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let dbpath = env::var("DATABASE_URL").expect("DATABASE_URL not set in environment");
    let pool = SqlitePool::connect(dbpath.as_str()).await.unwrap();
    let nodes = get_all_nodes(&pool).await;
    let links = get_all_links(&pool).await;
}
