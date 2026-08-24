mod commands;
mod models;
mod services;

use commands::book::*;
use services::db::init_pool;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init()) // 对话框插件
        .plugin(tauri_plugin_updater::Builder::new().build()) // 更新插件
        .plugin(tauri_plugin_process::init()) // 进程插件（更新后重启）
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
