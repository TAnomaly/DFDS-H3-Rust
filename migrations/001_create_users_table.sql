-- Kullanıcılar tablosunu oluştur
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- İndeksler oluştur
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
CREATE INDEX IF NOT EXISTS idx_users_created_at ON users(created_at);

-- Örnek kullanıcılar ekle (sadece tablo boşsa)
INSERT INTO users (name, email)
SELECT 'Ahmet Yılmaz', 'ahmet@example.com'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE email = 'ahmet@example.com');

INSERT INTO users (name, email)
SELECT 'Mehmet Demir', 'mehmet@example.com'  
WHERE NOT EXISTS (SELECT 1 FROM users WHERE email = 'mehmet@example.com');

INSERT INTO users (name, email)
SELECT 'Ayşe Kaya', 'ayse@example.com'
WHERE NOT EXISTS (SELECT 1 FROM users WHERE email = 'ayse@example.com');
