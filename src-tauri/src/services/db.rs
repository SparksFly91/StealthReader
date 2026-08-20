use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use tauri::{AppHandle, Manager};

pub async fn init_pool(app: &AppHandle) -> Result<SqlitePool, sqlx::Error> {
    // 数据库文件路径
    let app_dir = app.path().app_data_dir().expect("无法获取APP数据目录");
    std::fs::create_dir_all(&app_dir).unwrap();
    let db_path = app_dir.join("reader.db");
    // 初始化数据库连接池
    let options = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .connect_with(options)
        .await?;
    sqlx::query(
        "
        CREATE TABLE IF NOT EXISTS books (
        id INTEGER,
        title TEXT DEFAULT '',
        author TEXT DEFAULT '',
        cover TEXT DEFAULT '',
        introduction TEXT DEFAULT '',
        file_path TEXT DEFAULT '',
        total_chapters INTEGER DEFAULT 0,
        total_chars INTEGER DEFAULT 0,
        create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        last_read_chapter_id INTEGER DEFAULT 0,
        last_read_position INTEGER DEFAULT 0,
        last_read_time TIMESTAMP,
        PRIMARY KEY (id)
        );

        CREATE TABLE IF NOT EXISTS chapters (
        id INTEGER,
        book_id INTEGER NOT NULL,
        number INTEGER NOT NULL,
        title TEXT NOT NULL DEFAULT '',
        content TEXT NOT NULL,
        total_chars INTEGER DEFAULT 0,
        FOREIGN KEY (book_id) REFERENCES books ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_chapters_book 
        ON chapters(book_id, number);
    ",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}
