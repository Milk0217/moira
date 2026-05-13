use crate::get_lunar_mansion;

/// 计算上升点 (Ascendant) 的黄道经度
pub fn calculate_ascendant(lst_deg: f64, latitude: f64, obliquity_rad: f64) -> f64 {
    let lst_rad = lst_deg.to_radians();
    let lat_rad = latitude.to_radians();
    let eps = obliquity_rad;

    let x = -lst_rad.cos();
    let y = lst_rad.sin() * eps.cos() + lat_rad.tan() * eps.sin();

    let mut asc = y.atan2(x).to_degrees();
    if asc < 0.0 {
        asc += 360.0;
    }
    asc
}

/// 计算天顶 (MC / Midheaven) 的黄道经度
pub fn calculate_mc(lst_deg: f64, obliquity_rad: f64) -> f64 {
    let lst_rad = lst_deg.to_radians();
    let eps = obliquity_rad;

    let mut mc = lst_rad.tan().atan2(eps.cos()).to_degrees();
    // 调整象限: MC 应始终与 LST 在同一半区
    if mc < 0.0 {
        mc += 360.0;
    }
    // 若 LST 在 0-180°，MC 应在 0-180°；否则 180-360°
    let lst_norm = lst_deg % 360.0;
    let mc_norm = mc % 360.0;
    if lst_norm < 180.0 && mc_norm > 180.0 {
        mc -= 180.0;
    } else if lst_norm >= 180.0 && mc_norm < 180.0 {
        mc += 180.0;
    }
    if mc < 0.0 {
        mc += 360.0;
    } else if mc >= 360.0 {
        mc -= 360.0;
    }
    mc
}

/// 判断日夜生: true = 日生 (Sun above horizon)
/// Sun 在 Asc~Dsc 之间为日生
pub fn is_diurnal(ascendant: f64, sun_longitude: f64) -> bool {
    let mut diff = sun_longitude - ascendant;
    if diff < 0.0 {
        diff += 360.0;
    }
    diff < 180.0
}

/// 计算福点 (Part of Fortune)
pub fn calculate_part_of_fortune(
    ascendant: f64,
    sun_longitude: f64,
    moon_longitude: f64,
) -> f64 {
    let diurnal = is_diurnal(ascendant, sun_longitude);
    let fortune = if diurnal {
        // 日生: Asc + Moon - Sun
        ascendant + moon_longitude - sun_longitude
    } else {
        // 夜生: Asc + Sun - Moon
        ascendant + sun_longitude - moon_longitude
    };
    let mut f = fortune % 360.0;
    if f < 0.0 {
        f += 360.0;
    }
    f
}

/// 计算上升、天顶、福点
pub struct AngleData {
    pub ascendant: f64,
    pub midheaven: f64,
    pub part_of_fortune: f64,
    pub ascendant_mansion: String,
    pub ascendant_mansion_deg: f64,
    pub mc_mansion: String,
    pub mc_mansion_deg: f64,
    pub fortune_mansion: String,
    pub fortune_mansion_deg: f64,
}

impl AngleData {
    pub fn new(
        lst_deg: f64,
        latitude: f64,
        obliquity_rad: f64,
        sun_longitude: f64,
        moon_longitude: f64,
    ) -> Self {
        let ascendant = calculate_ascendant(lst_deg, latitude, obliquity_rad);
        let midheaven = calculate_mc(lst_deg, obliquity_rad);
        let part_of_fortune = calculate_part_of_fortune(ascendant, sun_longitude, moon_longitude);

        let (am, _, amd, _) = get_lunar_mansion(ascendant);
        let (mm, _, mmd, _) = get_lunar_mansion(midheaven);
        let (fm, _, fmd, _) = get_lunar_mansion(part_of_fortune);

        AngleData {
            ascendant,
            midheaven,
            part_of_fortune,
            ascendant_mansion: am.to_string(),
            ascendant_mansion_deg: amd,
            mc_mansion: mm.to_string(),
            mc_mansion_deg: mmd,
            fortune_mansion: fm.to_string(),
            fortune_mansion_deg: fmd,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_EPS: f64 = 23.44_f64.to_radians();

    #[test]
    fn test_ascendant_range() {
        let asc = calculate_ascendant(0.0, 40.0, TEST_EPS);
        assert!(asc >= 0.0 && asc < 360.0);
    }

    #[test]
    fn test_mc_range() {
        let mc = calculate_mc(0.0, TEST_EPS);
        assert!(mc >= 0.0 && mc < 360.0);
    }

    #[test]
    fn test_mc_90_degree_relation() {
        // LST 90° → MC 应约 90°
        let mc = calculate_mc(90.0, TEST_EPS);
        assert!((mc - 90.0).abs() < 2.0, "LST=90°时 MC 应≈90°, 实际={}", mc);
    }

    #[test]
    fn test_part_of_fortune_range() {
        let f = calculate_part_of_fortune(120.0, 30.0, 200.0);
        assert!(f >= 0.0 && f < 360.0);
    }

    #[test]
    fn test_diurnal_detection() {
        // Asc=120°, Sun=200° → Sun 在 Asc~Dsc(300°)之间 → 日生
        assert!(is_diurnal(120.0, 200.0));
        // Asc=120°, Sun=90° → Sun 在 Dsc~Asc 之间 → 夜生
        assert!(!is_diurnal(120.0, 90.0));
    }

    #[test]
    fn test_angle_data_creation() {
        let ad = AngleData::new(100.0, 40.0, TEST_EPS, 30.0, 200.0);
        assert!(ad.ascendant >= 0.0 && ad.ascendant < 360.0);
        assert!(ad.midheaven >= 0.0 && ad.midheaven < 360.0);
        assert!(ad.part_of_fortune >= 0.0 && ad.part_of_fortune < 360.0);
        assert!(!ad.ascendant_mansion.is_empty());
    }
}
