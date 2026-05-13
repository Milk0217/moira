pub mod angles;
pub mod bazi;

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
    pub quality: String,
}

#[derive(Debug, Serialize)]
pub struct AstrologyData {
    pub timestamp: String,
    pub bodies: Vec<CelestialBodyData>,
    pub extra_bodies: Vec<ExtraBodyData>,
    pub aspects: Vec<Aspect>,
    pub houses: Vec<HouseData>,
    pub shen_sha: Vec<ShenSha>,
    pub ascendant: f64,
    pub midheaven: f64,
    pub part_of_fortune: f64,
    pub bazi: bazi::BaziData,
    pub shiganhuayao: Vec<(String, String)>,
    pub ming_zhu: String,
    pub shen_zhu: String,
    pub xijige: Vec<(String, String)>,
    pub xiaoxian_result: (String, usize),
    pub yuexian_result: (String, usize),
    pub dongweifeixian_result: Vec<(u32, String, String)>,
    pub zodiac_type: String,
    pub ayanamsa: f64,
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
            zodiac_sign: ("白羊".to_string(), 0.0),
            mansion_name: "角".to_string(),
            mansion_degree: 0.0,
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
        body.zodiac_sign = (ZODIAC_SIGNS[sign_index].to_string(), degree_in_sign);
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
    (0..4)
        .map(|i| ShenSha {
            name: format!("{} {}", SHEN_SHA_NAMES[i], values[i]),
            category: "日干".into(),
            quality: "吉".into(),
        })
        .collect()
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

pub fn calculate_chart(
    ctx: &Almanac,
    epoch: Epoch,
    latitude: f64,
    longitude: f64,
    timezone: f64,
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

    let (ascendant, midheaven, part_of_fortune) = match (sun_lon_opt, moon_lon_opt) {
        (Some(sun_lon), Some(moon_lon)) => {
            let angles = angles::AngleData::new(lst, latitude, obliquity_rad, sun_lon, moon_lon);
            (angles.ascendant, angles.midheaven, angles.part_of_fortune)
        }
        _ => (0.0, 0.0, 0.0),
    };

    let bazi = bazi::calculate_bazi(year, month, day, hour);
    let day_stem_index = bazi.day_pillar.stem_index as usize;
    let hour_branch_index = bazi.hour_pillar.branch_index as usize;
    let shen_sha = calculate_all_shensha(year, month as u8, day_stem_index, hour_branch_index);

    let shiganhuayao = calculate_shiganhuayao(bazi.day_pillar.stem_index as usize);
    let asc_branch = ((ascendant / 30.0) as usize) % 12;
    let hour_branch = bazi.hour_pillar.branch_index as usize;
    let (ming_zhu, shen_zhu) = calculate_liming_anshen(asc_branch, hour_branch);
    let xijige = calculate_xijige(bazi.day_pillar.stem_index as usize, &bodies, &extra_bodies);

    let xiaoxian_result = calculate_xiaoxian(asc_branch, 0);
    let yuexian_result = calculate_yuexian(xiaoxian_result.1, month);
    let dongweifeixian_result = calculate_dongweifeixian(asc_branch, 0, "阳男阴女");

    AstrologyData {
        timestamp: epoch.to_string(),
        bodies,
        extra_bodies,
        aspects,
        houses,
        shen_sha,
        ascendant,
        midheaven,
        part_of_fortune,
        bazi,
        shiganhuayao,
        ming_zhu,
        shen_zhu,
        xijige,
        xiaoxian_result,
        yuexian_result,
        dongweifeixian_result,
        zodiac_type: "回归".to_string(),
        ayanamsa: calculate_precession_offset(&epoch),
    }
}

pub fn calculate_shiganhuayao(_day_stem_index: usize) -> Vec<(String, String)> {
    const STEMS: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
    const PAIR_ELEMENTS: [&str; 5] = ["土", "金", "水", "木", "火"];
    (0..10)
        .map(|i| (STEMS[i].to_string(), PAIR_ELEMENTS[i % 5].to_string()))
        .collect()
}

pub fn calculate_liming_anshen(asc_branch_index: usize, hour_branch_index: usize) -> (String, String) {
    const MING_ZHU: [&str; 12] = [
        "贪狼", "巨门", "禄存", "文曲", "廉贞", "武曲",
        "破军", "武曲", "廉贞", "文曲", "禄存", "巨门",
    ];
    let ming_zhu = MING_ZHU[asc_branch_index % 12];
    let shen_zhu = MING_ZHU[(hour_branch_index + 6) % 12];
    (ming_zhu.to_string(), shen_zhu.to_string())
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
        let (ming, shen) = calculate_liming_anshen(0, 0);
        assert_eq!(ming, "贪狼");
        assert_eq!(shen, "破军");
    }

    #[test]
    fn test_xijige() {
        let bodies = vec![
            CelestialBodyData { name: "太阳".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "太阴".into(), longitude: 180.0, latitude: 0.0, speed: 13.0, zodiac_sign: ("天秤".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "水星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "金星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "火星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "木星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
            CelestialBodyData { name: "土星".into(), longitude: 0.0, latitude: 0.0, speed: 1.0, zodiac_sign: ("白羊".into(), 0.0), mansion_name: "角".into(), mansion_degree: 0.0 },
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
        assert_eq!(result.len(), 4);
        assert!(result.iter().any(|s| s.name.contains("天德")));
        assert!(result.iter().any(|s| s.name.contains("月德")));
        assert!(result.iter().any(|s| s.name.contains("红艳")));
        assert_eq!(result[0].category, "日干");
    }

    #[test]
    fn test_shensha_by_day_stem_all() {
        for i in 0..10 {
            let r = calculate_shensha_by_day_stem(i);
            assert_eq!(r.len(), 4, "每个天干应有4个神煞, stem={}", i);
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
}
