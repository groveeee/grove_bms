use actix_web::{App, HttpServer};
use crate::routers::health_check;

mod config;
mod models;
mod handlers;
mod error;
mod routers;
mod utils;
mod test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = config::app::app_init().await;
    HttpServer::new(move || App::new()
        .app_data(app_state.clone())
        .service(handlers::user::get_users)
        .configure(health_check)
    )
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
