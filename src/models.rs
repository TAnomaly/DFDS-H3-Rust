use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use utoipa::ToSchema;


// Custom DateTime wrapper for Swagger
#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[schema(
    title = "DateTime",
    description = "ISO 8601 DateTime string",
    example = "2024-01-01T00:00:00Z",
    value_type = String,
    format = "date-time"
)]
pub struct DateTimeSchema;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct User {
    /// Kullanıcının benzersiz kimliği (UUID)
    pub id: Uuid,
    /// Kullanıcının adı
    pub name: String,
    /// Kullanıcının e-posta adresi
    pub email: String,
    /// Kullanıcının enlemi (isteğe bağlı)
    pub latitude: Option<f64>,
    /// Kullanıcının boylamı (isteğe bağlı)
    pub longitude: Option<f64>,
    /// Kullanıcının H3 indeksi (isteğe bağlı)
    pub h3_index: Option<String>,
    /// Kullanıcının oluşturulma tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Kullanıcının güncellenme tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUser {
    /// Kullanıcının adı
    pub name: String,
    /// Kullanıcının e-posta adresi
    pub email: String,
    /// Kullanıcının enlemi (isteğe bağlı)
    pub latitude: Option<f64>,
    /// Kullanıcının boylamı (isteğe bağlı)
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUser {
    /// Kullanıcının adı (isteğe bağlı)
    pub name: Option<String>,
    /// Kullanıcının e-posta adresi (isteğe bağlı)
    pub email: Option<String>,
    /// Kullanıcının enlemi (isteğe bağlı)
    pub latitude: Option<f64>,
    /// Kullanıcının boylamı (isteğe bağlı)
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    /// Kullanıcının string formatında kimliği
    pub id: String,
    /// Kullanıcının adı
    pub name: String,
    /// Kullanıcının e-posta adresi
    pub email: String,
    /// Kullanıcının enlemi (isteğe bağlı)
    pub latitude: Option<f64>,
    /// Kullanıcının boylamı (isteğe bağlı)
    pub longitude: Option<f64>,
    /// Kullanıcının H3 indeksi (isteğe bağlı)
    pub h3_index: Option<String>,
    /// Kullanıcının oluşturulma tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Kullanıcının güncellenme tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
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
#[derive(Debug, Serialize, Deserialize, FromRow, Clone, ToSchema)]
pub struct Port {
    /// Limanın benzersiz kimliği (UUID)
    pub id: Uuid,
    /// Limanın adı
    pub name: String,
    /// Limanın IATA/ICAO kodu (örn: TRIST, TRMER)
    pub code: String,
    /// Limanın bulunduğu ülke
    pub country: String,
    /// Limanın bulunduğu şehir
    pub city: String,
    /// Limanın enlemi
    pub latitude: f64,
    /// Limanın boylamı
    pub longitude: f64,
    /// Limanın H3 indeksi
    pub h3_index: String,
    /// Limanın tipi ("container", "cruise", "cargo", "fishing")
    pub port_type: String,
    /// TEU kapasitesi (isteğe bağlı)
    pub capacity: Option<i32>,
    /// Limanın oluşturulma tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Limanın güncellenme tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreatePort {
    /// Limanın adı
    pub name: String,
    /// Limanın IATA/ICAO kodu (örn: TRIST, TRMER)
    pub code: String,
    /// Limanın bulunduğu ülke
    pub country: String,
    /// Limanın bulunduğu şehir
    pub city: String,
    /// Limanın enlemi
    pub latitude: f64,
    /// Limanın boylamı
    pub longitude: f64,
    /// Limanın tipi ("container", "cruise", "cargo", "fishing")
    pub port_type: String,
    /// TEU kapasitesi (isteğe bağlı)
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdatePort {
    /// Limanın adı (isteğe bağlı)
    pub name: Option<String>,
    /// Limanın IATA/ICAO kodu (örn: TRIST, TRMER) (isteğe bağlı)
    pub code: Option<String>,
    /// Limanın bulunduğu ülke (isteğe bağlı)
    pub country: Option<String>,
    /// Limanın bulunduğu şehir (isteğe bağlı)
    pub city: Option<String>,
    /// Limanın enlemi (isteğe bağlı)
    pub latitude: Option<f64>,
    /// Limanın boylamı (isteğe bağlı)
    pub longitude: Option<f64>,
    /// Limanın tipi ("container", "cruise", "cargo", "fishing") (isteğe bağlı)
    pub port_type: Option<String>,
    /// TEU kapasitesi (isteğe bağlı)
    pub capacity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PortResponse {
    /// Limanın string formatında kimliği
    pub id: String,
    /// Limanın adı
    pub name: String,
    /// Limanın IATA/ICAO kodu (örn: TRIST, TRMER)
    pub code: String,
    /// Limanın bulunduğu ülke
    pub country: String,
    /// Limanın bulunduğu şehir
    pub city: String,
    /// Limanın enlemi
    pub latitude: f64,
    /// Limanın boylamı
    pub longitude: f64,
    /// Limanın H3 indeksi
    pub h3_index: String,
    /// Limanın tipi ("container", "cruise", "cargo", "fishing")
    pub port_type: String,
    /// TEU kapasitesi (isteğe bağlı)
    pub capacity: Option<i32>,
    /// Limanın oluşturulma tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub created_at: DateTime<Utc>,
    /// Limanın güncellenme tarihi
    #[schema(value_type = String, format = "date-time", example = "2024-01-01T00:00:00Z")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FindNearestPortRequest {
    /// Arama yapılacak konumun enlemi
    pub latitude: f64,
    /// Arama yapılacak konumun boylamı
    pub longitude: f64,
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

// H3 heatmap için model
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct H3HeatmapCell {
    /// H3 hücre indeksi
    pub h3_index: String,
    /// Hücredeki kullanıcı sayısı
    pub user_count: i64,
    /// Hücrenin merkez enlemi
    pub center_latitude: f64,
    /// Hücrenin merkez boylamı
    pub center_longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct H3HeatmapResponse {
    /// Heatmap hücreleri
    pub cells: Vec<H3HeatmapCell>,
    /// Toplam kullanıcı sayısı
    pub total_users: i64,
    /// Heatmap için kullanılan H3 çözünürlüğü
    pub resolution: u8,
}
