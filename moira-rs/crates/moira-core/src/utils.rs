use anise::prelude::Epoch;

/// 儒略日 → Epoch (UTC)
/// hifitime 的 `from_utc_days` 接受的是 UTC 日起而非 JDE，
/// 因此保留偏移公式：相对 J2000 偏移秒数
pub fn epoch_from_jd(jd: f64) -> Epoch {
    let ref_epoch = Epoch::from_gregorian_tai_at_midnight(2000, 1, 1);
    let ref_jd = ref_epoch.to_jde_utc_days();
    let offset_seconds = (jd - ref_jd) * 86400.0;
    if offset_seconds >= 0.0 {
        ref_epoch + hifitime::Duration::from_seconds(offset_seconds)
    } else {
        ref_epoch - hifitime::Duration::from_seconds(-offset_seconds)
    }
}

/// 儒略日 → 公历 (year, month, day, hour, minute)
pub fn jd_to_gregorian(jd: f64) -> (i32, u8, u8, u8, u8) {
    let epoch = epoch_from_jd(jd);
    let (y, m, d, h, min, _s, _ns) = epoch.to_gregorian_utc();
    (y, m, d, h, min)
}

/// 公历 → 儒略日 (UT 午夜)
pub fn ymd_to_jd(year: i32, month: u8, day: u8) -> f64 {
    Epoch::from_gregorian_utc_at_midnight(year, month, day).to_jde_utc_days()
}
