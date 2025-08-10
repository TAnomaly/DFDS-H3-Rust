-- Kullanıcılar tablosuna lokasyon alanları ekle
ALTER TABLE users 
ADD COLUMN IF NOT EXISTS latitude DOUBLE PRECISION,
ADD COLUMN IF NOT EXISTS longitude DOUBLE PRECISION,
ADD COLUMN IF NOT EXISTS h3_index VARCHAR;

-- H3 index için indeks oluştur
CREATE INDEX IF NOT EXISTS idx_users_h3_index ON users(h3_index);

-- Mevcut kullanıcılara örnek lokasyonlar ekle (isteğe bağlı)
UPDATE users SET 
    latitude = 41.0082, 
    longitude = 28.9784, 
    h3_index = '891fb466257ffff'
WHERE email = 'ahmet@example.com' AND latitude IS NULL;

UPDATE users SET 
    latitude = 39.9334, 
    longitude = 32.8597, 
    h3_index = '891ea6992b7ffff'
WHERE email = 'mehmet@example.com' AND latitude IS NULL;

UPDATE users SET 
    latitude = 38.4237, 
    longitude = 27.1428, 
    h3_index = '891f1d6b89fffff'
WHERE email = 'ayse@example.com' AND latitude IS NULL;
