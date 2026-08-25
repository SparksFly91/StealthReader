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