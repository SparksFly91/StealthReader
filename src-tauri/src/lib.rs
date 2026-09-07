mod commands;
mod models;
mod services;

use commands::book::*;
use services::db::init_pool;
use tauri::Manager;

use tauri_plugin_prevent_default::Flags;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init()) // 对话框插件
        .plugin(tauri_plugin_updater::Builder::new().build()) // 更新插件
        .plugin(tauri_plugin_process::init()) // 进程插件（更新后重启）
        .plugin(prevent_default())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(async {
                init_pool(app.handle())
                    .await
                    .expect("sqlite数据库连接池初始化失败!")
            });
            app.manage(pool);

            // Windows 11 下 DWM 会默认给顶层窗口裁一个约 8px 的系统圆角，
            // 与前端 CSS 的 12px 圆角不一致时，两段弧线之间会露出一圈透明留白。
            // 这里关闭系统圆角，让窗口边界完全由前端 CSS 圆角决定。
            #[cfg(target_os = "windows")]
            if let Some(window) = app.get_webview_window("main") {
                disable_system_rounded_corners(&window);
            }

            Ok(())
        })
        .plugin(tauri_plugin_store::Builder::default().build())
        // .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri_helper::tauri_collect_commands!())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// 开发模式: 保留DevTools和Reload
#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::Builder::new().with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD)).build()
}

// 生产模式: 阻止所有默认事件
#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}

/// 关闭 Windows 11 的系统默认圆角（DWM 裁剪），避免与前端 CSS 圆角之间出现透明缝隙
#[cfg(target_os = "windows")]
fn disable_system_rounded_corners(window: &tauri::WebviewWindow) {
    use windows::Win32::Graphics::Dwm::{
        DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND, DwmSetWindowAttribute,
    };

    if let Ok(hwnd) = window.hwnd() {
        let preference = DWMWCP_DONOTROUND;
        unsafe {
            if let Err(e) = DwmSetWindowAttribute(
                hwnd,
                DWMWA_WINDOW_CORNER_PREFERENCE,
                &preference as *const _ as *const std::ffi::c_void,
                std::mem::size_of_val(&preference) as u32,
            ) {
                eprintln!("关闭系统圆角失败: {e}");
            }
        }
    }
}