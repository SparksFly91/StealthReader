use tauri_helper::auto_collect_command;

use crate::models::ApiResponse;

/**
 * 导入书籍
 * @param path 书籍文件路径
 * @returns 导入结果
 */
#[tauri::command]
#[auto_collect_command]
pub async fn import_book(path: String) -> Result<ApiResponse<()>, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string());
    Ok(ApiResponse::success(()))
}

/**
 * 获取书籍列表
 * @returns 书籍列表
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_list() -> Result<ApiResponse<Vec<()>>, String> {
    Ok(ApiResponse::success(Vec::new()))
}
