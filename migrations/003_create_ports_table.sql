-- Limanlar tablosunu oluştur
CREATE TABLE IF NOT EXISTS ports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL,
    code VARCHAR(10) UNIQUE NOT NULL, -- Port kodu (IATA/ICAO)
    country VARCHAR NOT NULL,
    city VARCHAR NOT NULL,
    latitude DOUBLE PRECISION NOT NULL,
    longitude DOUBLE PRECISION NOT NULL,
    h3_index VARCHAR NOT NULL,
    port_type VARCHAR NOT NULL, -- container, cruise, cargo, fishing
    capacity INTEGER, -- TEU kapasitesi
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- İndeksler oluştur
CREATE INDEX IF NOT EXISTS idx_ports_h3_index ON ports(h3_index);
CREATE INDEX IF NOT EXISTS idx_ports_country ON ports(country);
CREATE INDEX IF NOT EXISTS idx_ports_port_type ON ports(port_type);
CREATE INDEX IF NOT EXISTS idx_ports_code ON ports(code);

-- Türkiye'deki önemli limanları ekle
INSERT INTO ports (name, code, country, city, latitude, longitude, h3_index, port_type, capacity)
SELECT * FROM (VALUES
    ('Ambarlı Limanı', 'TRAMB', 'Turkey', 'Istanbul', 40.9770, 28.6850, '891ec90b52fffff', 'container', 2500000),
    ('Haydarpaşa Limanı', 'TRHAY', 'Turkey', 'Istanbul', 40.9990, 29.0180, '891ec91434fffff', 'cargo', 500000),
    ('İzmir Alsancak Limanı', 'TRIZM', 'Turkey', 'Izmir', 38.4480, 27.1350, '891f1d6baabffff', 'container', 1200000),
    ('Mersin Limanı', 'TRMER', 'Turkey', 'Mersin', 36.8000, 34.6333, '891f574062fffff', 'container', 1800000),
    ('Samsun Limanı', 'TRSAM', 'Turkey', 'Samsun', 41.2928, 36.3313, '891ea42692fffff', 'cargo', 400000),
    ('Trabzon Limanı', 'TRTRB', 'Turkey', 'Trabzon', 40.9980, 39.7769, '891ea551b8fffff', 'cargo', 300000),
    ('Iskenderun Limanı', 'TRISK', 'Turkey', 'Iskenderun', 36.5833, 36.1667, '891f56c5c6fffff', 'container', 1000000),
    ('Bandırma Limanı', 'TRBAN', 'Turkey', 'Bandirma', 40.3500, 27.9833, '891ec5b152fffff', 'cargo', 200000),
    ('Çanakkale Limanı', 'TRCAN', 'Turkey', 'Canakkale', 40.1553, 26.4142, '891ec4b28afffff', 'cargo', 150000),
    ('Antalya Limanı', 'TRANT', 'Turkey', 'Antalya', 36.8841, 30.7056, '891f50d492fffff', 'cruise', 100000)
) AS new_ports(name, code, country, city, latitude, longitude, h3_index, port_type, capacity)
WHERE NOT EXISTS (SELECT 1 FROM ports WHERE ports.code = new_ports.code);

-- Dünya'dan önemli limanlar
INSERT INTO ports (name, code, country, city, latitude, longitude, h3_index, port_type, capacity)
SELECT * FROM (VALUES
    ('Port of Shanghai', 'CNSHA', 'China', 'Shanghai', 31.2304, 121.4737, '8922816a553ffff', 'container', 47000000),
    ('Port of Singapore', 'SGSIN', 'Singapore', 'Singapore', 1.2966, 103.8518, '8922a4c8443ffff', 'container', 37200000),
    ('Port of Rotterdam', 'NLRTM', 'Netherlands', 'Rotterdam', 51.9244, 4.4777, '891fb1b1517ffff', 'container', 14800000),
    ('Port of Hamburg', 'DEHAM', 'Germany', 'Hamburg', 53.5511, 9.9937, '891fb4b0527ffff', 'container', 8800000),
    ('Port of Los Angeles', 'USLAX', 'USA', 'Los Angeles', 33.7362, -118.2647, '892411b5b07ffff', 'container', 9300000),
    ('Port of Piraeus', 'GRPIR', 'Greece', 'Piraeus', 37.9385, 23.6442, '891f59c940bffff', 'container', 5400000)
) AS world_ports(name, code, country, city, latitude, longitude, h3_index, port_type, capacity)
WHERE NOT EXISTS (SELECT 1 FROM ports WHERE ports.code = world_ports.code);
