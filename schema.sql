-- Veritabanı şemasını oluştur
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS ports;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    email VARCHAR UNIQUE NOT NULL,
    latitude DOUBLE PRECISION,
    longitude DOUBLE PRECISION,
    h3_index VARCHAR,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE ports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR NOT NULL,
    code VARCHAR UNIQUE NOT NULL, -- IATA/ICAO kodu (örn: TRIST, TRMER)
    country VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    h3_index VARCHAR NOT NULL, -- H3 indeksi
    port_type VARCHAR NOT NULL CHECK (port_type IN ('container', 'cruise', 'cargo', 'fishing')), -- Liman tipi
    capacity INTEGER, -- TEU kapasitesi
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Örnek kullanıcılar ekle
INSERT INTO users (name, email) VALUES 
    ('Ahmet Yılmaz', 'ahmet@example.com'),
    ('Mehmet Demir', 'mehmet@example.com'),
    ('Ayşe Kaya', 'ayse@example.com');

-- Örnek limanlar ekle
INSERT INTO ports (name, code, country, city, latitude, longitude, h3_index, port_type, capacity) VALUES
    ('İstanbul Limanı', 'TRIST', 'Turkey', 'İstanbul', 41.015137, 28.979530, '8a1d0c1ffffffff', 'container', 5000000),
    ('Mersin Limanı', 'TRMER', 'Turkey', 'Mersin', 36.786670, 34.786670, '8a1d0b1ffffffff', 'container', 2000000),
    ('İzmir Limanı', 'TRIZM', 'Turkey', 'İzmir', 38.423734, 27.142826, '8a1d0a1ffffffff', 'cruise', 1000000);

-- İndeksler oluştur
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_ports_code ON ports(code);
CREATE INDEX idx_ports_h3_index ON ports(h3_index); -- H3 indeksi için indeks
CREATE INDEX idx_ports_location ON ports(latitude, longitude); -- Konum için indeks