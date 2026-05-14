pub mod angles;
pub mod bazi;
pub mod rule_engine;
pub mod solar_terms;

use anise::constants::frames::{
    EARTH_J2000, JUPITER_BARYCENTER_J2000, MARS_BARYCENTER_J2000, MERCURY_J2000, MOON_J2000,
    NEPTUNE_BARYCENTER_J2000, PLUTO_BARYCENTER_J2000, SATURN_BARYCENTER_J2000, SUN_J2000,
    URANUS_BARYCENTER_J2000, VENUS_J2000,
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
     pub zodiac_sign: String,
     pub zodiac_degree: f64,
     pub mansion_name: String,
     pub mansion_degree: f64,
     pub coordinate_system: String,
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
    pub quality: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AstrologyData {
    pub timestamp: String,
    pub bodies: Vec<CelestialBodyData>,
    pub extra_bodies: Vec<ExtraBodyData>,
    pub aspects: Vec<Aspect>,
    pub houses: Vec<HouseData>,
    pub shen_sha: Vec<ShenSha>,
    pub ascendant: f64,
    pub midheaven: f64,
    pub descendant: f64,
    pub imum_coeli: f64,
    pub part_of_fortune: f64,
    pub bazi: bazi::BaziData,
    pub shiganhuayao: Vec<(String, String)>,
    pub ming_zhu: String,
    pub shen_zhu: String,
    pub ming_du_zhu: String,
    pub ming_gong_zhu: String,
    pub shen_gong_zhu: String,
    pub xijige: Vec<(String, String)>,
    pub xiaoxian_result: (String, usize),
    pub yuexian_result: (String, usize),
    pub dongweifeixian_result: Vec<(u32, String, String)>,
    pub zodiac_type: String,
    pub ayanamsa: f64,
    pub dst_applied: bool,
    pub coordinate_system: String,
    pub star_powers: Option<Vec<rule_engine::StarPower>>,
    pub house_analyses: Option<Vec<rule_engine::HouseAnalysis>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_orbs: Option<Vec<(f64, f64)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_shensha: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_ziqui_offset: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_dayun_start_age: Option<i32>,
}

impl Default for CustomRules {
    fn default() -> Self {
        CustomRules {
            aspect_orbs: None,
            enabled_shensha: None,
            custom_ziqui_offset: None,
            custom_dayun_start_age: None,
        }
    }
}

pub fn load_custom_rules(path: &str) -> Result<CustomRules, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(CustomRules::default()),
        Err(e) => return Err(format!("读取自定义规则文件失败: {}", e)),
    };
    serde_json::from_str(&content).map_err(|e| format!("解析自定义规则文件失败: {}", e))
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

/// 二十八宿度主星 (木金土日月火水 循环4次)
pub const MANSION_DEGREE_MASTERS: [&str; 28] = [
    "木星","金星","土星","太阳","太阴","火星","水星",
    "木星","金星","土星","太阳","太阴","火星","水星",
    "木星","金星","土星","太阳","太阴","火星","水星",
    "木星","金星","土星","太阳","太阴","火星","水星",
];

/// 十二宫(黄道12宫)宫主星
pub const SIGN_HOUSE_MASTERS: [&str; 12] = [
    // 子丑寅卯辰巳午未申酉戌亥 — 对应西式12宫
    "土星","土星","木星","金星","金星","水星","太阳","太阴","水星","金星","火星","木星",
];
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

pub fn calculate_precession_offset(epoch: &Epoch) -> f64 {
    let jd = epoch.to_jde_utc_days();
    let years_since_j2000 = (jd - 2451545.0) / 365.25;
    22.438 + years_since_j2000 * 50.256 / 3600.0
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
         zodiac_sign: ZODIAC_SIGNS[sign_index].to_string(),
         zodiac_degree: degree_in_sign,
         mansion_name: m_name.to_string(),
         mansion_degree: m_deg,
         coordinate_system: "ecliptic".to_string(),
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

pub fn calculate_body_with_zodiac_type(
    ctx: &Almanac,
    target_frame: Frame,
    observer_frame: Frame,
    epoch: Epoch,
    body_name: &str,
    obliquity_rad: f64,
    use_sidereal: bool,
) -> CelestialBodyData {
let mut body = calculate_celestial_body(ctx, target_frame, observer_frame, epoch, body_name, obliquity_rad)
         .unwrap_or_else(|_| CelestialBodyData {
             name: body_name.to_string(),
             longitude: 0.0,
             latitude: 0.0,
             speed: 0.0,
             zodiac_sign: "白羊".to_string(),
             zodiac_degree: 0.0,
             mansion_name: "角".to_string(),
             mansion_degree: 0.0,
             coordinate_system: "ecliptic".to_string(),
         });
     if use_sidereal {
         let ayanamsa = calculate_precession_offset(&epoch);
         let mut lon = body.longitude - ayanamsa;
         if lon < 0.0 {
             lon += 360.0;
         }
         body.longitude = lon;
         let sign_index = ((lon / 30.0) as usize) % 12;
         let degree_in_sign = (lon % 30.0).abs();
         body.zodiac_sign = ZODIAC_SIGNS[sign_index].to_string();
         body.zodiac_degree = degree_in_sign;
         let (m_name, _, m_deg, _) = get_lunar_mansion(lon);
         body.mansion_name = m_name.to_string();
         body.mansion_degree = m_deg;
     }
     body
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

pub const DEFAULT_ASPECT_ORB: f64 = 8.0;

pub fn calculate_aspects(
    bodies: &[CelestialBodyData],
    extras: &[ExtraBodyData],
) -> Vec<Aspect> {
    calculate_aspects_with_orb(bodies, extras, DEFAULT_ASPECT_ORB)
}

pub fn calculate_aspects_with_orb(
    bodies: &[CelestialBodyData],
    extras: &[ExtraBodyData],
    orb: f64,
) -> Vec<Aspect> {
    let mut aspects = Vec::new();

    for (i, b1) in bodies.iter().enumerate() {
        for b2 in bodies.iter().skip(i + 1) {
            push_aspect(b1.longitude, b2.longitude, &b1.name, &b2.name, &mut aspects, orb);
        }
        for ext in extras {
            push_aspect(b1.longitude, ext.longitude, &b1.name, &ext.name, &mut aspects, orb);
        }
    }
    aspects
}

fn push_aspect(lon1: f64, lon2: f64, n1: &str, n2: &str, aspects: &mut Vec<Aspect>, orb: f64) {
    let raw = (lon1 - lon2).abs();
    let angle = if raw > 180.0 { 360.0 - raw } else { raw };
    for &(target, name) in ASPECT_TYPES {
        let diff = (angle - target).abs();
        if diff <= orb {
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

pub fn equatorial_to_horizontal(hour_angle: f64, declination: f64, latitude: f64) -> (f64, f64) {
    let ha_rad = hour_angle.to_radians();
    let dec_rad = declination.to_radians();
    let lat_rad = latitude.to_radians();

    let alt = (dec_rad.sin() * lat_rad.sin() + dec_rad.cos() * lat_rad.cos() * ha_rad.cos()).asin();
    let az = ((-ha_rad.sin() * dec_rad.cos()).atan2(
        dec_rad.sin() * lat_rad.cos() - dec_rad.cos() * lat_rad.sin() * ha_rad.cos(),
    ))
    .to_degrees();

    let altitude = alt.to_degrees();
    let azimuth = (az + 360.0) % 360.0;
    (azimuth, altitude)
}

pub fn ecliptic_to_equatorial(ecliptic_lon: f64, ecliptic_lat: f64, obliquity_rad: f64) -> (f64, f64) {
    let lon_rad = ecliptic_lon.to_radians();
    let lat_rad = ecliptic_lat.to_radians();
    let eps = obliquity_rad;

    let ra = (lon_rad.sin() * eps.cos() - lat_rad.tan() * eps.sin())
        .atan2(lon_rad.cos())
        .to_degrees();
    let dec = (lat_rad.sin() * eps.cos() + lat_rad.cos() * eps.sin() * lon_rad.sin())
        .asin()
        .to_degrees();

    let right_ascension = (ra + 360.0) % 360.0;
    (right_ascension, dec)
}

pub fn calculate_hour_angle(lst_deg: f64, right_ascension: f64) -> f64 {
    let mut ha = lst_deg - right_ascension;
    if ha < 0.0 {
        ha += 360.0;
    }
    ha
}

pub fn get_body_horizontal(
    body_longitude: f64,
    body_latitude: f64,
    lst_deg: f64,
    latitude: f64,
    obliquity_rad: f64,
) -> (f64, f64) {
    let (ra, dec) = ecliptic_to_equatorial(body_longitude, body_latitude, obliquity_rad);
    let ha = calculate_hour_angle(lst_deg, ra);
    equatorial_to_horizontal(ha, dec, latitude)
}

const BRANCH_NAMES: [&str; 12] =
    ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];

const YEAR_BRANCH_SPIRITS: [(&str, &str, &str, &str); 12] = [
    ("将星", "吉", "华盖", "吉"),
    ("暗曜", "凶", "福星", "吉"),
    ("岁前", "凶", "丧门", "凶"),
    ("勾神", "凶", "绞神", "凶"),
    ("孤辰", "凶", "寡宿", "凶"),
    ("福星", "吉", "天德", "吉"),
    ("岁殿", "吉", "游奕", "凶"),
    ("孤辰", "凶", "寡宿", "凶"),
    ("天喜", "吉", "红鸾", "吉"),
    ("勾神", "凶", "绞神", "凶"),
    ("空亡", "凶", "擎天", "凶"),
    ("天喜", "吉", "红鸾", "吉"),
];

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

    let (s1, q1, s2, q2) = YEAR_BRANCH_SPIRITS[branch];
    list.push(ShenSha { name: format!("{} {}", s1, BRANCH_NAMES[branch]), category: "年支".into(), quality: q1.into() });
    list.push(ShenSha { name: format!("{} {}", s2, BRANCH_NAMES[branch]), category: "年支".into(), quality: q2.into() });

    // 年干神煞
    list.push(ShenSha { name: format!("禄神 {}", stems[stem]), category: "年干".into(), quality: "吉".into() });
    // 天乙贵人 (简化: 甲戊→牛羊, 乙己→鼠猴, ...)
    let tian_yi = match stem { 0 | 4 => "丑未", 1 | 5 => "子申", 2 | 6 => "酉亥", 3 | 7 => "卯巳", 8 | 9 => "午寅", _ => "" };
    list.push(ShenSha { name: format!("天乙贵人 {}", tian_yi), category: "年干".into(), quality: "吉".into() });

    list
}

const TIAN_DE_BY_DAY: [&str; 10] = ["寅", "酉", "巳", "子", "申", "申", "寅", "巳", "巳", "巳"];
const YUE_DE_BY_DAY: [&str; 10] = ["丙", "申", "壬", "甲", "午", "壬", "丙", "甲", "巳", "壬"];
const JIN_YU_BY_DAY: [&str; 10] = ["辰", "巳", "未", "申", "丑", "寅", "卯", "亥", "午", "未"];
const HONG_YAN_BY_DAY: [&str; 10] = ["午", "申", "寅", "亥", "未", "辰", "戌", "酉", "子", "巳"];

const SHEN_SHA_NAMES: [&str; 4] = ["天德", "月德", "金舆", "红艳"];

pub fn calculate_shensha_by_day_stem(day_stem_index: usize) -> Vec<ShenSha> {
    let idx = day_stem_index % 10;
    let values = [
        TIAN_DE_BY_DAY[idx],
        YUE_DE_BY_DAY[idx],
        JIN_YU_BY_DAY[idx],
        HONG_YAN_BY_DAY[idx],
    ];
    let mut result: Vec<ShenSha> = (0..4)
        .map(|i| ShenSha {
            name: format!("{} {}", SHEN_SHA_NAMES[i], values[i]),
            category: "日干".into(),
            quality: "吉".into(),
        })
        .collect();

    let has_wenchang = matches!(idx, 0 | 2 | 6 | 7 | 8 | 9);
    if has_wenchang {
        result.push(ShenSha {
            name: "文昌".into(),
            category: "日干".into(),
            quality: "吉".into(),
        });
    }
    result.push(ShenSha {
        name: "天赦".into(),
        category: "日干".into(),
        quality: "吉".into(),
    });

    result
}

const HOUR_SHENSHA: [&str; 12] = [
    "青龙", "明堂", "天刑", "朱雀", "金匮", "天德",
    "青龙", "明堂", "天刑", "朱雀", "金匮", "天德",
];

pub fn calculate_shensha_by_hour_branch(hour_branch_index: usize) -> Vec<ShenSha> {
    vec![ShenSha {
        name: format!("{}", HOUR_SHENSHA[hour_branch_index % 12]),
        category: "时辰".into(),
        quality: "吉".into(),
    }]
}

pub fn calculate_all_shensha(year: i32, month: u8, day_stem_index: usize, hour_branch_index: usize) -> Vec<ShenSha> {
    let _ = month;
    let mut result = calculate_shen_sha(year);
    result.extend(calculate_shensha_by_day_stem(day_stem_index));
    result.extend(calculate_shensha_by_hour_branch(hour_branch_index));
    let mut seen_names = Vec::new();
    result.retain(|s| {
        if seen_names.contains(&s.name) {
            false
        } else {
            seen_names.push(s.name.clone());
            true
        }
    });
    result
}

/// 计算 Ascendant（上升点），Equal House 系统
pub fn calculate_houses(epoch: &Epoch, latitude: f64, longitude: f64, obliquity_rad: f64) -> Vec<HouseData> {
    let lst = local_sidereal_time(epoch, longitude);
    let asc_deg = angles::calculate_ascendant(lst, latitude, obliquity_rad);

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

pub fn calculate_dayun_activation_age(
    birth_utc_jd: f64,
    local_year: i32,
    is_male: bool,
    ctx: &Almanac,
) -> f64 {
    let year_stem_index = ((local_year - 4) % 10 + 10) as usize % 10;
    let is_yang = year_stem_index % 2 == 0;
    let forward = (is_yang && is_male) || (!is_yang && !is_male);
    let jie_indices: [usize; 12] = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];

    let search_forward = |terms: &[solar_terms::SolarTerm]| -> Option<f64> {
        for &idx in &jie_indices {
            if let Some(term) = terms.get(idx) {
                if term.julian_day > birth_utc_jd {
                    return Some((term.julian_day - birth_utc_jd) / 3.0);
                }
            }
        }
        None
    };

    let search_backward = |terms: &[solar_terms::SolarTerm]| -> Option<f64> {
        for &idx in jie_indices.iter().rev() {
            if let Some(term) = terms.get(idx) {
                if term.julian_day < birth_utc_jd {
                    return Some((birth_utc_jd - term.julian_day) / 3.0);
                }
            }
        }
        None
    };

    let this_year = match solar_terms::calculate_solar_terms(local_year, ctx) {
        Ok(t) => t,
        Err(_) => return 0.0,
    };

    if forward {
        if let Some(age) = search_forward(&this_year) {
            return age;
        }
        if let Ok(next_year) = solar_terms::calculate_solar_terms(local_year + 1, ctx) {
            if let Some(age) = search_forward(&next_year) {
                return age;
            }
        }
    } else {
        if let Some(age) = search_backward(&this_year) {
            return age;
        }
        if let Ok(prev_year) = solar_terms::calculate_solar_terms(local_year - 1, ctx) {
            if let Some(age) = search_backward(&prev_year) {
                return age;
            }
        }
    }

    0.0
}

pub fn calculate_chart(
    ctx: &Almanac,
    epoch: Epoch,
    latitude: f64,
    longitude: f64,
    timezone: f64,
    is_male: bool,
) -> AstrologyData {
    let obliquity_rad = calculate_obliquity(&epoch);

    // 本地时间拆分为年月日时（用于八字、福点等）
    let jd = epoch.to_jde_utc_days();
    // 本地时间的儒略日 ≈ UTC JDN + timezone / 24
    let local_jd = jd + timezone as f64 / 24.0;
    // 简化为公历日期
    let z = (local_jd + 0.5) as i64;
    let a = ((z as f64 - 1867216.25) / 36524.25) as i64;
    let a2 = z + 1 + a - (a / 4);
    let b = a2 + 1524;
    let c = ((b as f64 - 122.1) / 365.25) as i64;
    let d = (365.25 * c as f64) as i64;
    let e = ((b - d) as f64 / 30.6001) as i64;
    let day = (b - d - (30.6001 * e as f64) as i64) as u8;
    let month = if e < 14 { (e - 1) as u8 } else { (e - 13) as u8 };
    let year = if month > 2 { c as i32 } else { (c - 1) as i32 };
    // 本地时间的小时
    let hour = ((local_jd - (local_jd.floor())) * 24.0) as u8;
    let body_frames: [(Frame, &str); 10] = [
        (SUN_J2000, "太阳"),
        (MOON_J2000, "太阴"),
        (MERCURY_J2000, "水星"),
        (VENUS_J2000, "金星"),
        (MARS_BARYCENTER_J2000, "火星"),
        (JUPITER_BARYCENTER_J2000, "木星"),
        (SATURN_BARYCENTER_J2000, "土星"),
        (URANUS_BARYCENTER_J2000, "天王星"),
        (NEPTUNE_BARYCENTER_J2000, "海王星"),
        (PLUTO_BARYCENTER_J2000, "冥王星"),
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

    let lst = local_sidereal_time(&epoch, longitude);
    let sun_lon_opt = bodies.iter().find(|b| b.name == "太阳").map(|b| b.longitude);
    let moon_lon_opt = bodies.iter().find(|b| b.name == "太阴").map(|b| b.longitude);

    let (ascendant, midheaven, part_of_fortune, descendant, imum_coeli) = match (sun_lon_opt, moon_lon_opt) {
        (Some(sun_lon), Some(moon_lon)) => {
            let angles = angles::AngleData::new(lst, latitude, obliquity_rad, sun_lon, moon_lon);
            (angles.ascendant, angles.midheaven, angles.part_of_fortune, (angles.ascendant + 180.0) % 360.0, (angles.midheaven + 180.0) % 360.0)
        }
        _ => (0.0, 0.0, 0.0, 0.0, 0.0),
    };

    let birth_utc_jd = epoch.to_jde_utc_days();
    let activation_age = calculate_dayun_activation_age(birth_utc_jd, year, is_male, ctx);
    let bazi = bazi::calculate_bazi(year, month, day, hour, is_male, activation_age);
    let day_stem_index = bazi.day_pillar.stem_index as usize;
    let hour_branch_index = bazi.hour_pillar.branch_index as usize;
    let shen_sha = calculate_all_shensha(year, month as u8, day_stem_index, hour_branch_index);

    let shiganhuayao = calculate_shiganhuayao(bazi.day_pillar.stem_index as usize);
    let asc_branch = ((ascendant / 30.0) as usize) % 12;
    let hour_branch = bazi.hour_pillar.branch_index as usize;
    let (asc_mansion_name, _, _, _) = get_lunar_mansion(ascendant);
    let (ming_gong_zhu, ming_du_zhu, shen_gong_zhu) = calculate_liming_anshen(asc_branch, hour_branch, asc_mansion_name);
    let xijige = calculate_xijige(bazi.day_pillar.stem_index as usize, &bodies, &extra_bodies);

    let xiaoxian_result = calculate_xiaoxian(asc_branch, 0);
    let yuexian_result = calculate_yuexian(xiaoxian_result.1, month);
    let dongweifeixian_result = calculate_dongweifeixian(asc_branch, 0, "阳男阴女");

    // Star power computation
    let mut body_infos: Vec<rule_engine::BodyInfo> = Vec::new();
    for b in &bodies {
        let sign_index = ((b.longitude / 30.0) as usize) % 12;
        body_infos.push(rule_engine::BodyInfo {
            body_id: rule_engine::BodyId::from_name(&b.name).unwrap_or(rule_engine::BodyId::太阳),
            longitude: b.longitude,
            mansion_name: b.mansion_name.clone(),
            sign_zodiac: format!("{}", sign_index),
        });
    }
    for b in &extra_bodies {
        let sign_index = ((b.longitude / 30.0) as usize) % 12;
        body_infos.push(rule_engine::BodyInfo {
            body_id: rule_engine::BodyId::from_name(&b.name).unwrap_or(rule_engine::BodyId::计都),
            longitude: b.longitude,
            mansion_name: b.mansion_name.clone(),
            sign_zodiac: format!("{}", sign_index),
        });
    }
    let body_ming_du = rule_engine::BodyId::from_name(&ming_du_zhu).unwrap_or(rule_engine::BodyId::木星);
    let body_ming_gong = rule_engine::BodyId::from_name(&ming_gong_zhu).unwrap_or(rule_engine::BodyId::火星);
    let star_powers: Vec<rule_engine::StarPower> = body_infos.iter().map(|bi| {
        let sign_idx: usize = bi.sign_zodiac.parse().unwrap_or(0);
        rule_engine::compute_star_power(
            bi.body_id, bi.longitude, &bi.mansion_name, sign_idx, month,
            &body_infos, body_ming_du, body_ming_gong,
        )
    }).collect();
    let house_analyses: Vec<rule_engine::HouseAnalysis> = houses.iter().enumerate().map(|(i, h)| {
        rule_engine::analyze_house(
            i, h.longitude, &body_infos, year,
            day_stem_index, body_ming_du, body_ming_gong,
        )
    }).collect();

    AstrologyData {
        timestamp: epoch.to_string(),
        bodies,
        extra_bodies,
        aspects,
        houses,
        shen_sha,
        ascendant,
        midheaven,
        descendant,
        imum_coeli,
        part_of_fortune,
        bazi,
        shiganhuayao,
        ming_zhu: ming_gong_zhu.clone(),
        shen_zhu: shen_gong_zhu.clone(),
        ming_du_zhu,
        ming_gong_zhu,
        shen_gong_zhu,
        xijige,
        xiaoxian_result,
        yuexian_result,
        dongweifeixian_result,
        zodiac_type: "回归".to_string(),
        ayanamsa: calculate_precession_offset(&epoch),
        dst_applied: false,
        coordinate_system: "ecliptic".to_string(),
        star_powers: Some(star_powers),
        house_analyses: Some(house_analyses),
    }
}

pub fn calculate_shiganhuayao(_day_stem_index: usize) -> Vec<(String, String)> {
    const STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const PAIR_ELEMENTS: [&str; 5] = ["土", "金", "水", "木", "火"];
    (0..10)
        .map(|i| (STEMS[i].to_string(), PAIR_ELEMENTS[i % 5].to_string()))
        .collect()
}

pub fn get_ming_du_zhu(mansion_name: &str) -> String {
    for (i, &(name, _, _)) in LUNAR_MANSIONS.iter().enumerate() {
        if name == mansion_name {
            return MANSION_DEGREE_MASTERS[i].to_string();
        }
    }
    "木星".to_string()
}

pub fn get_sign_master(sign_index: usize) -> String {
    SIGN_HOUSE_MASTERS[sign_index % 12].to_string()
}

/// 安身立命: 计算命宫主、命度主、身宫主及命身歧
pub fn calculate_liming_anshen(
    asc_branch_index: usize,
    hour_branch_index: usize,
    asc_mansion_name: &str,
) -> (String, String, String) {
    let ming_gong_zhu = SIGN_HOUSE_MASTERS[asc_branch_index % 12].to_string();
    let ming_du_zhu = get_ming_du_zhu(asc_mansion_name);
    let shen_gong_zhu = SIGN_HOUSE_MASTERS[hour_branch_index % 12].to_string();
    (ming_gong_zhu, ming_du_zhu, shen_gong_zhu)
}

pub fn calculate_xijige(
    day_stem_index: usize,
    bodies: &[CelestialBodyData],
    extra_bodies: &[ExtraBodyData],
) -> Vec<(String, String)> {
    const STEM_ELEMENT: [usize; 10] = [0, 0, 1, 1, 2, 2, 3, 3, 4, 4];

    fn body_elem(name: &str) -> Option<usize> {
        match name {
            "太阳" | "火星" | "冥王星" | "罗睺" => Some(1),
            "太阴" | "水星" | "海王星" | "月孛" => Some(4),
            "金星" | "天王星" => Some(3),
            "木星" | "紫炁" => Some(0),
            "土星" | "计都" => Some(2),
            _ => None,
        }
    }

    fn relation(day: usize, body: usize) -> &'static str {
        if day == body {
            "平"
        } else if (body + 1) % 5 == day {
            "喜"
        } else if (day + 1) % 5 == body {
            "忌"
        } else if (body + 2) % 5 == day {
            "忌"
        } else {
            "平"
        }
    }

    let day_elem = STEM_ELEMENT[day_stem_index % 10];
    let mut result = Vec::new();

    for body in bodies {
        if let Some(e) = body_elem(&body.name) {
            result.push((body.name.clone(), relation(day_elem, e).to_string()));
        }
    }
    for ext in extra_bodies {
        if let Some(e) = body_elem(&ext.name) {
            result.push((ext.name.clone(), relation(day_elem, e).to_string()));
        }
    }

    result
}

pub fn calculate_xiaoxian(asc_branch_index: usize, current_age: u32) -> (String, usize) {
    let branch_index = (asc_branch_index + current_age as usize) % 12;
    (BRANCH_NAMES[branch_index].to_string(), branch_index)
}

pub fn calculate_yuexian(xiaoxian_branch_index: usize, current_month: u8) -> (String, usize) {
    let branch_index = (xiaoxian_branch_index + current_month as usize - 1) % 12;
    (BRANCH_NAMES[branch_index].to_string(), branch_index)
}

const XIAN_NAMES: [&str; 12] = [
    "命宫", "父母", "福德", "田宅", "官禄", "交友",
    "迁移", "疾厄", "财帛", "子女", "夫妻", "兄弟",
];

pub fn calculate_dongweifeixian(asc_branch_index: usize, age: u32, life_direction: &str) -> Vec<(u32, String, String)> {
    let forward = life_direction == "阳男阴女";
    let mut result = Vec::with_capacity(12);
    for i in 0..12 {
        let period_start_age = age + (i as u32 * 16) / 3;
        let branch_index = if forward {
            (asc_branch_index + i) % 12
        } else {
            (asc_branch_index + 12 - i) % 12
        };
        result.push((
            period_start_age,
            XIAN_NAMES[i].to_string(),
            BRANCH_NAMES[branch_index].to_string(),
        ));
    }
    result
}

pub fn reverse_bazi_time(year: i32, month: u8, day: u8, target_day_stem_index: usize) -> Vec<(u8, String)> {
    let day_pillar = bazi::calculate_day_pillar(year, month, day);
    if day_pillar.stem_index as usize != target_day_stem_index {
        return vec![];
    }
    const BRANCH_START_HOURS: [u8; 12] = [23, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21];
    let mut result = Vec::with_capacity(12);
    for branch_idx in 0..12 {
        let start_hour = BRANCH_START_HOURS[branch_idx];
        let stem = if branch_idx == 0 {
            (target_day_stem_index + 1) % 10
        } else {
            target_day_stem_index
        };
        let _ = bazi::calculate_hour_pillar(stem, start_hour);
        result.push((start_hour, bazi::EARTHLY_BRANCHES[branch_idx].to_string()));
    }
    result
}

/// Search for solar and lunar eclipses in the given year range.
/// Uses geometric approach: checks sun-moon alignment and moon's ecliptic latitude.
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
        let obliquity = calculate_obliquity(&current);
        let sun_state = get_body_state(ctx, SUN_J2000, EARTH_J2000, current);
        let moon_state = get_body_state(ctx, MOON_J2000, EARTH_J2000, current);

        if let (Ok(sun), Ok(moon)) = (sun_state, moon_state) {
            let (sun_lon, _) = cartesian_to_ecliptic(sun.radius_km, obliquity);
            let (moon_lon, moon_lat) = cartesian_to_ecliptic(moon.radius_km, obliquity);

            let raw_diff = (sun_lon - moon_lon + 360.0) % 360.0;
            let new_moon_dist = raw_diff.min(360.0 - raw_diff);
            let full_moon_dist = (raw_diff - 180.0).abs();

            if new_moon_dist < max_lon_diff && moon_lat.abs() < max_moon_lat {
                let lat_factor = 1.0 - (moon_lat.abs() / max_moon_lat).powi(2);
                let lon_factor = 1.0 - (new_moon_dist / max_lon_diff).powi(2);
                let magnitude = (lat_factor * 0.7 + lon_factor * 0.3) * 100.0;
                let (y, m, d, _, _, _, _) = current.to_gregorian_utc();
                results.push((format!("{:04}-{:02}-{:02}", y, m, d), "solar".into(), magnitude));
            }

            if full_moon_dist < max_lon_diff && moon_lat.abs() < max_moon_lat {
                let lat_factor = 1.0 - (moon_lat.abs() / max_moon_lat).powi(2);
                let lon_factor = 1.0 - (full_moon_dist / max_lon_diff).powi(2);
                let magnitude = (lat_factor * 0.7 + lon_factor * 0.3) * 100.0;
                let (y, m, d, _, _, _, _) = current.to_gregorian_utc();
                results.push((format!("{:04}-{:02}-{:02}", y, m, d), "lunar".into(), magnitude));
            }
        }

        current += one_day;
    }

    results.dedup_by(|a, b| a.0 == b.0 && a.1 == b.1);
    results
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub is_leap: bool,
    pub year_stem: String,
    pub year_branch: String,
}

#[derive(Debug, Clone)]
pub struct LunarMonthInfo {
    pub month: u8,
    pub is_leap: bool,
    pub first_day_jd: f64,
    pub days: u8,
}

#[derive(Debug, Clone)]
pub struct LunarYearInfo {
    pub year: i32,
    pub months: Vec<LunarMonthInfo>,
}

fn epoch_from_jd(jd: f64) -> Epoch {
    let ref_epoch = Epoch::from_gregorian_tai_at_midnight(2000, 1, 1);
    let ref_jd = ref_epoch.to_jde_utc_days();
    let offset_days = jd - ref_jd;
    let offset_seconds = offset_days * 86400.0;
    if offset_seconds >= 0.0 {
        ref_epoch + hifitime::Duration::from_seconds(offset_seconds)
    } else {
        ref_epoch - hifitime::Duration::from_seconds(-offset_seconds)
    }
}

fn moon_sun_elongation(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let epoch = epoch_from_jd(jd);
    let obl = calculate_obliquity(&epoch);
    let sun_state = get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch)?;
    let moon_state = get_body_state(ctx, MOON_J2000, EARTH_J2000, epoch)?;
    let (sun_lon, _) = cartesian_to_ecliptic(sun_state.radius_km, obl);
    let (moon_lon, _) = cartesian_to_ecliptic(moon_state.radius_km, obl);
    Ok(moon_lon - sun_lon)
}

fn normalized_elongation(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let elong = moon_sun_elongation(jd, ctx)?;
    let norm = ((elong + 180.0) % 360.0 + 360.0) % 360.0;
    Ok(norm - 180.0)
}

pub fn find_new_moon(approx_jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let mut low = approx_jd - 6.0;
    let mut high = approx_jd + 6.0;

    let mut g_low = normalized_elongation(low, ctx)?;
    let mut g_high = normalized_elongation(high, ctx)?;

    let mut safety = 0;
    while g_low > 0.0 && safety < 10 {
        low -= 3.0;
        g_low = normalized_elongation(low, ctx)?;
        safety += 1;
    }
    safety = 0;
    while g_high < 0.0 && safety < 10 {
        high += 3.0;
        g_high = normalized_elongation(high, ctx)?;
        safety += 1;
    }

    while high - low > 1.0 / 1440.0 {
        let mid = (low + high) / 2.0;
        let g_mid = normalized_elongation(mid, ctx)?;
        if g_mid > 0.0 {
            high = mid;
        } else {
            low = mid;
        }
    }

    Ok((low + high) / 2.0)
}

fn find_previous_new_moon(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let elong_raw = moon_sun_elongation(jd, ctx)?;
    let elong_mod = ((elong_raw % 360.0) + 360.0) % 360.0;
    let days_back = elong_mod / 12.19;
    find_new_moon(jd - days_back, ctx)
}

fn find_next_new_moon(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let elong_raw = moon_sun_elongation(jd, ctx)?;
    let elong_mod = ((elong_raw % 360.0) + 360.0) % 360.0;
    let days_ahead = (360.0 - elong_mod) / 12.19;
    find_new_moon(jd + days_ahead, ctx)
}

fn ymd_to_jd(year: i32, month: u8, day: u8) -> f64 {
    let a = (14 - month as i32) / 12;
    let y = (year + 4800 - a) as i64;
    let m = (month as i32 + 12 * a - 3) as i64;
    let jdn = day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;
    jdn as f64
}

fn jd_to_ymd(jd: f64) -> (i32, u8, u8) {
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
    (year, month, day)
}

const HEAVENLY_STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const EARTHLY_BRANCHES: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
const CST_OFFSET: f64 = 8.0 / 24.0;

const ZHONGQI_NAMES: [&str; 12] = [
    "大寒", "雨水", "春分", "谷雨", "小满", "夏至",
    "大暑", "处暑", "秋分", "霜降", "小雪", "冬至",
];
const ZHONGQI_MONTHS: [u8; 12] = [12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

pub fn calculate_lunar_year(year: i32, ctx: &Almanac) -> Result<LunarYearInfo, String> {
    let terms_curr = solar_terms::calculate_solar_terms(year, ctx)?;
    let terms_next = solar_terms::calculate_solar_terms(year + 1, ctx)?;

    let dongzhi = terms_curr.iter().find(|t| t.name == "冬至").ok_or("冬至 not found")?;
    let dongzhi_next = terms_next.iter().find(|t| t.name == "冬至").ok_or("冬至 next not found")?;

    let shuo_11 = find_previous_new_moon(dongzhi.julian_day + 1.0, ctx)?;
    let shuo_11_next = find_previous_new_moon(dongzhi_next.julian_day + 1.0, ctx)?;

    let mut new_moons = vec![shuo_11];
    loop {
        let last = *new_moons.last().unwrap();
        let next = find_next_new_moon(last + 1.0, ctx)?;
        new_moons.push(next);
        if next >= shuo_11_next - 0.5 {
            break;
        }
    }

    let mut prev_moons = vec![];
    let mut current = shuo_11;
    for _ in 0..12 {
        let prev = find_previous_new_moon(current - 1.0, ctx)?;
        prev_moons.push(prev);
        current = prev;
    }
    prev_moons.reverse();

    let all_shuos: Vec<f64> = prev_moons.into_iter()
        .chain(new_moons.into_iter())
        .collect();

    let terms_prev = solar_terms::calculate_solar_terms(year - 1, ctx)?;
    let all_terms = [terms_prev, terms_curr, terms_next].concat();

    let mut months: Vec<LunarMonthInfo> = Vec::new();
    for i in 0..all_shuos.len() - 1 {
        let start = all_shuos[i];
        let end = all_shuos[i + 1];
        let start_cst_jdn = (start + CST_OFFSET + 0.5) as i64;
        let end_cst_jdn = (end + CST_OFFSET + 0.5) as i64;
        let days = (end_cst_jdn - start_cst_jdn) as u8;

        let mut found_month: Option<u8> = None;
        for (zq_idx, zq_name) in ZHONGQI_NAMES.iter().enumerate() {
            for term in &all_terms {
                if term.name == *zq_name && term.julian_day >= start && term.julian_day < end {
                    found_month = Some(ZHONGQI_MONTHS[zq_idx]);
                    break;
                }
            }
            if found_month.is_some() {
                break;
            }
        }

        months.push(LunarMonthInfo {
            month: found_month.unwrap_or(0),
            is_leap: found_month.is_none(),
            first_day_jd: start,
            days,
        });
    }

    let mut prev_month = 0u8;
    for m in &mut months {
        if m.is_leap {
            m.month = prev_month;
        } else {
            prev_month = m.month;
        }
    }

    if let Some(first_m1) = months.iter().position(|m| m.month == 1 && !m.is_leap) {
        months = months[first_m1..].to_vec();
    }

    Ok(LunarYearInfo { year, months })
}

pub fn solar_to_lunar(year: i32, month: u8, day: u8, ctx: &Almanac) -> Result<LunarDate, String> {
    let jd = ymd_to_jd(year, month, day);
    let jdn_cst = (jd + CST_OFFSET + 0.5) as i64;

    let terms_curr = solar_terms::calculate_solar_terms(year, ctx)?;
    let yushui = terms_curr.iter().find(|t| t.name == "雨水").ok_or("雨水 not found")?;
    let month1_start = find_previous_new_moon(yushui.julian_day + 1.0, ctx)?;
    let m1_jdn_cst = (month1_start + CST_OFFSET + 0.5) as i64;

    let lunar_year = if jdn_cst < m1_jdn_cst { year - 1 } else { year };

    let ly = calculate_lunar_year(lunar_year, ctx)?;

    for lm in &ly.months {
        let start_jdn_cst = (lm.first_day_jd + CST_OFFSET + 0.5) as i64;
        let end_jdn_cst = start_jdn_cst + lm.days as i64;
        if jdn_cst >= start_jdn_cst && jdn_cst < end_jdn_cst {
            let day_in_month = (jdn_cst - start_jdn_cst + 1) as u8;
            let stem_idx = ((ly.year - 4) % 10 + 10) as usize % 10;
            let branch_idx = ((ly.year - 4) % 12 + 12) as usize % 12;
            return Ok(LunarDate {
                year: ly.year,
                month: lm.month,
                day: day_in_month,
                is_leap: lm.is_leap,
                year_stem: HEAVENLY_STEMS[stem_idx].to_string(),
                year_branch: EARTHLY_BRANCHES[branch_idx].to_string(),
            });
        }
    }

    Err("Date not found in lunar calendar".into())
}

pub fn lunar_to_solar(
    lunar_year: i32,
    lunar_month: u8,
    lunar_day: u8,
    is_leap: bool,
    ctx: &Almanac,
) -> Result<(i32, u8, u8), String> {
    let ly = calculate_lunar_year(lunar_year, ctx)?;

    for lm in &ly.months {
        if lm.month == lunar_month && lm.is_leap == is_leap {
            let jd = lm.first_day_jd + (lunar_day as f64) - 1.0;
            return Ok(jd_to_ymd(jd + CST_OFFSET));
        }
    }

    Err("Lunar month not found in calendar".into())
}

/// Simple magnetic declination model using polynomial approximation.
/// Based on IGRF/WMM simplified coefficients for 2020-2025 epoch.
/// Returns declination in degrees (positive = east, negative = west).
pub fn magnetic_declination(latitude: f64, longitude: f64, year: f64) -> f64 {
    let lat_rad = latitude.to_radians();
    let lon_rad = longitude.to_radians();
    let declination = 0.0
        + 10.0 * (0.3 * lon_rad).sin()
        + 5.0 * lat_rad.sin() * (lon_rad - 0.5).cos()
        - 0.2 * (year - 2020.0)
        - 2.0 * (2.0 * lat_rad).cos() * (lon_rad + 1.0).sin();
    declination
}

/// Apply magnetic declination correction.
/// true_bearing = magnetic_bearing + declination (when declination is east)
/// If is_magnetic is true, converts magnetic → true; otherwise returns as-is.
pub fn apply_magnetic_declination(bearing_deg: f64, declination_deg: f64, is_magnetic: bool) -> f64 {
    if !is_magnetic {
        return bearing_deg;
    }
    let corrected = bearing_deg + declination_deg;
    ((corrected % 360.0) + 360.0) % 360.0
}

pub fn search_sun_to_mountain(
    target_azimuth: f64,
    start_jd: f64,
    end_jd: f64,
    latitude: f64,
    longitude: f64,
    timezone: f64,
    ctx: &Almanac,
) -> Vec<(f64, f64, f64)> {
    let mut results = Vec::new();
    let one_day = 1.0;
    let mut current_jd = start_jd;

    while current_jd < end_jd {
        let local_noon_jd = current_jd + (12.0 - timezone) / 24.0;
        let epoch = epoch_from_jd(local_noon_jd);

        if let Ok(state) = get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch) {
            let obl = calculate_obliquity(&state.epoch);
            let (sun_lon, sun_lat) = cartesian_to_ecliptic(state.radius_km, obl);
            let lst = local_sidereal_time(&state.epoch, longitude);
            let (az, alt) = get_body_horizontal(sun_lon, sun_lat, lst, latitude, obl);

            let mut diff = (az - target_azimuth).abs();
            if diff > 180.0 {
                diff = 360.0 - diff;
            }

            if diff < 5.0 && alt > 0.0 {
                results.push((current_jd, az, alt));
            }
        }

        current_jd += one_day;
    }

    results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    results.dedup_by(|a, b| (a.0 - b.0).abs() < 2.0);

    results
}

pub fn search_sun_at_date(
    target_azimuth: f64,
    date_jd: f64,
    latitude: f64,
    longitude: f64,
    timezone: f64,
    ctx: &Almanac,
) -> Vec<(f64, f64, f64)> {
    let mut results = Vec::new();
    let start_local = 6.0;
    let end_local = 18.0;
    let step_hours = 10.0 / 60.0;

    let mut local_hour = start_local;
    while local_hour <= end_local {
        let jd = date_jd + (local_hour - timezone) / 24.0;
        let epoch = epoch_from_jd(jd);

        if let Ok(state) = get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch) {
            let obl = calculate_obliquity(&state.epoch);
            let (sun_lon, sun_lat) = cartesian_to_ecliptic(state.radius_km, obl);
            let lst = local_sidereal_time(&state.epoch, longitude);
            let (az, alt) = get_body_horizontal(sun_lon, sun_lat, lst, latitude, obl);

            let mut diff = (az - target_azimuth).abs();
            if diff > 180.0 {
                diff = 360.0 - diff;
            }

            if diff < 5.0 && alt > 0.0 {
                results.push((jd, az, alt));
            }
        }

        local_hour += step_hours;
    }

    results
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
            zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(),
            mansion_name: "角".into(), mansion_degree: 0.0,
        };
        let b2 = CelestialBodyData {
name: "太阴".into(), longitude: 180.0, latitude: 0.0, speed: 13.0,
             zodiac_sign: "天秤".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(),
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

    #[test]
    fn test_shiganhuayao() {
        let result = calculate_shiganhuayao(0);
        assert_eq!(result.len(), 10);
        assert_eq!(result[0], ("甲".to_string(), "土".to_string()));
        assert_eq!(result[5], ("己".to_string(), "土".to_string()));
        assert_eq!(result[1], ("乙".to_string(), "金".to_string()));
        assert_eq!(result[6], ("庚".to_string(), "金".to_string()));
    }

    #[test]
    fn test_liming_anshen() {
        let (ming_gong, ming_du, shen_gong) = calculate_liming_anshen(0, 0, "角");
        assert_eq!(ming_gong, "土星");
        assert_eq!(ming_du, "木星");
        assert_eq!(shen_gong, "土星");
    }

    #[test]
    fn test_xijige() {
        let bodies = vec![
            CelestialBodyData { name: "太阳".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "太阴".into(), longitude: 180.0, latitude: 0.0, speed: 13.0, zodiac_sign: "天秤".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "水星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "金星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "火星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "木星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "土星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: "白羊".to_string(), zodiac_degree: 0.0, coordinate_system: "ecliptic".to_string(), mansion_name: "角".into(), mansion_degree: 0.0 },
        ];
        let extras = vec![
            ExtraBodyData { name: "天王星".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "海王星".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "冥王星".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "罗睺".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "计都".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "月孛".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
            ExtraBodyData { name: "紫炁".into(), longitude: 0.0, mansion_name: "角".into(), mansion_degree: 0.0 },
        ];
        let result = calculate_xijige(0, &bodies, &extras);
        assert_eq!(result.len(), 14);
        assert!(result.iter().any(|(n, _)| n == "太阳"));
        assert!(result.iter().any(|(n, _)| n == "紫炁"));
    }

    #[test]
    fn test_xiaoxian() {
        let (name, idx) = calculate_xiaoxian(0, 0);
        assert_eq!(name, "子");
        assert_eq!(idx, 0);
        let (name, idx) = calculate_xiaoxian(0, 1);
        assert_eq!(name, "丑");
        assert_eq!(idx, 1);
        let (name, idx) = calculate_xiaoxian(2, 10);
        assert_eq!(idx, 0);
        assert_eq!(name, "子");
    }

    #[test]
    fn test_yuexian() {
        let (name, idx) = calculate_yuexian(0, 1);
        assert_eq!(name, "子");
        assert_eq!(idx, 0);
        let (name, idx) = calculate_yuexian(0, 2);
        assert_eq!(name, "丑");
        assert_eq!(idx, 1);
        let (name, idx) = calculate_yuexian(5, 12);
        assert_eq!(idx, 4);
        assert_eq!(name, "辰");
    }

    #[test]
    fn test_shensha_by_day_stem() {
        let result = calculate_shensha_by_day_stem(0);
        assert!(!result.is_empty(), "日干神煞不应为空");
        assert_eq!(result.len(), 6);
        assert!(result.iter().any(|s| s.name.contains("天德")));
        assert!(result.iter().any(|s| s.name.contains("月德")));
        assert!(result.iter().any(|s| s.name.contains("红艳")));
        assert!(result.iter().any(|s| s.name.contains("文昌")));
        assert!(result.iter().any(|s| s.name.contains("天赦")));
        assert_eq!(result[0].category, "日干");
    }

    #[test]
    fn test_shensha_by_day_stem_all() {
        for i in 0..10 {
            let r = calculate_shensha_by_day_stem(i);
            let has_wenchang = matches!(i, 0 | 2 | 6 | 7 | 8 | 9);
            let expected = if has_wenchang { 6 } else { 5 };
            assert_eq!(r.len(), expected, "stem={} 应有{}个神煞", i, expected);
        }
    }

    #[test]
    fn test_shensha_by_hour() {
        let result = calculate_shensha_by_hour_branch(0);
        assert!(!result.is_empty(), "时辰神煞不应为空");
        assert_eq!(result[0].name, "青龙");
        let result = calculate_shensha_by_hour_branch(6);
        assert_eq!(result[0].name, "青龙");
        let result = calculate_shensha_by_hour_branch(3);
        assert_eq!(result[0].name, "朱雀");
    }

    #[test]
    fn test_shensha_by_hour_all() {
        for i in 0..12 {
            let r = calculate_shensha_by_hour_branch(i);
            assert_eq!(r.len(), 1, "每个时辰应有1个神煞, branch={}", i);
            assert_eq!(r[0].category, "时辰");
        }
    }

    #[test]
    fn test_precession_offset() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        let offset = calculate_precession_offset(&epoch);
        assert!(offset > 0.0 && offset < 360.0, "岁差偏移应在合理范围, 实际={}", offset);
        assert!(offset > 20.0 && offset < 30.0, "岁差偏移应在~24°附近, 实际={}", offset);
    }

    #[test]
    fn test_dongweifeixian() {
        let result = calculate_dongweifeixian(0, 0, "阳男阴女");
        assert_eq!(result.len(), 12);
        assert_eq!(result[0].1, "命宫");
        assert_eq!(result[0].2, "子");
        assert_eq!(result[1].1, "父母");
        assert_eq!(result[1].2, "丑");
        let result = calculate_dongweifeixian(0, 20, "阴男阳女");
        assert_eq!(result.len(), 12);
        assert_eq!(result[0].2, "子");
        assert_eq!(result[1].2, "亥");
        assert_eq!(result[2].2, "戌");
    }

    #[test]
    fn test_custom_rules_empty() {
        let rules = CustomRules::default();
        assert!(rules.aspect_orbs.is_none());
        assert!(rules.enabled_shensha.is_none());
        assert!(rules.custom_ziqui_offset.is_none());
        assert!(rules.custom_dayun_start_age.is_none());

        let json = r#"{}"#;
        let rules: CustomRules = serde_json::from_str(json).unwrap();
        assert!(rules.aspect_orbs.is_none());
        assert!(rules.enabled_shensha.is_none());
        assert!(rules.custom_ziqui_offset.is_none());
        assert!(rules.custom_dayun_start_age.is_none());

        let json = r#"{"custom_ziqui_offset": 180.0}"#;
        let rules: CustomRules = serde_json::from_str(json).unwrap();
        assert!(rules.aspect_orbs.is_none());
        assert!(rules.enabled_shensha.is_none());
        assert_eq!(rules.custom_ziqui_offset, Some(180.0));
        assert!(rules.custom_dayun_start_age.is_none());
    }

    #[test]
    fn test_custom_rules_override() {
        let json = r#"{"custom_ziqui_offset": 180.0}"#;
        let rules: CustomRules = serde_json::from_str(json).unwrap();
        assert_eq!(rules.custom_ziqui_offset, Some(180.0));
    }

    #[test]
    fn test_load_custom_rules_nonexistent() {
        let rules = load_custom_rules("/tmp/nonexistent_custom_rules_file.json").unwrap();
        assert!(rules.aspect_orbs.is_none());
        assert!(rules.enabled_shensha.is_none());
        assert!(rules.custom_ziqui_offset.is_none());
        assert!(rules.custom_dayun_start_age.is_none());
    }

    #[test]
    fn test_reverse_bazi() {
        let result = reverse_bazi_time(2000, 1, 1, 4);
        assert_eq!(result.len(), 12);
        assert!(result.contains(&(23, "子".to_string())));
        assert!(result.contains(&(1, "丑".to_string())));
        assert!(result.contains(&(3, "寅".to_string())));
        assert!(result.contains(&(5, "卯".to_string())));
        assert!(result.contains(&(7, "辰".to_string())));
        assert!(result.contains(&(9, "巳".to_string())));
        assert!(result.contains(&(11, "午".to_string())));
        assert!(result.contains(&(13, "未".to_string())));
        assert!(result.contains(&(15, "申".to_string())));
        assert!(result.contains(&(17, "酉".to_string())));
        assert!(result.contains(&(19, "戌".to_string())));
        assert!(result.contains(&(21, "亥".to_string())));
    }

    #[test]
    fn test_reverse_bazi_no_match() {
        let result = reverse_bazi_time(2000, 1, 1, 5);
        assert!(result.is_empty());
    }

    #[test]
    fn test_reverse_bazi_late_zi() {
        let result = reverse_bazi_time(2000, 1, 1, 4);
        assert!(result.contains(&(23, "子".to_string())));
        let late_zi = bazi::calculate_hour_pillar(5, 23);
        assert_eq!(late_zi.heavenly_stem, "甲");
        assert_eq!(late_zi.earthly_branch, "子");
    }

    #[test]
    fn test_eclipse_search() {
        let bsp_path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/bsp/de440.bsp");
        if !std::path::Path::new(bsp_path).exists() {
            eprintln!("Skipping: BSP file not found at {}", bsp_path);
            return;
        }
        let ctx = load_almanac(bsp_path).unwrap();
        let results = search_eclipses(2024, 2024, &ctx);
        assert!(!results.is_empty(), "Should find at least 1 eclipse in 2024");
        assert!(results.iter().any(|(_, t, _)| t == "solar"), "Should find a solar eclipse");
    }

    fn get_lunar_test_almanac() -> Option<Almanac> {
        let paths = [
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/bsp/de440.bsp"),
            "assets/bsp/de440.bsp",
            "../assets/bsp/de440.bsp",
        ];
        for path in &paths {
            if std::path::Path::new(path).exists() {
                if let Ok(ctx) = load_almanac(path) {
                    return Some(ctx);
                }
            }
        }
        None
    }

    #[test]
    fn test_new_moon_2024_jan() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let result = find_new_moon(ymd_to_jd(2024, 1, 10), &ctx).unwrap();
        let (y, m, d) = jd_to_ymd(result);
        assert_eq!((y, m, d), (2024, 1, 11));
    }

    #[test]
    fn test_solar_to_lunar_spring_festival() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let result = solar_to_lunar(2024, 2, 10, &ctx).unwrap();
        assert_eq!((result.year, result.month, result.day, result.is_leap), (2024, 1, 1, false));
    }

    #[test]
    fn test_solar_to_lunar_known() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let result = solar_to_lunar(2024, 12, 25, &ctx).unwrap();
        assert_eq!((result.year, result.month, result.day), (2024, 11, 25));
    }

    #[test]
    fn test_lunar_roundtrip() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let test_dates = [(2024, 2u8, 10u8), (2024, 6u8, 15u8), (2024, 12u8, 25u8)];
        for &(y, m, d) in &test_dates {
            let lunar = solar_to_lunar(y, m, d, &ctx).unwrap();
            let (y2, m2, d2) = lunar_to_solar(lunar.year, lunar.month, lunar.day, lunar.is_leap, &ctx).unwrap();
            assert_eq!((y, m, d), (y2, m2, d2), "Roundtrip failed for {}/{}/{}", y, m, d);
        }
    }

    #[test]
    fn test_lunar_to_solar_known() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let (y, m, d) = lunar_to_solar(2024, 1, 1, false, &ctx).unwrap();
        assert_eq!((y, m, d), (2024, 2, 10));
    }

    #[test]
    fn test_lunar_year_2024_structure() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let ly = calculate_lunar_year(2024, &ctx).unwrap();
        assert!(!ly.months.is_empty());
        assert_eq!(ly.months[0].month, 1);
        assert!(!ly.months[0].is_leap);
        let has_month_12 = ly.months.iter().any(|m| m.month == 12 && !m.is_leap);
        assert!(has_month_12, "Lunar year should have month 12");
    }

    #[test]
    fn test_solar_to_lunar_new_year_eve() {
        let ctx = match get_lunar_test_almanac() {
            Some(c) => c,
            None => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let result = solar_to_lunar(2024, 2, 9, &ctx).unwrap();
        assert_eq!(result.year, 2023);
        assert_eq!(result.day, 30);
        let result = solar_to_lunar(2024, 2, 10, &ctx).unwrap();
        assert_eq!((result.year, result.month, result.day), (2024, 1, 1));
    }

    #[test]
    fn test_ecliptic_to_equatorial_vernal_equinox() {
        let obliquity = TEST_EPS;
        let (ra, dec) = ecliptic_to_equatorial(0.0, 0.0, obliquity);
        assert!((ra - 0.0).abs() < 0.01, "春分点赤经应为0°, 实际={}", ra);
        assert!((dec - 0.0).abs() < 0.01, "春分点赤纬应为0°, 实际={}", dec);
    }

    #[test]
    fn test_ecliptic_to_equatorial_summer_solstice() {
        let obliquity = TEST_EPS;
        let (ra, dec) = ecliptic_to_equatorial(90.0, 0.0, obliquity);
        assert!((ra - 90.0).abs() < 0.01, "夏至点赤经应为90°, 实际={}", ra);
        let eps_deg = obliquity.to_degrees();
        assert!((dec - eps_deg).abs() < 0.01, "夏至点赤纬应≈黄赤交角={}, 实际={}", eps_deg, dec);
    }

    #[test]
    fn test_equatorial_to_horizontal_zenith() {
        let ha = 0.0;
        let dec = 40.0;
        let lat = 40.0;
        let (az, alt) = equatorial_to_horizontal(ha, dec, lat);
        assert!((alt - 90.0).abs() < 0.01, "天顶高度应为90°, 实际={}", alt);
        assert!(az >= 0.0 && az < 360.0, "方位角应在0-360°, 实际={}", az);
    }

    #[test]
    fn test_equatorial_to_horizontal_range() {
        for ha in [0.0, 90.0, 180.0, 270.0] {
            for dec in [-80.0, 0.0, 80.0] {
                for lat in [-90.0, 0.0, 90.0] {
                    let (az, alt) = equatorial_to_horizontal(ha, dec, lat);
                    assert!(az >= 0.0 && az < 360.0, "方位角越界: ha={} dec={} lat={} az={}", ha, dec, lat, az);
                    assert!(alt >= -90.0 && alt <= 90.0, "高度角越界: ha={} dec={} lat={} alt={}", ha, dec, lat, alt);
                }
            }
        }
    }

    #[test]
    fn test_calculate_hour_angle_basic() {
        let ha = calculate_hour_angle(100.0, 50.0);
        assert!((ha - 50.0).abs() < 0.01, "时角应为50°, 实际={}", ha);
    }

    #[test]
    fn test_calculate_hour_angle_wrap() {
        let ha = calculate_hour_angle(30.0, 350.0);
        assert!((ha - 40.0).abs() < 0.01, "时角(30-350)wrap应为40°, 实际={}", ha);
    }

    #[test]
    fn test_get_body_horizontal_range() {
        let (az, alt) = get_body_horizontal(0.0, 0.0, 100.0, 40.0, TEST_EPS);
        assert!(az >= 0.0 && az < 360.0);
        assert!(alt >= -90.0 && alt <= 90.0);
    }

    #[test]
    fn test_astrology_data_coordinate_system() {
        let epoch = Epoch::from_tdb_seconds(0.0);
        if let Ok(ctx) = load_almanac(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/bsp/de440.bsp")) {
            let data = calculate_chart(&ctx, epoch, 40.0, -74.0, -5.0, true);
            assert_eq!(data.coordinate_system, "ecliptic");
        }
    }

    #[test]
    fn test_magnetic_declination_range() {
        let d1 = magnetic_declination(39.9, 116.4, 2024.0);
        assert!(d1 >= -30.0 && d1 <= 30.0, "Beijing declination out of range: {}", d1);
        let d2 = magnetic_declination(-33.9, 151.2, 2024.0);
        assert!(d2 >= -30.0 && d2 <= 30.0);
        let d3 = magnetic_declination(0.0, 0.0, 2024.0);
        assert!(d3 >= -30.0 && d3 <= 30.0);
    }

    #[test]
    fn test_apply_magnetic_declination() {
        assert!((apply_magnetic_declination(0.0, 10.0, true) - 10.0).abs() < 0.01);
        assert!((apply_magnetic_declination(0.0, -10.0, true) - (-10.0 + 360.0) % 360.0).abs() < 0.01);
        assert!((apply_magnetic_declination(0.0, 10.0, false) - 0.0).abs() < 0.01);
    }

    #[test]
    fn test_sun_azimuth_range() {
        let epoch = Epoch::from_gregorian_utc_at_midnight(2024, 6, 21) + hifitime::Duration::from_hours(4.0);
        let ctx = match load_almanac(concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/bsp/de440.bsp")) {
            Ok(c) => c,
            Err(_) => { eprintln!("Skipping: BSP file not found"); return; }
        };
        let state = get_body_state(&ctx, SUN_J2000, EARTH_J2000, epoch).unwrap();
        let obl = calculate_obliquity(&state.epoch);
        let (sun_lon, sun_lat) = cartesian_to_ecliptic(state.radius_km, obl);
        let lst = local_sidereal_time(&state.epoch, 116.4);
        let (az, alt) = get_body_horizontal(sun_lon, sun_lat, lst, 39.9, obl);
        assert!(az >= 0.0 && az < 360.0);
        assert!(alt >= -90.0 && alt <= 90.0);
        assert!(alt > 0.0, "Sun should be above horizon at noon in Beijing");
    }
}
