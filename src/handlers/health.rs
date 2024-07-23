use actix_web::HttpResponse;
use actix_web::web::Data;
use crate::config::app::AppState;

pub async fn health_check_handler(app_state: Data<AppState>) -> HttpResponse {
    *app_state.visit_count.lock().unwrap()+=1;
    let result = format!("server states:{} visit_count:{}", app_state.health_check_response, app_state.visit_count.lock().unwrap());
    HttpResponse::Ok().json(result)
}