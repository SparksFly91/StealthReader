mod commands;
mod services;
mod models;

use tauri::Manager;
use commands::book::*;
use services::db::init_pool;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            let pool = tauri::async_runtime::block_on(async {
                init_pool().await.expect("sqlite数据库连接池初始化失败!")
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
