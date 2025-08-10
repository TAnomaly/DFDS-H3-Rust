use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub h3_index: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub h3_index: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name,
            email: user.email,
            latitude: user.latitude,
            longitude: user.longitude,
            h3_index: user.h3_index,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

// Port (Liman) modelleri
#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Port {
    pub id: Uuid,
    pub name: String,
    pub code: String, // IATA/ICAO kodu (örn: TRIST, TRMER)
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub h3_index: String,
    pub port_type: String, // "container", "cruise", "cargo", "fishing"
    pub capacity: Option<i32>, // TEU kapasitesi
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePort {
    pub name: String,
    pub code: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub port_type: String,
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePort {
    pub name: Option<String>,
    pub code: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub port_type: Option<String>,
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PortResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub country: String,
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    pub h3_index: String,
    pub port_type: String,
    pub capacity: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FindNearestPortRequest {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearestPortResponse {
    pub port: PortResponse,
    pub distance_km: f64, // Gerçek mesafe km cinsinden
}

impl From<Port> for PortResponse {
    fn from(port: Port) -> Self {
        Self {
            id: port.id.to_string(),
            name: port.name,
            code: port.code,
            country: port.country,
            city: port.city,
            latitude: port.latitude,
            longitude: port.longitude,
            h3_index: port.h3_index,
            port_type: port.port_type,
            capacity: port.capacity,
            created_at: port.created_at,
            updated_at: port.updated_at,
        }
    }
}
