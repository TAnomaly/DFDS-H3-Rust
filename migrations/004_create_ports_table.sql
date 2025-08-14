-- Add migration script here
CREATE TABLE IF NOT EXISTS ports (
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

-- İndeksler oluştur
CREATE INDEX IF NOT EXISTS idx_ports_code ON ports(code);
CREATE INDEX IF NOT EXISTS idx_ports_h3_index ON ports(h3_index); -- H3 indeksi için indeks
CREATE INDEX IF NOT EXISTS idx_ports_location ON ports(latitude, longitude); -- Konum için indeks