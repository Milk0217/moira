use anise::constants::frames::{EARTH_J2000, SUN_J2000};
use anise::prelude::*;

pub const SOLAR_TERMS: [(&str, f64); 24] = [
    ("小寒", 285.0),
    ("大寒", 300.0),
    ("立春", 315.0),
    ("雨水", 330.0),
    ("惊蛰", 345.0),
    ("春分", 0.0),
    ("清明", 15.0),
    ("谷雨", 30.0),
    ("立夏", 45.0),
    ("小满", 60.0),
    ("芒种", 75.0),
    ("夏至", 90.0),
    ("小暑", 105.0),
    ("大暑", 120.0),
    ("立秋", 135.0),
    ("处暑", 150.0),
    ("白露", 165.0),
    ("秋分", 180.0),
    ("寒露", 195.0),
    ("霜降", 210.0),
    ("立冬", 225.0),
    ("小雪", 240.0),
    ("大雪", 255.0),
    ("冬至", 270.0),
];

#[derive(Debug, Clone)]
pub struct SolarTerm {
    pub name: String,
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub julian_day: f64,
}

fn sun_longitude_at(ctx: &Almanac, epoch: Epoch) -> Result<f64, String> {
    let obl = super::calculate_obliquity(&epoch);
    let body = super::calculate_celestial_body(ctx, SUN_J2000, EARTH_J2000, epoch, "太阳", obl)?;
    Ok(body.longitude)
}

pub fn calculate_solar_terms(year: i32, ctx: &Almanac) -> Result<Vec<SolarTerm>, String> {
    let jan1 = Epoch::from_gregorian_tai_at_midnight(year, 1, 1);
    let jan1_jd = jan1.to_jde_utc_days();
    let jan1_lon = sun_longitude_at(ctx, jan1)?;

    let mut results = Vec::with_capacity(24);

    for &(name, target_lon) in &SOLAR_TERMS {
        let diff_lon = (target_lon - jan1_lon + 360.0) % 360.0;
        let approx_days = diff_lon / 360.0 * 365.25;
        let approx_jd = jan1_jd + approx_days;

        let mut low = approx_jd - 20.0;
        let mut high = approx_jd + 20.0;

        while high - low > 1.0 / 1440.0 {
            let mid = (low + high) / 2.0;
            let offset_seconds = (mid - jan1_jd) * 86400.0;
            let mid_epoch = if offset_seconds >= 0.0 {
                jan1 + hifitime::Duration::from_seconds(offset_seconds)
            } else {
                jan1 - hifitime::Duration::from_seconds(-offset_seconds)
            };
            let lon = sun_longitude_at(ctx, mid_epoch)?;

            let mut diff = lon - target_lon;
            if diff > 180.0 {
                diff -= 360.0;
            }
            if diff < -180.0 {
                diff += 360.0;
            }

            if diff > 0.0 {
                high = mid;
            } else {
                low = mid;
            }
        }

        let result_jd = (low + high) / 2.0;
        let (y, m, d, h, min) = jd_to_gregorian(result_jd);
        results.push(SolarTerm {
            name: name.to_string(),
            year: y,
            month: m,
            day: d,
            hour: h,
            minute: min,
            julian_day: result_jd,
        });
    }

    Ok(results)
}

fn jd_to_gregorian(jd: f64) -> (i32, u8, u8, u8, u8) {
    let z = (jd + 0.5) as i64;
    let a = ((z as f64 - 1867216.25) / 36524.25) as i64;
    let a2 = z + 1 + a - (a / 4);
    let b = a2 + 1524;
    let c = ((b as f64 - 122.1) / 365.25) as i64;
    let d = (365.25 * c as f64) as i64;
    let e = ((b - d) as f64 / 30.6001) as i64;
    let day = (b - d - (30.6001 * e as f64) as i64) as u8;
    let month = if e < 14 { (e - 1) as u8 } else { (e - 13) as u8 };
    let year = if month > 2 { c as i32 - 4716 } else { c as i32 - 4715 };
    let frac = jd - (jd.floor());
    let total_minutes = (frac * 24.0 * 60.0) as u32;
    let hour = (total_minutes / 60) as u8;
    let minute = (total_minutes % 60) as u8;
    (year, month, day, hour, minute)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_almanac;

    fn get_test_almanac() -> Result<Almanac, String> {
        let cwd = std::env::current_dir().unwrap_or_default();
        let paths = [
            "assets/bsp/de440.bsp",
            "../assets/bsp/de440.bsp",
            "../../assets/bsp/de440.bsp",
        ];
        for path in &paths {
            let full = cwd.join(path);
            if full.exists() {
                let path_str = full.to_string_lossy().to_string();
                return load_almanac(&path_str);
            }
        }
        if let Ok(path) = std::env::var("BSP_PATH") {
            return load_almanac(&path);
        }
        Err(format!("找不到星历文件, cwd={}", cwd.display()))
    }

    #[test]
    fn test_solar_terms_2024() {
        let ctx = match get_test_almanac() {
            Ok(c) => c,
            Err(_) => { eprintln!("Skipping test: BSP file not found"); return; }
        };
        let terms = calculate_solar_terms(2024, &ctx).expect("计算节气失败");

        assert_eq!(terms.len(), 24, "应有24个节气");

        let lichun = terms.iter().find(|t| t.name == "立春").expect("立春未找到");
        assert_eq!(lichun.month, 2);
        assert!(
            lichun.day >= 3 && lichun.day <= 5,
            "立春日不在预期范围(2/3-2/5): 实际={}/{}",
            lichun.month,
            lichun.day
        );

        let chunfen = terms.iter().find(|t| t.name == "春分").expect("春分未找到");
        assert_eq!(chunfen.month, 3);
        assert!(
            chunfen.day >= 19 && chunfen.day <= 21,
            "春分日不在预期范围(3/19-3/21): 实际={}/{}",
            chunfen.month,
            chunfen.day
        );

        let dongzhi = terms.iter().find(|t| t.name == "冬至").expect("冬至未找到");
        assert_eq!(dongzhi.month, 12);
        assert!(
            dongzhi.day >= 20 && dongzhi.day <= 22,
            "冬至日不在预期范围(12/20-12/22): 实际={}/{}",
            dongzhi.month,
            dongzhi.day
        );
    }
}
