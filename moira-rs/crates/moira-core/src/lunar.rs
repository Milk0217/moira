use anise::constants::frames::{EARTH_J2000, MOON_J2000, SUN_J2000};
use anise::prelude::{Almanac, Epoch};
use serde::{Deserialize, Serialize};

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

const CST_OFFSET: f64 = 8.0 / 24.0;

const ZHONGQI_NAMES: [&str; 12] = [
    "大寒", "雨水", "春分", "谷雨", "小满", "夏至",
    "大暑", "处暑", "秋分", "霜降", "小雪", "冬至",
];
const ZHONGQI_MONTHS: [u8; 12] = [12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

fn epoch_from_jd(jd: f64) -> Epoch {
    crate::utils::epoch_from_jd(jd)
}

fn moon_sun_elongation(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let epoch = epoch_from_jd(jd);
    let obl = crate::calculate_obliquity(&epoch);
    let sun_state = crate::get_body_state(ctx, SUN_J2000, EARTH_J2000, epoch)?;
    let moon_state = crate::get_body_state(ctx, MOON_J2000, EARTH_J2000, epoch)?;
    let (sun_lon, _) = crate::cartesian_to_ecliptic(sun_state.radius_km, obl);
    let (moon_lon, _) = crate::cartesian_to_ecliptic(moon_state.radius_km, obl);
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
    while g_low > 0.0 && safety < 10 { low -= 3.0; g_low = normalized_elongation(low, ctx)?; safety += 1; }
    safety = 0;
    while g_high < 0.0 && safety < 10 { high += 3.0; g_high = normalized_elongation(high, ctx)?; safety += 1; }

    while high - low > 1.0 / 1440.0 {
        let mid = (low + high) / 2.0;
        let g_mid = normalized_elongation(mid, ctx)?;
        if g_mid > 0.0 { high = mid; } else { low = mid; }
    }
    Ok((low + high) / 2.0)
}

fn find_previous_new_moon(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let elong_raw = moon_sun_elongation(jd, ctx)?;
    let elong_mod = ((elong_raw % 360.0) + 360.0) % 360.0;
    find_new_moon(jd - elong_mod / 12.19, ctx)
}

fn find_next_new_moon(jd: f64, ctx: &Almanac) -> Result<f64, String> {
    let elong_raw = moon_sun_elongation(jd, ctx)?;
    let elong_mod = ((elong_raw % 360.0) + 360.0) % 360.0;
    find_new_moon(jd + (360.0 - elong_mod) / 12.19, ctx)
}

pub fn ymd_to_jd(year: i32, month: u8, day: u8) -> f64 {
    crate::utils::ymd_to_jd(year, month, day)
}

pub fn jd_to_ymd(jd: f64) -> (i32, u8, u8) {
    let (y, m, d, _h, _min) = crate::utils::jd_to_gregorian(jd);
    (y, m, d)
}

pub fn calculate_lunar_year(year: i32, ctx: &Almanac) -> Result<LunarYearInfo, String> {
    let terms_curr = crate::solar_terms::calculate_solar_terms(year, ctx)?;
    let terms_next = crate::solar_terms::calculate_solar_terms(year + 1, ctx)?;
    let dongzhi = terms_curr.iter().find(|t| t.name == "冬至").ok_or("冬至 not found")?;
    let dongzhi_next = terms_next.iter().find(|t| t.name == "冬至").ok_or("冬至 next not found")?;

    let shuo_11 = find_previous_new_moon(dongzhi.julian_day + 1.0, ctx)?;
    let shuo_11_next = find_previous_new_moon(dongzhi_next.julian_day + 1.0, ctx)?;

    let mut new_moons = vec![shuo_11];
    loop {
        let last = *new_moons.last().unwrap();
        let next = find_next_new_moon(last + 1.0, ctx)?;
        new_moons.push(next);
        if next >= shuo_11_next - 0.5 { break; }
    }

    let mut prev_moons = vec![];
    let mut current = shuo_11;
    for _ in 0..12 { let prev = find_previous_new_moon(current - 1.0, ctx)?; prev_moons.push(prev); current = prev; }
    prev_moons.reverse();

    let all_shuos: Vec<f64> = prev_moons.into_iter().chain(new_moons.into_iter()).collect();
    let terms_prev = crate::solar_terms::calculate_solar_terms(year - 1, ctx)?;
    let all_terms = [terms_prev, terms_curr, terms_next].concat();

    let mut months: Vec<LunarMonthInfo> = Vec::new();
    for i in 0..all_shuos.len() - 1 {
        let start = all_shuos[i];
        let end = all_shuos[i + 1];
        let days = ((end + CST_OFFSET + 0.5) as i64 - (start + CST_OFFSET + 0.5) as i64) as u8;

        let mut found_month: Option<u8> = None;
        for (zq_idx, zq_name) in ZHONGQI_NAMES.iter().enumerate() {
            for term in &all_terms {
                if term.name == *zq_name && term.julian_day >= start && term.julian_day < end {
                    found_month = Some(ZHONGQI_MONTHS[zq_idx]); break;
                }
            }
            if found_month.is_some() { break; }
        }
        months.push(LunarMonthInfo { month: found_month.unwrap_or(0), is_leap: found_month.is_none(), first_day_jd: start, days });
    }

    let mut prev_month = 0u8;
    for m in &mut months {
        if m.is_leap { m.month = prev_month; } else { prev_month = m.month; }
    }
    if let Some(first_m1) = months.iter().position(|m| m.month == 1 && !m.is_leap) { months = months[first_m1..].to_vec(); }

    Ok(LunarYearInfo { year, months })
}

pub fn solar_to_lunar(year: i32, month: u8, day: u8, ctx: &Almanac) -> Result<LunarDate, String> {
    let jd = ymd_to_jd(year, month, day);
    let jdn_cst = (jd + CST_OFFSET + 0.5) as i64;

    let terms_curr = crate::solar_terms::calculate_solar_terms(year, ctx)?;
    let yushui = terms_curr.iter().find(|t| t.name == "雨水").ok_or("雨水 not found")?;
    let month1_start = find_previous_new_moon(yushui.julian_day + 1.0, ctx)?;
    let lunar_year = if jdn_cst < (month1_start + CST_OFFSET + 0.5) as i64 { year - 1 } else { year };

    let ly = calculate_lunar_year(lunar_year, ctx)?;
    for lm in &ly.months {
        let start_jdn_cst = (lm.first_day_jd + CST_OFFSET + 0.5) as i64;
        let end_jdn_cst = start_jdn_cst + lm.days as i64;
        if jdn_cst >= start_jdn_cst && jdn_cst < end_jdn_cst {
            let day_in_month = (jdn_cst - start_jdn_cst + 1) as u8;
            let stem_idx = ((ly.year - 4) % 10 + 10) as usize % 10;
            let branch_idx = ((ly.year - 4) % 12 + 12) as usize % 12;
            return Ok(LunarDate {
                year: ly.year, month: lm.month, day: day_in_month, is_leap: lm.is_leap,
                year_stem: crate::bazi::HEAVENLY_STEMS[stem_idx].to_string(),
                year_branch: crate::bazi::EARTHLY_BRANCHES[branch_idx].to_string(),
            });
        }
    }
    Err("Date not found in lunar calendar".into())
}

pub fn lunar_to_solar(lunar_year: i32, lunar_month: u8, lunar_day: u8, is_leap: bool, ctx: &Almanac) -> Result<(i32, u8, u8), String> {
    let ly = calculate_lunar_year(lunar_year, ctx)?;
    for lm in &ly.months {
        if lm.month == lunar_month && lm.is_leap == is_leap {
            return Ok(jd_to_ymd(lm.first_day_jd + (lunar_day as f64) - 1.0 + CST_OFFSET));
        }
    }
    Err("Lunar month not found in calendar".into())
}
