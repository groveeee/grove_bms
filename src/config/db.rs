use std::env;

use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Connection, Pool, Postgres};

/**
初始化数据库连接
**/
pub async fn db_init() -> Pool<Postgres> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("NO SET DATABASE_URL");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}
