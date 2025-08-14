mod handlers;
mod routes;
mod config;
mod models;
mod database;
mod setup;
mod h3_utils;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use sqlx::postgres::PgPoolOptions;
use routes::configure_routes;
use config::AppConfig;
use database::Database;
// Swagger UI imports
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

// Import handler functions for Swagger documentation
use crate::handlers::{
    index, health_check, create_user, get_user, get_all_users, update_user, delete_user, get_stats,
    get_all_ports, create_port, find_nearest_port, get_ports_by_country, get_ports_by_type
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .env dosyasını yükle
    dotenvy::dotenv().ok();
    
    let config = AppConfig::from_env();
    
    // Logger'ı ayarla
    unsafe {
        std::env::set_var("RUST_LOG", &config.log_level);
    }
    env_logger::init();

    // Otomatik sistem kurulumu
    println!("🚀 Otomatik sistem kurulumu başlatılıyor...");
    
    // PostgreSQL'i otomatik başlat
    if let Err(e) = setup::ensure_postgresql_running().await {
        eprintln!("⚠️  PostgreSQL otomatik başlatılamadı: {}", e);
        eprintln!("📝 Manuel olarak başlatmayı deneyin...");
    }
    
    // Veritabanını otomatik oluştur
    if let Err(e) = setup::ensure_database_exists(&config.database_url).await {
        eprintln!("⚠️  Veritabanı otomatik oluşturulamadı: {}", e);
        eprintln!("📝 Manuel olarak oluşturmayı deneyin...");
    }

    // Veritabanı pool'u oluştur
    println!("🔌 PostgreSQL'e bağlanılıyor: {}", &config.database_url.replace(|c: char| c.is_ascii_digit(), "*"));
    
    let pool = match PgPoolOptions::new()
        .max_connections(config.database_max_connections)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("❌ Veritabanına bağlanılamadı: {}", e);
            eprintln!("💡 İpuçları:");
            eprintln!("   1. PostgreSQL server'ınız çalıştığından emin olun:");
            eprintln!("      brew services start postgresql    # macOS ile");
            eprintln!("      sudo service postgresql start     # Linux ile");
            eprintln!("");
            eprintln!("   2. Veritabanını oluşturun:");
            eprintln!("      createdb rustmicro               # veya");
            eprintln!("      psql -U postgres -c \"CREATE DATABASE rustmicro;\"");
            eprintln!("");
            eprintln!("   3. DATABASE_URL'nizi kontrol edin (.env dosyası):");
            eprintln!("      DATABASE_URL={}", config.database_url);
            eprintln!("");
            std::process::exit(1);
        }
    };

    println!("✅ Veritabanı bağlantısı başarılı!");

    // Veritabanı migrasyonlarını çalıştır
    println!("🔄 Veritabanı migrasyonları kontrol ediliyor...");
    match sqlx::migrate!("./migrations").run(&pool).await {
        Ok(_) => println!("✅ Veritabanı migrasyonları başarıyla tamamlandı!"),
        Err(e) => {
            eprintln!("❌ Migrasyon hatası: {}", e);
            eprintln!("💡 İpucuları:");
            eprintln!("   - PostgreSQL server'ınız çalıştığından emin olun");
            eprintln!("   - DATABASE_URL'nin doğru olduğunu kontrol edin");
            eprintln!("   - Veritabanının mevcut olduğunu kontrol edin");
            std::process::exit(1);
        }
    };

    let db = web::Data::new(Database::new(pool));

    // Başarı banner'ını göster
    setup::print_success_banner();

    // Define OpenAPI documentation
    #[derive(OpenApi)]
    #[openapi(
        paths(
            handlers::index,
            handlers::health_check,
            handlers::get_stats,
            handlers::get_all_users,
            handlers::create_user,
            handlers::get_user,
            handlers::update_user,
            handlers::delete_user,
            handlers::get_all_ports,
            handlers::create_port,
            handlers::find_nearest_port,
            handlers::get_ports_by_country,
            handlers::get_ports_by_type,
            handlers::get_user_heatmap,
        ),
        components(
            schemas(
                models::User,
                models::CreateUser,
                models::UpdateUser,
                models::UserResponse,
                models::Port,
                models::CreatePort,
                models::UpdatePort,
                models::PortResponse,
                models::FindNearestPortRequest,
                models::H3HeatmapResponse,
                models::H3HeatmapCell,
                models::DateTimeSchema,
                handlers::ApiResponse,
                handlers::StatsResponse,
            )
        ),
        tags(
            (name = "health", description = "Health check endpoints"),
            (name = "users", description = "User management endpoints"),
            (name = "ports", description = "Port management endpoints"),
            (name = "stats", description = "Statistics endpoints"),
        )
    )]
    struct ApiDoc;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .app_data(db.clone())
            .wrap(cors)
            .wrap(Logger::default())
            .configure(configure_routes)
            // Swagger UI integration
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
    })
    .bind(config.server_address())?
    .run()
    .await
}
