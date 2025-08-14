use std::process::Command;

pub async fn ensure_postgresql_running() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 PostgreSQL durumu kontrol ediliyor...");
    
    // PostgreSQL'in çalışıp çalışmadığını kontrol et
    let check_result = Command::new("pg_isready")
        .arg("-h")
        .arg("localhost")
        .arg("-p")
        .arg("5432")
        .output();

    match check_result {
        Ok(output) if output.status.success() => {
            println!("✅ PostgreSQL zaten çalışıyor!");
            return Ok(());
        }
        _ => {
            println!("⏳ PostgreSQL çalışmıyor, başlatılıyor...");
        }
    }

    // PostgreSQL'i başlatmaya çalış
    let start_result = Command::new("brew")
        .args(&["services", "start", "postgresql"])
        .output();

    match start_result {
        Ok(output) if output.status.success() => {
            println!("✅ PostgreSQL başlatıldı!");
            
            // Biraz bekle ki PostgreSQL hazır olsun
            println!("⏳ PostgreSQL'in hazır olması bekleniyor...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            Ok(())
        }
        Ok(output) => {
            eprintln!("⚠️  PostgreSQL başlatılamadı: {}", 
                String::from_utf8_lossy(&output.stderr));
            Err("PostgreSQL başlatılamadı".into())
        }
        Err(e) => {
            eprintln!("❌ brew komutu çalıştırılamadı: {}", e);
            eprintln!("💡 PostgreSQL'i manuel olarak başlatın:");
            eprintln!("   brew services start postgresql");
            Err(e.into())
        }
    }
}

pub async fn ensure_database_exists(database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Veritabanı varlığı kontrol ediliyor...");
    
    // Database URL'den database ismini çıkar
    let db_name = database_url
        .split('/')
        .last()
        .unwrap_or("rustmicro");
    
    // Veritabanının var olup olmadığını kontrol et
    let check_result = Command::new("psql")
        .args(&["-U", "postgres", "-lqt"])
        .output();

    match check_result {
        Ok(output) if output.status.success() => {
            let db_list = String::from_utf8_lossy(&output.stdout);
            if db_list.contains(db_name) {
                println!("✅ Veritabanı '{}' zaten mevcut!", db_name);
                return Ok(());
            }
        }
        _ => {}
    }

    // Veritabanını oluştur
    println!("🔨 Veritabanı '{}' oluşturuluyor...", db_name);
    
    let create_result = Command::new("createdb")
        .arg(db_name)
        .output();

    match create_result {
        Ok(output) if output.status.success() => {
            println!("✅ Veritabanı '{}' başarıyla oluşturuldu!", db_name);
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("already exists") {
                println!("✅ Veritabanı '{}' zaten mevcut!", db_name);
                Ok(())
            } else {
                eprintln!("⚠️  Veritabanı oluşturulamadı: {}", stderr);
                Err("Veritabanı oluşturulamadı".into())
            }
        }
        Err(e) => {
            eprintln!("❌ createdb komutu çalıştırılamadı: {}", e);
            eprintln!("💡 Veritabanını manuel olarak oluşturun:");
            eprintln!("   createdb {}", db_name);
            Err(e.into())
        }
    }
}

pub fn print_success_banner() {
    println!();
    println!("🎉 ================================");
    println!("🎉  RUST MICROSERVICE HAZIR!");
    println!("🎉 ================================");
    println!();
    println!("📡 API Server: http://127.0.0.1:8080");
    println!("💚 Health Check: http://127.0.0.1:8080/health");
    println!();
    println!("🧪 Test komutları:");
    println!("   curl http://127.0.0.1:8080/health");
    println!("   curl http://127.0.0.1:8080/api/v1/users");
    println!("   curl http://127.0.0.1:8080/api/v1/stats");
    println!();
    println!("📚 Daha fazla endpoint için README.md'yi inceleyin!");
    println!();
}
