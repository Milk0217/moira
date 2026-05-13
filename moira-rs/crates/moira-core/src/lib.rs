use anise::constants::frames::{
    EARTH_J2000, JUPITER_BARYCENTER_J2000, MARS_BARYCENTER_J2000, MERCURY_J2000, MOON_J2000,
    SATURN_BARYCENTER_J2000, SUN_J2000, VENUS_J2000,
};

use anise::math::cartesian::CartesianState;
use anise::math::Vector3;
use anise::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CelestialBodyData {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub speed: f64,
    pub zodiac_sign: (String, f64),
    pub mansion_name: String,
    pub mansion_degree: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExtraBodyData {
    pub name: String,
    pub longitude: f64,
    pub mansion_name: String,
    pub mansion_degree: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aspect {
    pub point1: String,
    pub point2: String,
    pub angle: f64,
    pub aspect_type: String,
    pub orb: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HouseData {
    pub index: u8,
    pub longitude: f64,
    pub mansion_name: String,
    pub mansion_degree: f64,
}

impl HouseData {
    pub fn new(index: u8, longitude: f64) -> Self {
        let (m_name, _, m_deg, _) = get_lunar_mansion(longitude);
        HouseData {
            index,
            longitude,
            mansion_name: m_name.to_string(),
            mansion_degree: m_deg,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShenSha {
    pub name: String,
    pub category: String,
    pub quality: String, // "吉" or "凶"
}

#[derive(Debug, Serialize)]
pub struct AstrologyData {
    pub timestamp: String,
    pub bodies: Vec<CelestialBodyData>,
    pub extra_bodies: Vec<ExtraBodyData>,
    pub aspects: Vec<Aspect>,
    pub houses: Vec<HouseData>,
    pub shen_sha: Vec<ShenSha>,
}

pub const ASPECT_TYPES: &[(f64, &str)] = &[
    (0.0, "合相"),
    (30.0, "半六分相"),
    (60.0, "六分相"),
    (90.0, "四分相"),
    (120.0, "三分相"),
    (150.0, "半四分相"),
    (180.0, "对分相"),
];

pub const ZODIAC_SIGNS: [&str; 12] = [
    "白羊", "金牛", "双子", "巨蟹", "狮子", "处女",
    "天秤", "天蝎", "射手", "摩羯", "水瓶", "双鱼",
];

/// 二十八宿数据: (名称, 四象, 宽度(度))
pub const LUNAR_MANSIONS: [(&str, &str, f64); 28] = [
    ("角", "青龙", 12.0),
    ("亢", "青龙", 9.0),
    ("氐", "青龙", 15.0),
    ("房", "青龙", 5.0),
    ("心", "青龙", 5.0),
    ("尾", "青龙", 18.0),
    ("箕", "青龙", 11.0),
    ("斗", "玄武", 26.0),
    ("牛", "玄武", 8.0),
    ("女", "玄武", 12.0),
    ("虚", "玄武", 10.0),
    ("危", "玄武", 17.0),
    ("室", "玄武", 16.0),
    ("壁", "玄武", 9.0),
    ("奎", "白虎", 16.0),
    ("娄", "白虎", 12.0),
    ("胃", "白虎", 14.0),
    ("昴", "白虎", 11.0),
    ("毕", "白虎", 16.0),
    ("觜", "白虎", 2.0),
    ("参", "白虎", 9.0),
    ("井", "朱雀", 33.0),
    ("鬼", "朱雀", 4.0),
    ("柳", "朱雀", 15.0),
    ("星", "朱雀", 7.0),
    ("张", "朱雀", 18.0),
    ("翼", "朱雀", 18.0),
    ("轸", "朱雀", 17.0),
];

const MU_EARTH: f64 = 398600.4415;

/// 根据历元计算真实黄赤交角 (IAU 2006)
pub fn calculate_obliquity(epoch: &Epoch) -> f64 {
    let jd = epoch.to_jde_utc_days();
    let t = (jd - 2451545.0) / 36525.0;
    // 标准 IAU 2006 公式 (单位: 度)
    let eps = 23.4392911111 - 0.0130101997 * t - 5.086e-8 * t.powi(2) + 5.565e-7 * t.powi(3);
    eps.to_radians()
}

pub fn get_body_state(
    ctx: &Almanac,
    target_frame: Frame,
    observer_frame: Frame,
    epoch: Epoch,
) -> Result<CartesianState, String> {
    ctx.translate(target_frame, observer_frame, epoch, None)
        .map_err(|e| format!("星体计算失败: {}", e))
}

pub fn celestial_body_from_state(state: &CartesianState, body_name: &str, obliquity_rad: f64) -> CelestialBodyData {
    let (ecliptic_longitude, ecliptic_latitude) =
        cartesian_to_ecliptic(state.radius_km, obliquity_rad);
    let speed = state.velocity_km_s.x.hypot(state.velocity_km_s.y) * 0.01;

    let sign_index = ((ecliptic_longitude / 30.0) as usize) % 12;
    let degree_in_sign = (ecliptic_longitude % 30.0).abs();
    let (m_name, _, m_deg, _) = get_lunar_mansion(ecliptic_longitude);

    CelestialBodyData {
        name: body_name.to_string(),
        longitude: ecliptic_longitude,
        latitude: ecliptic_latitude,
        speed,
        zodiac_sign: (ZODIAC_SIGNS[sign_index].to_string(), degree_in_sign),
        mansion_name: m_name.to_string(),
        mansion_degree: m_deg,
    }
}

fn cartesian_to_ecliptic(pos: Vector3, obliquity_rad: f64) -> (f64, f64) {
    let distance = pos.norm();
    if distance < 1e-10 {
        return (0.0, 0.0);
    }
    let declination = (pos.z / distance).asin();
    let mut right_ascension = pos.y.atan2(pos.x).to_degrees();
    if right_ascension < 0.0 {
        right_ascension += 360.0;
    }
    let ra_rad = right_ascension.to_radians();
    let dec_rad = declination;
    let eps = obliquity_rad;

    // 标准转换: 赤道(α,δ) → 黄道(λ,β)
    // cos(δ)*cos(α) = cos(λ)*cos(β)
    // sin(δ)*sin(ε) + cos(δ)*cos(ε)*sin(α) = sin(λ)*cos(β)
    // sin(δ)*cos(ε) - cos(δ)*sin(ε)*sin(α) = sin(β)
    let x = dec_rad.cos() * ra_rad.cos();
    let y = dec_rad.sin() * eps.sin() + dec_rad.cos() * eps.cos() * ra_rad.sin();
    let z = dec_rad.sin() * eps.cos() - dec_rad.cos() * eps.sin() * ra_rad.sin();

    let mut ecliptic_longitude = y.atan2(x).to_degrees();
    if ecliptic_longitude < 0.0 {
        ecliptic_longitude += 360.0;
    }
    let ecliptic_latitude = z.asin().to_degrees();
    (ecliptic_longitude, ecliptic_latitude)
}

pub fn calculate_celestial_body(
    ctx: &Almanac,
    target_frame: Frame,
    observer_frame: Frame,
    epoch: Epoch,
    body_name: &str,
    obliquity_rad: f64,
) -> Result<CelestialBodyData, String> {
    let state = get_body_state(ctx, target_frame, observer_frame, epoch)?;
    Ok(celestial_body_from_state(&state, body_name, obliquity_rad))
}

fn extra_body(name: &str, longitude: f64) -> ExtraBodyData {
    let (m_name, _, m_deg, _) = get_lunar_mansion(longitude);
    ExtraBodyData {
        name: name.to_string(),
        longitude,
        mansion_name: m_name.to_string(),
        mansion_degree: m_deg,
    }
}

pub fn calculate_lunar_nodes(state: &CartesianState) -> (ExtraBodyData, ExtraBodyData) {
    let h = state.radius_km.cross(&state.velocity_km_s);
    let n = Vector3::new(0.0, 0.0, 1.0).cross(&h);
    let mut rahu = n.y.atan2(n.x).to_degrees();
    if rahu < 0.0 {
        rahu += 360.0;
    }
    let mut ketu = rahu + 180.0;
    if ketu >= 360.0 {
        ketu -= 360.0;
    }
    (extra_body("罗睺", rahu), extra_body("计都", ketu))
}

pub fn calculate_lunar_apogee(state: &CartesianState) -> ExtraBodyData {
    let r = state.radius_km;
    let v = state.velocity_km_s;
    let h = r.cross(&v);
    let r_mag = r.norm();
    let ecc = v.cross(&h) / MU_EARTH - r / r_mag;
    let ecc_mag = ecc.norm();

    let perigee = if ecc_mag > 1e-10 {
        let mut p = ecc.y.atan2(ecc.x).to_degrees();
        if p < 0.0 { p += 360.0; }
        p
    } else { 0.0 };

    let mut apogee = perigee + 180.0;
    if apogee >= 360.0 { apogee -= 360.0; }
    extra_body("月孛", apogee)
}

/// 紫炁计算。
/// 传统算法各流派不同。主算法：取木星与太阳黄经的中点（紫炁为木星之馀气），
/// 在某些流派无人调用时可回退至罗睺+120°。
pub fn calculate_ziqui(rahu_longitude: f64, sun_longitude: Option<f64>, jupiter_longitude: Option<f64>) -> ExtraBodyData {
    let lon = match (jupiter_longitude, sun_longitude) {
        (Some(jup), Some(sun)) => {
            let mut m = (jup + sun) / 2.0;
            if m < 0.0 { m += 360.0; }
            if m >= 360.0 { m -= 360.0; }
            m
        }
        _ => {
            let mut z = rahu_longitude + 120.0;
            if z >= 360.0 { z -= 360.0; }
            z
        }
    };
    extra_body("紫炁", lon)
}

const ASPECT_ORB: f64 = 0.1;

pub fn calculate_aspects(
    bodies: &[CelestialBodyData],
    extras: &[ExtraBodyData],
) -> Vec<Aspect> {
    let mut aspects = Vec::new();

    for (i, b1) in bodies.iter().enumerate() {
        for b2 in bodies.iter().skip(i + 1) {
            push_aspect(b1.longitude, b2.longitude, &b1.name, &b2.name, &mut aspects);
        }
        for ext in extras {
            push_aspect(b1.longitude, ext.longitude, &b1.name, &ext.name, &mut aspects);
        }
    }
    aspects
}

fn push_aspect(lon1: f64, lon2: f64, n1: &str, n2: &str, aspects: &mut Vec<Aspect>) {
    let raw = (lon1 - lon2).abs();
    let angle = if raw > 180.0 { 360.0 - raw } else { raw };
    for &(target, name) in ASPECT_TYPES {
        let diff = (angle - target).abs();
        if diff <= ASPECT_ORB {
            aspects.push(Aspect {
                point1: n1.to_string(),
                point2: n2.to_string(),
                angle,
                aspect_type: name.to_string(),
                orb: diff,
            });
        }
    }
}

pub fn get_lunar_mansion(ecliptic_longitude: f64) -> (&'static str, &'static str, f64, f64) {
    let total_width: f64 = LUNAR_MANSIONS.iter().map(|(_, _, w)| w).sum();
    let normalized = ecliptic_longitude % 360.0;
    let pos = (normalized / 360.0) * total_width;
    let mut accum = 0.0;
    for &(name, direction, width) in &LUNAR_MANSIONS {
        accum += width;
        if pos < accum {
            let degree_in_mansion = pos - (accum - width);
            return (name, direction, degree_in_mansion, width);
        }
    }
    let (name, direction, width) = LUNAR_MANSIONS[27];
    (name, direction, width, width)
}

/// 计算 GMST 和当地恒星时
pub fn local_sidereal_time(epoch: &Epoch, longitude_deg: f64) -> f64 {
    let jd = epoch.to_jde_utc_days();
    let t = (jd - 2451545.0) / 36525.0;
    let gmst = 280.46061837
        + 360.98564736629 * (jd - 2451545.0)
        + 0.000387933 * t.powi(2)
        - t.powi(3) / 38710000.0;
    let gmst = gmst % 360.0;
    let mut lst = gmst + longitude_deg;
    if lst < 0.0 {
        lst += 360.0;
    } else if lst >= 360.0 {
        lst -= 360.0;
    }
    lst
}

const BRANCH_NAMES: [&str; 12] =
    ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

/// 根据出生年份的地支推算年支神煞
pub fn calculate_shen_sha(year: i32) -> Vec<ShenSha> {
    let branch = ((year - 1984) % 12 + 12) as usize % 12;
    let stems = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    let stem = ((year - 1984) % 10 + 10) as usize % 10;

    let mut list = vec![
        ShenSha { name: format!("太岁 {}", BRANCH_NAMES[branch]), category: "年支".into(), quality: "凶".into() },
        ShenSha { name: format!("岁破 {}", BRANCH_NAMES[(branch + 6) % 12]), category: "年支".into(), quality: "凶".into() },
    ];

    let lookup = |table: &[[usize; 4]]| -> Option<usize> {
        for &g in table {
            if g[..3].contains(&branch) { return Some(g[3]); }
        }
        None
    };

    // 驿马
    if let Some(v) = lookup(&[[2, 6, 10, 8], [3, 9, 1, 11], [8, 0, 4, 2], [11, 3, 7, 5]]) {
        list.push(ShenSha { name: format!("驿马 {}", BRANCH_NAMES[v]), category: "年支".into(), quality: "吉".into() });
    }
    // 桃花
    if let Some(v) = lookup(&[[2, 6, 10, 3], [3, 9, 1, 6], [8, 0, 4, 9], [11, 3, 7, 0]]) {
        list.push(ShenSha { name: format!("桃花 {}", BRANCH_NAMES[v]), category: "年支".into(), quality: "吉".into() });
    }
    // 劫煞 (申子辰→巳, 寅午戌→亥, 巳酉丑→寅, 亥卯未→申)
    if let Some(v) = lookup(&[[8, 0, 4, 5], [2, 6, 10, 11], [3, 9, 1, 2], [11, 3, 7, 8]]) {
        list.push(ShenSha { name: format!("劫煞 {}", BRANCH_NAMES[v]), category: "年支".into(), quality: "凶".into() });
    }
    // 灾煞 (申子辰→午, 寅午戌→子, 巳酉丑→卯, 亥卯未→酉)
    if let Some(v) = lookup(&[[8, 0, 4, 6], [2, 6, 10, 0], [3, 9, 1, 3], [11, 3, 7, 9]]) {
        list.push(ShenSha { name: format!("灾煞 {}", BRANCH_NAMES[v]), category: "年支".into(), quality: "凶".into() });
    }

    // 年干神煞
    list.push(ShenSha { name: format!("禄神 {}", stems[stem]), category: "年干".into(), quality: "吉".into() });
    // 天乙贵人 (简化: 甲戊→牛羊, 乙己→鼠猴, ...)
    let tian_yi = match stem { 0 | 4 => "丑未", 1 | 5 => "子申", 2 | 6 => "酉亥", 3 | 7 => "卯巳", 8 | 9 => "午寅", _ => "" };
    list.push(ShenSha { name: format!("天乙贵人 {}", tian_yi), category: "年干".into(), quality: "吉".into() });

    list
}

/// 计算 Ascendant（上升点），Equal House 系统
pub fn calculate_houses(epoch: &Epoch, latitude: f64, longitude: f64, obliquity_rad: f64) -> Vec<HouseData> {
    let lst = local_sidereal_time(epoch, longitude);
    let lst_rad = lst.to_radians();
    let lat_rad = latitude.to_radians();
    let eps = obliquity_rad;

    // Ascendant 公式
    let asc_rad = (lst_rad.sin() * eps.cos() + lat_rad.tan() * eps.sin())
        .atan2(-lst_rad.cos());

    let asc_deg = {
        let mut d = asc_rad.to_degrees();
        if d < 0.0 { d += 360.0; }
        d
    };

    // Equal House: 每宫 30°
    (0..12)
        .map(|i| {
            let mut h = asc_deg + (i as f64) * 30.0;
            if h >= 360.0 { h -= 360.0; }
            HouseData::new((i + 1) as u8, h)
        })
        .collect()
}

pub fn load_almanac(bsp_path: &str) -> Result<Almanac, String> {
    let spk = SPK::load(bsp_path).map_err(|e| format!("加载星历失败: {}", e))?;
    Ok(Almanac::from_spk(spk))
}

pub fn calculate_chart(ctx: &Almanac, epoch: Epoch, latitude: f64, longitude: f64) -> AstrologyData {
    let obliquity_rad = calculate_obliquity(&epoch);
    let body_frames: [(Frame, &str); 7] = [
        (SUN_J2000, "太阳"),
        (MOON_J2000, "太阴"),
        (MERCURY_J2000, "水星"),
        (VENUS_J2000, "金星"),
        (MARS_BARYCENTER_J2000, "火星"),
        (JUPITER_BARYCENTER_J2000, "木星"),
        (SATURN_BARYCENTER_J2000, "土星"),
    ];

    let mut bodies = Vec::new();
    let mut moon_state: Option<CartesianState> = None;

    for &(frame, name) in &body_frames {
        match calculate_celestial_body(ctx, frame, EARTH_J2000, epoch, name, obliquity_rad) {
            Ok(b) => {
                if name == "太阴" {
                    if let Ok(st) = get_body_state(ctx, frame, EARTH_J2000, epoch) {
                        moon_state = Some(st);
                    }
                }
                bodies.push(b);
            }
            Err(e) => log::warn!("计算 {} 失败: {}", name, e),
        }
    }

    let sun_lon = bodies.iter().find(|b| b.name == "太阳").map(|b| b.longitude);
    let jup_lon = bodies.iter().find(|b| b.name == "木星").map(|b| b.longitude);

    let extra_bodies = moon_state
        .as_ref()
        .map(|ms| {
            let (rahu, ketu) = calculate_lunar_nodes(ms);
            let apogee = calculate_lunar_apogee(ms);
            let ziqui = calculate_ziqui(rahu.longitude, sun_lon, jup_lon);
            vec![rahu, ketu, apogee, ziqui]
        })
        .unwrap_or_default();

    let aspects = calculate_aspects(&bodies, &extra_bodies);
    let houses = calculate_houses(&epoch, latitude, longitude, obliquity_rad);
    let shen_sha = calculate_shen_sha(epoch.year());

    AstrologyData {
        timestamp: epoch.to_string(),
        bodies,
        extra_bodies,
        aspects,
        houses,
        shen_sha,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_EPS: f64 = 23.44_f64.to_radians();

    fn test_frame() -> Frame {
        Frame { ephemeris_id: 10, orientation_id: 1, mu_km3_s2: None, shape: None }
    }
    fn moon_test_frame() -> Frame {
        Frame { ephemeris_id: 301, orientation_id: 1, mu_km3_s2: None, shape: None }
    }

    #[test]
    fn test_zodiac_array() {
        assert_eq!(ZODIAC_SIGNS[0], "白羊");
        assert_eq!(ZODIAC_SIGNS[11], "双鱼");
        assert_eq!(ZODIAC_SIGNS.len(), 12);
    }

    #[test]
    fn test_mansion_total() {
        let total: f64 = LUNAR_MANSIONS.iter().map(|(_, _, w)| w).sum();
        assert!((total - 365.0).abs() < 2.0, "二十八宿宽度和应≈365°, 实际={}", total);
    }

    #[test]
    fn test_mansion_boundaries() {
        let (name, _, deg, _) = get_lunar_mansion(0.0);
        assert_eq!(name, "角");
        assert!((deg - 0.0).abs() < 0.01);

        let (name, _, _, _) = get_lunar_mansion(350.0);
        assert_eq!(name, "轸");
    }

    #[test]
    fn test_cartesian_sun_ecliptic() {
        let state = CartesianState {
            radius_km: Vector3::new(1.521e8, 0.0, 0.0),
            velocity_km_s: Vector3::new(0.0, 29.8, 0.0),
            epoch: Epoch::from_tdb_seconds(0.0),
            frame: test_frame(),
        };
        let (lon, lat) = cartesian_to_ecliptic(state.radius_km, TEST_EPS);
        assert_eq!(lat, 0.0, "太阳黄纬应为0°");
        assert!((lon - 0.0).abs() < 0.01 || (lon - 180.0).abs() < 0.01);
    }

    #[test]
    fn test_cartesian_off_ecliptic() {
        let pos = Vector3::new(1.0, 0.0, 0.4);
        let (_, lat) = cartesian_to_ecliptic(pos, TEST_EPS);
        assert!(lat.abs() > 0.0, "非黄道面上的点应有黄纬≠0");
        assert!(lat.abs() < 90.0);
    }

    #[test]
    fn test_lunar_nodes_opposite() {
        let state = CartesianState {
            radius_km: Vector3::new(380000.0, 0.0, 0.0),
            velocity_km_s: Vector3::new(0.0, 1.02, 0.0),
            epoch: Epoch::from_tdb_seconds(0.0),
            frame: moon_test_frame(),
        };
        let (rahu, ketu) = calculate_lunar_nodes(&state);
        let diff = (rahu.longitude - ketu.longitude).abs();
        assert!((diff - 180.0).abs() < 1.0, "罗睺计都应该相差180°, 实际={}", diff);
    }

    #[test]
    fn test_aspects_empty() {
        let aspects = calculate_aspects(&[], &[]);
        assert!(aspects.is_empty());
    }

    #[test]
    fn test_aspects_basic() {
        let b1 = CelestialBodyData {
            name: "太阳".into(), longitude: 0.0, latitude: 0.0, speed: 1.0,
            zodiac_sign: ("白羊".into(), 0.0),
            mansion_name: "角".into(), mansion_degree: 0.0,
        };
        let b2 = CelestialBodyData {
            name: "太阴".into(), longitude: 180.0, latitude: 0.0, speed: 13.0,
            zodiac_sign: ("天秤".into(), 0.0),
            mansion_name: "角".into(), mansion_degree: 0.0,
        };
        let aspects = calculate_aspects(&[b1, b2], &[]);
        assert!(!aspects.is_empty(), "180° should be an opposition");
        assert_eq!(aspects[0].aspect_type, "对分相");
    }

    #[test]
    fn test_mansion_consistency() {
        for i in 0..360 {
            let (name, _, _, _) = get_lunar_mansion(i as f64);
            assert!(!name.is_empty(), "每个经度都应落在某个宿中");
        }
    }

    #[test]
    fn test_speed_positive() {
        let body = celestial_body_from_state(
            &CartesianState {
                radius_km: Vector3::new(1.0, 1.0, 0.0),
                velocity_km_s: Vector3::new(0.0, 30.0, 0.0),
                epoch: Epoch::from_tdb_seconds(0.0),
                frame: test_frame(),
            },
            "测试体", TEST_EPS,
        );
        assert!(body.speed >= 0.0);
    }

    #[test]
    fn test_ziqui_fallback() {
        let zq = calculate_ziqui(50.0, None, None);
        assert!((zq.longitude - 170.0).abs() < 0.01, "回退罗睺+120°, 实际={}", zq.longitude);
    }

    #[test]
    fn test_ziqui_jupiter_midpoint() {
        let zq = calculate_ziqui(50.0, Some(200.0), Some(100.0));
        // (木100 + 日200) / 2 = 150
        assert!((zq.longitude - 150.0).abs() < 0.01, "日月木中点应为150°, 实际={}", zq.longitude);
    }

    #[test]
    fn test_obliquity_range() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        let eps = calculate_obliquity(&epoch);
        assert!(eps > 0.40 && eps < 0.42, "黄赤交角应在~23.44°附近");
    }

    #[test]
    fn test_shen_sha_not_empty() {
        let list = calculate_shen_sha(2000);
        assert!(!list.is_empty(), "神煞列表不应为空");
        assert!(list.len() >= 7, "应至少有7种神煞, 实际={}", list.len());
    }

    #[test]
    fn test_shen_sha_2000() {
        let list = calculate_shen_sha(2000);
        assert!(list.iter().any(|s| s.name.contains("太岁")));
        assert!(list.iter().any(|s| s.name.contains("禄神")));
        assert!(list.iter().any(|s| s.name.contains("天乙")));
    }

    #[test]
    fn test_load_almanac_error() {
        let result = load_almanac("/nonexistent/path.bsp");
        assert!(result.is_err(), "不存在的路径应返回错误");
    }

    #[test]
    fn test_local_sidereal_time() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        let lst = local_sidereal_time(&epoch, 0.0);
        assert!(lst >= 0.0 && lst < 360.0);
    }

    #[test]
    fn test_houses_12() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        let houses = calculate_houses(&epoch, 40.0, -74.0, TEST_EPS);
        assert_eq!(houses.len(), 12);
        for h in &houses {
            assert!(h.longitude >= 0.0 && h.longitude < 360.0);
            assert!(h.index >= 1 && h.index <= 12);
            assert!(!h.mansion_name.is_empty());
        }
    }

    #[test]
    fn test_houses_equal_spacing() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        let houses = calculate_houses(&epoch, 40.0, -74.0, TEST_EPS);
        let mut prev = houses[0].longitude;
        for i in 1..houses.len() {
            let mut diff = houses[i].longitude - prev;
            if diff < 0.0 { diff += 360.0; }
            assert!((diff - 30.0).abs() < 0.01, "等宫制每宫应30°, 第{}宫差={}", i+1, diff);
            prev = houses[i].longitude;
        }
    }
}
