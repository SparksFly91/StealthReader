use tauri_helper::auto_collect_command;

#[tauri::command]
#[auto_collect_command]
pub async fn import_book() -> Result<(), String> {
    Ok(())
}