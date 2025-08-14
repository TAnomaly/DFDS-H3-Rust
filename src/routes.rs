use actix_web::{web, HttpResponse};
use crate::handlers::{
    index, health_check, create_user, get_user, get_all_users, update_user, delete_user, get_stats,
    get_all_ports, create_port, find_nearest_port, get_ports_by_country, get_ports_by_type, get_user_heatmap
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Ana routes
        .route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        .route("/swagger-ui", web::get().to(|| async { 
            actix_web::HttpResponse::Found()
                .append_header(("Location", "/swagger-ui/"))
                .finish()
        }))
        .route("/heatmap", web::get().to(|| async {
            let html = std::fs::read_to_string("static/hexagon-heatmap.html")
                .unwrap_or_else(|_| "<h1>H3 Hexagon Heatmap sayfası bulunamadı</h1>".to_string());
            HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(html)
        }))
        
        // API v1 routes
        .service(
            web::scope("/api/v1")
                .route("/stats", web::get().to(get_stats))
                .route("/heatmap", web::get().to(get_user_heatmap))
                .service(
                    web::scope("/users")
                        .route("", web::get().to(get_all_users))
                        .route("", web::post().to(create_user))
                        .route("/{id}", web::get().to(get_user))
                        .route("/{id}", web::put().to(update_user))
                        .route("/{id}", web::delete().to(delete_user))
                )
                .service(
                    web::scope("/ports")
                        .route("", web::get().to(get_all_ports))
                        .route("", web::post().to(create_port))
                        .route("/nearest", web::post().to(find_nearest_port))
                        .route("/country/{country}", web::get().to(get_ports_by_country))
                        .route("/type/{port_type}", web::get().to(get_ports_by_type))
                )
        );
}
