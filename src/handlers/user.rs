use actix_web::{get, Responder};
use actix_web::web::{Data, service};
use sqlx::Postgres;
use crate::config::app::AppState;
use crate::error::ServerError;
use crate::models::user::User;

/**
从数据库中获取所有的用户信息
 **/
#[get("/users")]
pub async fn get_users(app_state: Data<AppState>) -> Result<impl Responder,ServerError> {
    let vec = sqlx::query_as::<Postgres, User>("select * from gg_user")
        .fetch_all(&app_state.db)
        .await?;
    return Ok(serde_json::to_string(&vec));
}
