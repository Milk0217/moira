use anise::constants::frames::{EARTH_J2000, SUN_J2000, MOON_J2000};
use anise::prelude::{Almanac, Epoch};
use hifitime::Duration;

/// 搜索指定年份范围内的日月食
pub fn search_eclipses(
    start_year: i32,
    end_year: i32,
    ctx: &Almanac,
) -> Vec<(String, String, f64)> {
    let mut results = Vec::new();
    let start = Epoch::from_gregorian_utc_at_midnight(start_year, 1, 1);
    let end = Epoch::from_gregorian_utc_at_midnight(end_year + 1, 1, 1);
    let one_day = Duration::from_days(1.0);
    let max_lon_diff = 18.0;
    let max_moon_lat = 1.5;

    let mut current = start;
    while current < end {
        let obliquity = crate::calculate_obliquity(&current);
        let (sun_lon, moon_lon, moon_lat) = match (
            crate::get_body_state(ctx, SUN_J2000, EARTH_J2000, current),
            crate::get_body_state(ctx, MOON_J2000, EARTH_J2000, current),
        ) {
            (Ok(sun), Ok(moon)) => {
                let (sun_lon, _) = crate::cartesian_to_ecliptic(sun.radius_km, obliquity);
                let (moon_lon, moon_lat) = crate::cartesian_to_ecliptic(moon.radius_km, obliquity);
                (sun_lon, moon_lon, moon_lat)
            }
            _ => { current += one_day; continue; }
        };

        let raw_diff = (sun_lon - moon_lon + 360.0) % 360.0;
        let new_moon_dist = raw_diff.min(360.0 - raw_diff);
        let full_moon_dist = (raw_diff - 180.0).abs();

        for (dist, etype) in [(new_moon_dist, "solar"), (full_moon_dist, "lunar")] {
            if dist < max_lon_diff && moon_lat.abs() < max_moon_lat {
                let mag = ((1.0 - (moon_lat.abs() / max_moon_lat).powi(2)) * 0.7
                    + (1.0 - (dist / max_lon_diff).powi(2)) * 0.3) * 100.0;
                let (y, m, d, _, _, _, _) = current.to_gregorian_utc();
                results.push((format!("{:04}-{:02}-{:02}", y, m, d), etype.to_string(), mag));
            }
        }
        current += one_day;
    }
    results.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    results
}

/// 简单地磁偏角模型（多项式近似）
pub fn magnetic_declination(latitude: f64, longitude: f64, year: f64) -> f64 {
    let lat_rad = latitude.to_radians();
    let lon_rad = longitude.to_radians();
    10.0 * (0.3 * lon_rad).sin()
        + 5.0 * lat_rad.sin() * (lon_rad - 0.5).cos()
        - 0.2 * (year - 2020.0)
        - 2.0 * (2.0 * lat_rad).cos() * (lon_rad + 1.0).sin()
}

/// 应用地磁偏角修正
pub fn apply_magnetic_declination(bearing_deg: f64, declination_deg: f64, is_magnetic: bool) -> f64 {
    if !is_magnetic { return bearing_deg; }
    ((bearing_deg + declination_deg) % 360.0 + 360.0) % 360.0
}

/// 太阳到山搜索（逐日）
pub fn search_sun_to_mountain(
    target_azimuth: f64, start_jd: f64, end_jd: f64,
    latitude: f64, longitude: f64, timezone: f64,
    ctx: &Almanac,
) -> Vec<(f64, f64, f64)> {
    let mut results = Vec::new();
    let mut current_jd = start_jd;
    while current_jd < end_jd {
        let local_noon_jd = current_jd + (12.0 - timezone) / 24.0;
        let epoch = crate::utils::epoch_from_jd(local_noon_jd);
        if let Ok(state) = crate::get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch) {
            let obl = crate::calculate_obliquity(&state.epoch);
            let (sun_lon, sun_lat) = crate::cartesian_to_ecliptic(state.radius_km, obl);
            let lst = crate::local_sidereal_time(&state.epoch, longitude);
            let (az, alt) = crate::get_body_horizontal(sun_lon, sun_lat, lst, latitude, obl);
            let mut diff = (az - target_azimuth).abs();
            if diff > 180.0 { diff = 360.0 - diff; }
            if diff < 5.0 && alt > 0.0 { results.push((current_jd, az, alt)); }
        }
        current_jd += 1.0;
    }
    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    results.dedup_by(|a, b| (a.0 - b.0).abs() < 2.0);
    results
}

/// 太阳到山搜索（逐小时）
pub fn search_sun_at_date(
    target_azimuth: f64, date_jd: f64,
    latitude: f64, longitude: f64, timezone: f64,
    ctx: &Almanac,
) -> Vec<(f64, f64, f64)> {
    let mut results = Vec::new();
    let mut local_hour = 6.0;
    while local_hour <= 18.0 {
        let jd = date_jd + (local_hour - timezone) / 24.0;
        let epoch = crate::utils::epoch_from_jd(jd);
        if let Ok(state) = crate::get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch) {
            let obl = crate::calculate_obliquity(&state.epoch);
            let (sun_lon, sun_lat) = crate::cartesian_to_ecliptic(state.radius_km, obl);
            let lst = crate::local_sidereal_time(&state.epoch, longitude);
            let (az, alt) = crate::get_body_horizontal(sun_lon, sun_lat, lst, latitude, obl);
            let mut diff = (az - target_azimuth).abs();
            if diff > 180.0 { diff = 360.0 - diff; }
            if diff < 5.0 && alt > 0.0 { results.push((jd, az, alt)); }
        }
        local_hour += 10.0 / 60.0;
    }
    results
}
