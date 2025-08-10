use actix_web::web;
use crate::handlers::{
    index, health_check, create_user, get_user, get_all_users, update_user, delete_user, get_stats,
    get_all_ports, create_port, find_nearest_port, get_ports_by_country, get_ports_by_type
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        // Ana routes
        .route("/", web::get().to(index))
        .route("/health", web::get().to(health_check))
        
        // API v1 routes
        .service(
            web::scope("/api/v1")
                .route("/stats", web::get().to(get_stats))
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
