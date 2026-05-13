use anise::prelude::Epoch;
use log::{info, LevelFilter};
use moira_core::*;
use std::str::FromStr;

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();

    let ctx = load_almanac("assets/bsp/de440.bsp").expect("加载星历失败");
    let time = "2020-10-15T12:34:56.789Z";
    let epoch = Epoch::from_str(time).unwrap();
    let data = calculate_chart(&ctx, epoch, 39.9042, 116.4074); // 北京

    info!("[七政]");
    for b in &data.bodies {
        info!(
            "  {}: 黄经 {:.2}° 黄纬 {:.2}° {} {:.2}°",
            b.name, b.longitude, b.latitude, b.zodiac_sign.0, b.zodiac_sign.1
        );
    }

    info!("[四馀]");
    for e in &data.extra_bodies {
        info!("  {}: 黄经 {:.2}°", e.name, e.longitude);
    }

    info!("[相位]");
    for a in &data.aspects {
        info!("  {} - {}: {} ({:.1}°)", a.point1, a.point2, a.aspect_type, a.angle);
    }
}
