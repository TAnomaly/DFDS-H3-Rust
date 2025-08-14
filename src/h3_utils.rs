use h3o::{CellIndex, LatLng, Resolution};

/// Latitude ve longitude'dan H3 index'i oluşturur
pub fn coords_to_h3(lat: f64, lng: f64, resolution: u8) -> Result<String, Box<dyn std::error::Error>> {
    let resolution = Resolution::try_from(resolution)?;
    let latlng = LatLng::new(lat, lng)?;
    let cell = latlng.to_cell(resolution);
    Ok(cell.to_string())
}

/// H3 index'ini latitude/longitude'a çevirir
pub fn h3_to_coords(h3_string: &str) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    let cell: CellIndex = h3_string.parse()?;
    let latlng = LatLng::from(cell);
    Ok((latlng.lat(), latlng.lng()))
}

/// H3 cell'in komşularını getirir
pub fn get_neighbors(h3_string: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cell: CellIndex = h3_string.parse()?;
    let neighbors: Vec<String> = cell
        .grid_disk::<Vec<_>>(1)
        .into_iter()
        .map(|cell| cell.to_string())
        .collect();
    Ok(neighbors)
}

/// H3 cell'in alanını km² cinsinden getirir
pub fn get_cell_area_km2(h3_string: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let cell: CellIndex = h3_string.parse()?;
    Ok(cell.area_km2())
}

/// İki H3 cell arasındaki mesafeyi hesaplar
pub fn distance_between_cells(h3_1: &str, h3_2: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let cell1: CellIndex = h3_1.parse()?;
    let cell2: CellIndex = h3_2.parse()?;
    let distance = cell1.grid_distance(cell2)?;
    Ok(distance as u32) // i32'den u32'ye çevir
}

/// H3 resolution seviyelerine göre açıklama
pub fn get_resolution_info(resolution: u8) -> &'static str {
    match resolution {
        0 => "~4,250,000 km² - Kıta düzeyinde",
        1 => "~607,000 km² - Ülke düzeyinde", 
        2 => "~86,700 km² - Eyalet/bölge düzeyinde",
        3 => "~12,400 km² - İl düzeyinde",
        4 => "~1,770 km² - İlçe düzeyinde",
        5 => "~253 km² - Büyük kent düzeyinde",
        6 => "~36.1 km² - Şehir düzeyinde",
        7 => "~5.16 km² - Mahalle düzeyinde",
        8 => "~0.737 km² - Semt düzeyinde",
        9 => "~0.105 km² - Sokak bloğu düzeyinde",
        10 => "~0.0151 km² - Apartman blok düzeyinde",
        11 => "~0.00216 km² - Ev düzeyinde",
        12 => "~0.000308 km² - Bina içi düzeyinde",
        13 => "~0.0000441 km² - Oda düzeyinde",
        14 => "~0.00000629 km² - Masa düzeyinde",
        15 => "~0.000000898 km² - Sandalye düzeyinde",
        _ => "Geçersiz resolution",
    }
}

/// Varsayılan H3 resolution (şehir düzeyi)
pub const DEFAULT_RESOLUTION: u8 = 7;

/// İki koordinat arasındaki gerçek mesafeyi km cinsinden hesaplar (Haversine formülü)
pub fn calculate_distance_km(lat1: f64, lng1: f64, lat2: f64, lng2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lng = (lng2 - lng1).to_radians();
    
    let a = (delta_lat / 2.0).sin() * (delta_lat / 2.0).sin() +
        lat1_rad.cos() * lat2_rad.cos() *
        (delta_lng / 2.0).sin() * (delta_lng / 2.0).sin();
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    EARTH_RADIUS_KM * c
}

/// H3 kRing - belirli bir mesafedeki tüm H3 cell'leri getirir
pub fn get_k_ring(h3_string: &str, k: u32) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cell: CellIndex = h3_string.parse()?;
    let ring_cells: Vec<String> = cell
        .grid_disk::<Vec<_>>(k)
        .into_iter()
        .map(|cell| cell.to_string())
        .collect();
    Ok(ring_cells)
}

/// Gerçek km mesafesine göre uygun k değerini hesaplar
pub fn calculate_k_for_distance(distance_km: f64, resolution: u8) -> u32 {
    // H3 resolution'a göre ortalama cell boyutları (km cinsinden)
    let avg_cell_size_km = match resolution {
        0 => 1107.0,
        1 => 418.0, 
        2 => 158.0,
        3 => 59.8,
        4 => 22.6,
        5 => 8.54,
        6 => 3.23,
        7 => 1.22, // Resolution 7 için ortalama cell boyutu
        8 => 0.461,
        9 => 0.174, // Varsayılan resolution
        10 => 0.0659,
        11 => 0.0249,
        12 => 0.00943,
        13 => 0.00357,
        14 => 0.00135,
        15 => 0.000509,
        _ => 1.22, // Varsayılan olarak resolution 7
    };
    
    // Kabaca kaç ring gerektiğini hesapla
    let k = (distance_km / avg_cell_size_km).ceil() as u32;
    k.max(1) // En az 1 ring
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_to_h3() {
        // İstanbul koordinatları
        let h3 = coords_to_h3(41.0082, 28.9784, 9).unwrap();
        assert!(!h3.is_empty());
    }

    #[test]
    fn test_h3_to_coords() {
        let h3 = coords_to_h3(41.0082, 28.9784, 9).unwrap();
        let (lat, lng) = h3_to_coords(&h3).unwrap();
        
        // Yaklaşık eşitlik kontrolü (H3 quantization nedeniyle)
        assert!((lat - 41.0082).abs() < 0.01);
        assert!((lng - 28.9784).abs() < 0.01);
    }
}
