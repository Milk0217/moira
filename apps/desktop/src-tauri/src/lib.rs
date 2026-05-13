use anise::prelude::Epoch;
use hifitime::TimeScale;
use moira_core::{calculate_chart, load_almanac, AstrologyData};
use tauri::State;

struct AppState {
    almanac: anise::prelude::Almanac,
}

#[tauri::command]
fn calculate(
    state: State<AppState>,
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    timezone: f64,
    latitude: f64,
    longitude: f64,
) -> Result<AstrologyData, String> {
    let tz_hours = (timezone * 3600.0) as i64;
    let epoch = Epoch::maybe_from_gregorian(
        year,
        month,
        day,
        hour,
        minute,
        second,
        0,
        TimeScale::UTC,
    )
    .map_err(|e| format!("时间解析失败: {}", e))?;

    use hifitime::Duration;
    let utc_epoch = epoch - Duration::from_seconds(tz_hours as f64);

    let data = calculate_chart(&state.almanac, utc_epoch, latitude, longitude, timezone);
    Ok(data)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let almanac = load_almanac("assets/bsp/de440.bsp").expect("加载星历文件失败");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { almanac })
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
