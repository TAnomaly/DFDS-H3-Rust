-- Veritabanı şemasını oluştur
DROP TABLE IF EXISTS users;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Örnek kullanıcılar ekle
INSERT INTO users (name, email) VALUES 
    ('Ahmet Yılmaz', 'ahmet@example.com'),
    ('Mehmet Demir', 'mehmet@example.com'),
    ('Ayşe Kaya', 'ayse@example.com');

-- İndeksler oluştur
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);
