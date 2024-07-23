use super::db;
use actix_web::web::Data;
use dotenv::dotenv;
use sqlx::PgPool;
use std::{env, sync::Mutex};

pub struct AppState {
    pub app_name: String,
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    pub db: PgPool,
}

/// <h3>初始化程序配置</h3>
pub async fn app_init() -> Data<AppState> {
    dotenv().ok();
    // 配置日志
    env_logger::init();
    let app_name = env::var("APP_NAME").expect("NO SET APP_NAME");
    let db = db::db_init().await;
    Data::new(AppState {
        app_name,
        health_check_response: "I'm healthy".to_string(),
        visit_count: Mutex::new(0),
        db,
    })
}
