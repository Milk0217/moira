use anise::prelude::Epoch;
use hifitime::TimeScale;
use moira_core::{calculate_chart, get_mansion_data, load_almanac, AstrologyData, MansionInfo};
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
    dst_applied: bool,
    is_male: Option<bool>,
    use_sidereal: Option<bool>,
) -> Result<AstrologyData, String> {
    let tz_hours = (timezone * 3600.0) as i64;
    let dst_offset = if dst_applied { 3600 } else { 0 };
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
    let utc_epoch = epoch - Duration::from_seconds((tz_hours + dst_offset) as f64);

    let is_male = is_male.unwrap_or(true);
    let use_sidereal = use_sidereal.unwrap_or(false);
    let mut data = calculate_chart(&state.almanac, utc_epoch, latitude, longitude, timezone, is_male, use_sidereal);
    data.dst_applied = dst_applied;
    Ok(data)
}

#[tauri::command]
fn save_chart(app_handle: tauri::AppHandle, name: String, data: AstrologyData) -> Result<(), String> {
    use tauri::Manager;
    let path = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(path.join("charts")).map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&data).map_err(|e| e.to_string())?;
    std::fs::write(path.join("charts").join(format!("{}.json", name)), json).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn list_charts(app_handle: tauri::AppHandle) -> Result<Vec<String>, String> {
    use tauri::Manager;
    let path = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let charts_dir = path.join("charts");
    if !charts_dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = Vec::new();
    for entry in std::fs::read_dir(charts_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name().to_string_lossy().replace(".json", "");
        names.push(name);
    }
    names.sort();
    Ok(names)
}

#[tauri::command]
fn load_chart(app_handle: tauri::AppHandle, name: String) -> Result<AstrologyData, String> {
    use tauri::Manager;
    let path = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let json = std::fs::read_to_string(path.join("charts").join(format!("{}.json", name)))
        .map_err(|e| format!("加载星盘失败: {}", e))?;
    serde_json::from_str(&json).map_err(|e| format!("解析星盘失败: {}", e))
}

#[tauri::command]
fn get_mansion_info() -> Vec<MansionInfo> {
    get_mansion_data()
}

#[tauri::command]
fn delete_chart(app_handle: tauri::AppHandle, name: String) -> Result<(), String> {
    use tauri::Manager;
    let path = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::remove_file(path.join("charts").join(format!("{}.json", name)))
        .map_err(|e| format!("删除星盘失败: {}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let almanac = load_almanac("assets/bsp/de440.bsp").expect("加载星历文件失败");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState { almanac })
        .invoke_handler(tauri::generate_handler![calculate, save_chart, list_charts, load_chart, delete_chart, get_mansion_info])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
