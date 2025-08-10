use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::models::{User, CreateUser, UpdateUser, Port, CreatePort, UpdatePort};
use crate::h3_utils::{coords_to_h3, calculate_distance_km, get_k_ring, calculate_k_for_distance, DEFAULT_RESOLUTION};

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // Tüm kullanıcıları getir
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, latitude, longitude, h3_index, created_at, updated_at FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // ID'ye göre kullanıcı getir
    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, latitude, longitude, h3_index, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // Email'e göre kullanıcı getir
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, name, email, latitude, longitude, h3_index, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    // Yeni kullanıcı oluştur
    pub async fn create_user(&self, user_data: CreateUser) -> Result<User, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // Eğer latitude ve longitude verilmişse H3 index hesapla
        let h3_index = if let (Some(lat), Some(lng)) = (user_data.latitude, user_data.longitude) {
            coords_to_h3(lat, lng, DEFAULT_RESOLUTION).ok()
        } else {
            None
        };

        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, name, email, latitude, longitude, h3_index, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, name, email, latitude, longitude, h3_index, created_at, updated_at"
        )
        .bind(id)
        .bind(&user_data.name)
        .bind(&user_data.email)
        .bind(user_data.latitude)
        .bind(user_data.longitude)
        .bind(h3_index)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    // Kullanıcı güncelle
    pub async fn update_user(&self, id: Uuid, user_data: UpdateUser) -> Result<Option<User>, sqlx::Error> {
        let now = Utc::now();

        // Basit bir update işlemi yapalım
        let user = if let Some(name) = user_data.name {
            if let Some(email) = user_data.email {
                sqlx::query_as::<_, User>(
                    "UPDATE users SET name = $1, email = $2, updated_at = $3 WHERE id = $4 RETURNING id, name, email, created_at, updated_at"
                )
                .bind(name)
                .bind(email)
                .bind(now)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
            } else {
                sqlx::query_as::<_, User>(
                    "UPDATE users SET name = $1, updated_at = $2 WHERE id = $3 RETURNING id, name, email, created_at, updated_at"
                )
                .bind(name)
                .bind(now)
                .bind(id)
                .fetch_optional(&self.pool)
                .await?
            }
        } else if let Some(email) = user_data.email {
            sqlx::query_as::<_, User>(
                "UPDATE users SET email = $1, updated_at = $2 WHERE id = $3 RETURNING id, name, email, created_at, updated_at"
            )
            .bind(email)
            .bind(now)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
        } else {
            return self.get_user_by_id(id).await;
        };

        Ok(user)
    }

    // Kullanıcı sil
    pub async fn delete_user(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }

    // Kullanıcı sayısını getir
    pub async fn count_users(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM users")
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count)
    }
    
    // H3 index'e göre kullanıcıları getir
    pub async fn get_users_by_h3(&self, h3_index: &str) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, latitude, longitude, h3_index, created_at, updated_at FROM users WHERE h3_index = $1"
        )
        .bind(h3_index)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
    
    // Belirli bir alan içindeki kullanıcıları getir (H3 index listesi ile)
    pub async fn get_users_in_area(&self, h3_indices: Vec<String>) -> Result<Vec<User>, sqlx::Error> {
        if h3_indices.is_empty() {
            return Ok(vec![]);
        }
        
        let users = sqlx::query_as::<_, User>(
            "SELECT id, name, email, latitude, longitude, h3_index, created_at, updated_at FROM users WHERE h3_index = ANY($1)"
        )
        .bind(&h3_indices)
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }
    
    // ====== PORT OPERATIONS ====== 
    
    // Tüm limanları getir
    pub async fn get_all_ports(&self) -> Result<Vec<Port>, sqlx::Error> {
        let ports = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(ports)
    }
    
    // ID'ye göre liman getir
    pub async fn get_port_by_id(&self, id: Uuid) -> Result<Option<Port>, sqlx::Error> {
        let port = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(port)
    }
    
    // Koda göre liman getir
    pub async fn get_port_by_code(&self, code: &str) -> Result<Option<Port>, sqlx::Error> {
        let port = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports WHERE code = $1"
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await?;

        Ok(port)
    }
    
    // Yeni liman oluştur
    pub async fn create_port(&self, port_data: CreatePort) -> Result<Port, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        // H3 index hesapla
        let h3_index = coords_to_h3(port_data.latitude, port_data.longitude, DEFAULT_RESOLUTION)
            .map_err(|e| sqlx::Error::Protocol(format!("H3 hesaplama hatası: {}", e)))?;

        let port = sqlx::query_as::<_, Port>(
            "INSERT INTO ports (id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at"
        )
        .bind(id)
        .bind(&port_data.name)
        .bind(&port_data.code)
        .bind(&port_data.country)
        .bind(&port_data.city)
        .bind(port_data.latitude)
        .bind(port_data.longitude)
        .bind(h3_index)
        .bind(&port_data.port_type)
        .bind(port_data.capacity)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await?;

        Ok(port)
    }
    
    // En yakın limanı bul (gerçek km mesafesi ile)
    pub async fn find_nearest_port(&self, lat: f64, lng: f64, max_distance_km: Option<f64>) -> Result<Option<(Port, f64)>, sqlx::Error> {
        // H3 index hesapla
        let target_h3 = coords_to_h3(lat, lng, DEFAULT_RESOLUTION)
            .map_err(|e| sqlx::Error::Protocol(format!("H3 hesaplama hatası: {}", e)))?;
        println!("Target H3: {}", target_h3);

        // Varsayılan max mesafe 50 km, makul k değeri: 20 (yaklaşık 3.5 km yarıçapında arama)
        let max_dist = max_distance_km.unwrap_or(50.0);
        let k = 20; // Sabit makul değer
        println!("Max distance: {} km, k: {}", max_dist, k);

        // kRing ile çevredeki hücreler
        let ring_indices = get_k_ring(&target_h3, k)
            .map_err(|e| sqlx::Error::Protocol(format!("kRing hesaplama hatası: {}", e)))?;
        println!("Ring indices count: {}", ring_indices.len());

        if ring_indices.is_empty() {
            println!("Ring indices boş!");
            return Ok(None);
        }

        // Ring içindeki hücrelerdeki limanları çek
        let ports = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports WHERE h3_index = ANY($1)"
        )
        .bind(&ring_indices)
        .fetch_all(&self.pool)
        .await?;
        println!("Found {} ports in ring", ports.len());

        // Limanları gerçek mesafeye göre filtrele ve en yakın limanı bul
        let mut nearest_port: Option<(Port, f64)> = None;
        let mut shortest_distance = f64::MAX;

        for port in ports {
            let distance = calculate_distance_km(lat, lng, port.latitude, port.longitude);
            println!("Port: {} - Distance: {:.2} km (max: {:.2} km)", port.name, distance, max_dist);
            if distance < shortest_distance && distance <= max_dist {
                shortest_distance = distance;
                nearest_port = Some((port, distance));
                println!("New nearest port: {} at {:.2} km", nearest_port.as_ref().unwrap().0.name, distance);
            }
        }

        Ok(nearest_port)
    }
    
    // Ülkeye göre limanları getir
    pub async fn get_ports_by_country(&self, country: &str) -> Result<Vec<Port>, sqlx::Error> {
        let ports = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports WHERE country ILIKE $1 ORDER BY name"
        )
        .bind(format!("%{}%", country))
        .fetch_all(&self.pool)
        .await?;

        Ok(ports)
    }
    
    // Liman tipine göre limanları getir
    pub async fn get_ports_by_type(&self, port_type: &str) -> Result<Vec<Port>, sqlx::Error> {
        let ports = sqlx::query_as::<_, Port>(
            "SELECT id, name, code, country, city, latitude, longitude, h3_index, port_type, capacity, created_at, updated_at FROM ports WHERE port_type = $1 ORDER BY capacity DESC NULLS LAST"
        )
        .bind(port_type)
        .fetch_all(&self.pool)
        .await?;

        Ok(ports)
    }
    
    // Liman sayısını getir
    pub async fn count_ports(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM ports")
            .fetch_one(&self.pool)
            .await?;

        let count: i64 = row.get("count");
        Ok(count)
    }
}
