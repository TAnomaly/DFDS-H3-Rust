use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::database::Database;
use crate::models::{CreateUser, UpdateUser, UserResponse, CreatePort, PortResponse, FindNearestPortRequest, NearestPortResponse};

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    pub message: String,
    pub status: String,
}

#[derive(Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_users: i64,
    pub status: String,
}

// Ana sayfa endpoint'i
pub async fn index() -> Result<HttpResponse> {
    let response = ApiResponse {
        message: "Rust Microservice API'ye hoş geldiniz!".to_string(),
        status: "success".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

// Sağlık kontrolü endpoint'i
pub async fn health_check() -> Result<HttpResponse> {
    let response = ApiResponse {
        message: "API çalışıyor".to_string(),
        status: "healthy".to_string(),
    };
    Ok(HttpResponse::Ok().json(response))
}

// İstatistikler endpoint'i
pub async fn get_stats(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.count_users().await {
        Ok(count) => {
            let response = StatsResponse {
                total_users: count,
                status: "success".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "İstatistikler alınırken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Tüm kullanıcıları getirme endpoint'i (GET)
pub async fn get_all_users(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_users().await {
        Ok(users) => {
            let user_responses: Vec<UserResponse> = users.into_iter().map(|u| u.into()).collect();
            Ok(HttpResponse::Ok().json(user_responses))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Kullanıcılar getirilirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Kullanıcı oluşturma endpoint'i (POST)
pub async fn create_user(
    user_data: web::Json<CreateUser>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    // Email kontrolü
    match db.get_user_by_email(&user_data.email).await {
        Ok(Some(_)) => {
            let response = ApiResponse {
                message: "Bu email adresi zaten kullanımda".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::BadRequest().json(response));
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Veritabanı hatası".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    }

    match db.create_user(user_data.into_inner()).await {
        Ok(user) => {
            println!("Yeni kullanıcı oluşturuldu: {} - {}", user.name, user.email);
            let user_response: UserResponse = user.into();
            Ok(HttpResponse::Created().json(user_response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Kullanıcı oluşturulurken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// ID'ye göre kullanıcı getirme endpoint'i (GET)
pub async fn get_user(
    path: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    
    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => {
            let response = ApiResponse {
                message: "Geçersiz kullanıcı ID formatı".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    match db.get_user_by_id(user_id).await {
        Ok(Some(user)) => {
            let user_response: UserResponse = user.into();
            Ok(HttpResponse::Ok().json(user_response))
        }
        Ok(None) => {
            let response = ApiResponse {
                message: "Kullanıcı bulunamadı".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Kullanıcı getirilirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Kullanıcı güncelleme endpoint'i (PUT)
pub async fn update_user(
    path: web::Path<String>,
    user_data: web::Json<UpdateUser>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    
    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => {
            let response = ApiResponse {
                message: "Geçersiz kullanıcı ID formatı".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    // Email değişikliği varsa, başka kullanıcı tarafından kullanılmıyor mu kontrol et
    if let Some(ref email) = user_data.email {
        match db.get_user_by_email(email).await {
            Ok(Some(existing_user)) if existing_user.id != user_id => {
                let response = ApiResponse {
                    message: "Bu email adresi başka bir kullanıcı tarafından kullanılıyor".to_string(),
                    status: "error".to_string(),
                };
                return Ok(HttpResponse::BadRequest().json(response));
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("Database error: {}", e);
                let response = ApiResponse {
                    message: "Veritabanı hatası".to_string(),
                    status: "error".to_string(),
                };
                return Ok(HttpResponse::InternalServerError().json(response));
            }
        }
    }

    match db.update_user(user_id, user_data.into_inner()).await {
        Ok(Some(user)) => {
            println!("Kullanıcı güncellendi: {} - {}", user.name, user.email);
            let user_response: UserResponse = user.into();
            Ok(HttpResponse::Ok().json(user_response))
        }
        Ok(None) => {
            let response = ApiResponse {
                message: "Kullanıcı bulunamadı".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Kullanıcı güncellenirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Kullanıcı silme endpoint'i (DELETE)
pub async fn delete_user(
    path: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let user_id_str = path.into_inner();
    
    let user_id = match Uuid::parse_str(&user_id_str) {
        Ok(id) => id,
        Err(_) => {
            let response = ApiResponse {
                message: "Geçersiz kullanıcı ID formatı".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::BadRequest().json(response));
        }
    };

    match db.delete_user(user_id).await {
        Ok(true) => {
            println!("Kullanıcı silindi: {}", user_id);
            let response = ApiResponse {
                message: "Kullanıcı başarıyla silindi".to_string(),
                status: "success".to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(false) => {
            let response = ApiResponse {
                message: "Kullanıcı bulunamadı".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Kullanıcı silinirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// ====== PORT HANDLERS ======

// Tüm limanları getirme endpoint'i (GET)
pub async fn get_all_ports(db: web::Data<Database>) -> Result<HttpResponse> {
    match db.get_all_ports().await {
        Ok(ports) => {
            let port_responses: Vec<PortResponse> = ports.into_iter().map(|p| p.into()).collect();
            Ok(HttpResponse::Ok().json(port_responses))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Limanlar getirilirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Liman oluşturma endpoint'i (POST)
pub async fn create_port(
    port_data: web::Json<CreatePort>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    // Kod kontrolü
    match db.get_port_by_code(&port_data.code).await {
        Ok(Some(_)) => {
            let response = ApiResponse {
                message: "Bu liman kodu zaten kullanımda".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::BadRequest().json(response));
        }
        Ok(None) => {}
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Veritabanı hatası".to_string(),
                status: "error".to_string(),
            };
            return Ok(HttpResponse::InternalServerError().json(response));
        }
    }

    match db.create_port(port_data.into_inner()).await {
        Ok(port) => {
            println!("Yeni liman oluşturuldu: {} - {}", port.name, port.code);
            let port_response: PortResponse = port.into();
            Ok(HttpResponse::Created().json(port_response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Liman oluşturulurken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// En yakın liman bulma endpoint'i (POST)
pub async fn find_nearest_port(
    request: web::Json<FindNearestPortRequest>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    match db.find_nearest_port(request.latitude, request.longitude, None).await {
        Ok(Some((port, distance_km))) => {
            let response = NearestPortResponse {
                port: port.into(),
                distance_km,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => {
            let response = ApiResponse {
                message: "Belirtilen mesafede liman bulunamadı".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::NotFound().json(response))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "En yakın liman aranırken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Ülkeye göre limanları getirme endpoint'i (GET)
pub async fn get_ports_by_country(
    path: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let country = path.into_inner();
    
    match db.get_ports_by_country(&country).await {
        Ok(ports) => {
            let port_responses: Vec<PortResponse> = ports.into_iter().map(|p| p.into()).collect();
            Ok(HttpResponse::Ok().json(port_responses))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Limanlar getirilirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}

// Liman tipine göre limanları getirme endpoint'i (GET)
pub async fn get_ports_by_type(
    path: web::Path<String>,
    db: web::Data<Database>,
) -> Result<HttpResponse> {
    let port_type = path.into_inner();
    
    match db.get_ports_by_type(&port_type).await {
        Ok(ports) => {
            let port_responses: Vec<PortResponse> = ports.into_iter().map(|p| p.into()).collect();
            Ok(HttpResponse::Ok().json(port_responses))
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            let response = ApiResponse {
                message: "Limanlar getirilirken hata oluştu".to_string(),
                status: "error".to_string(),
            };
            Ok(HttpResponse::InternalServerError().json(response))
        }
    }
}
