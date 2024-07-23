use crate::handlers::health::health_check_handler;

/// 服务健康状态检查
pub fn health_check(cfg:&mut actix_web::web::ServiceConfig) {
    cfg.route("/health", actix_web::web::get().to(health_check_handler));
}


