fn main() {
    tauri_helper::generate_command_file(tauri_helper::TauriHelperOptions::default());
    tauri_build::build()
}
