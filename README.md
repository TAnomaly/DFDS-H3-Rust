# Rust Microservice

Actix Web framework'ü ve PostgreSQL kullanılarak geliştirilmiş modern bir Rust microservice projesi.

## Özellikler

- ✅ RESTful API endpoints
- ✅ PostgreSQL veritabanı entegrasyonu
- ✅ SQLx ile async database işlemleri
- ✅ JSON serialization/deserialization
- ✅ CORS desteği
- ✅ Request logging
- ✅ Health check endpoint
- ✅ CRUD işlemleri
- ✅ UUID kullanımı
- ✅ Modüler yapı
- ✅ Hata yönetimi

## Kurulum

### Ön Koşullar
- Rust (1.70+)
- PostgreSQL (12+)

### Otomatik Kurulum (Kolaysı)

1. PostgreSQL'i yükleyin ve çalıştırın:
```bash
# macOS ile Homebrew
brew install postgresql
brew services start postgresql

# Ubuntu/Debian
sudo apt-get install postgresql
sudo service postgresql start
```

2. Veritabanını oluşturun:
```bash
# Postgres kullanıcısıyla
createdb rustmicro

# veya SQL ile
psql -U postgres -c "CREATE DATABASE rustmicro;"
```

3. .env dosyasını kontrol edin (zaten mevcut):
```bash
cat .env  # DATABASE_URL'yi kontrol edin
```

**Not:** Tablolar, indeksler ve örnek veriler otomatik olarak oluşturulacak!

### Proje Kurulumu

1. Projeyi klonlayın veya indirin
2. Environment değişkenlerini ayarlayın:
```bash
cp .env.example .env
# .env dosyasını düzenleyin
```

3. Projeyi derleyin:
```bash
cargo build
```

## Çalıştırma

```bash
cargo run
```

Server başarıyla başladığında `http://localhost:8080` adresinde çalışacaktır.

## API Endpoints

### Ana Sayfa
- **GET** `/` - API'ye hoş geldin mesajı

### Sağlık Kontrolü
- **GET** `/health` - API'nin durumunu kontrol eder

### Kullanıcı İşlemleri
- **GET** `/api/v1/users` - Tüm kullanıcıları listeler
- **POST** `/api/v1/users` - Yeni kullanıcı oluşturur
- **GET** `/api/v1/users/{id}` - ID'ye göre kullanıcı getirir
- **PUT** `/api/v1/users/{id}` - Kullanıcı bilgilerini günceller
- **DELETE** `/api/v1/users/{id}` - Kullanıcıyı siler

### İstatistikler
- **GET** `/api/v1/stats` - Kullanıcı istatistikleri

### Liman İşlemleri
- **GET** `/api/v1/ports` - Tüm limanları listeler
- **POST** `/api/v1/ports` - Yeni liman oluşturur
- **POST** `/api/v1/ports/nearest` - En yakın limanı bulur (latitude ve longitude gereklidir)
- **GET** `/api/v1/ports/country/{country}` - Ülkeye göre limanları listeler
- **GET** `/api/v1/ports/type/{port_type}` - Liman tipine göre limanları listeler

## Örnek Kullanım

### Tüm Kullanıcıları Listeleme
```bash
curl http://localhost:8080/api/v1/users
```

### Kullanıcı Oluşturma
```bash
curl -X POST http://localhost:8080/api/v1/users \\
  -H "Content-Type: application/json" \\
  -d '{"name": "Ahmet Yılmaz", "email": "ahmet@example.com"}'
```

### Kullanıcı Getirme (UUID ile)
```bash
curl http://localhost:8080/api/v1/users/550e8400-e29b-41d4-a716-446655440000
```

### Kullanıcı Güncelleme
```bash
curl -X PUT http://localhost:8080/api/v1/users/550e8400-e29b-41d4-a716-446655440000 \\
  -H "Content-Type: application/json" \\
  -d '{"name": "Ahmet Yeni Ad", "email": "yeni@example.com"}'
```

### Kullanıcı Silme
```bash
curl -X DELETE http://localhost:8080/api/v1/users/550e8400-e29b-41d4-a716-446655440000
```

### İstatistikler
```bash
curl http://localhost:8080/api/v1/stats
```

### Health Check
```bash
curl http://localhost:8080/health
```

## Geliştirme

Bu proje aşağıdaki teknolojileri kullanır:

- **Actix Web**: High-performance web framework
- **SQLx**: Async SQL toolkit
- **PostgreSQL**: Relational database
- **Tokio**: Async runtime
- **Serde**: Serialization framework
- **UUID**: Unique identifier generation
- **Chrono**: Date and time handling
- **Actix CORS**: CORS middleware
- **env_logger**: Logging

## Sonraki Adımlar

- [x] Veritabanı entegrasyonu (PostgreSQL)
- [x] CRUD işlemleri
- [x] UUID kullanımı
- [x] Hata yönetimi
- [x] Database migrations (sqlx migrate)
- [x] Otomatik veritabanı kurulum
- [x] Environment configuration (.env)
- [ ] Authentication/Authorization (JWT)
- [ ] Input validation (validator crate)
- [ ] Unit & integration tests
- [ ] API documentation (OpenAPI)
- [ ] Docker containerization
- [ ] Health check endpoint improvements
- [ ] Logging improvements (structured logs)
- [ ] Rate limiting
- [ ] Caching (Redis)
