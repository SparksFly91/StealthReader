use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, Books, Chapters, PageResult, ReadingProgress};
use crate::services::parser::parse_book;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookSaveReq {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub introduction: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct ChaptersResp {
    pub id: i64,
    pub book_id: i64,
    pub number: i32,
    pub title: String,
    pub content: String,
    pub total_chars: i32,
    pub is_read: bool,
}

/**
 * 导入书籍
 * @param path 书籍文件路径
 * @returns 导入结果
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_import(
    pool: State<'_, SqlitePool>,
    path: String,
) -> Result<ApiResponse<()>, String> {
    let parsed = parse_book(&path)?;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        "INSERT INTO books (title, author, introduction, file_path, total_chapters, total_chars) VALUES (?,?,?,?,?,?)",
    )
    .bind(&parsed.title)
    .bind(&parsed.author)
    .bind(&parsed.introduction)
    .bind(&path)
    .bind(parsed.total_chapters)
    .bind(parsed.total_chars)
    .execute(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    let book_id = result.last_insert_rowid();

    for chapter in &parsed.chapters {
        sqlx::query(
            "INSERT INTO chapters (book_id, number, title, content, total_chars) VALUES (?,?,?,?,?)",
        )
        .bind(book_id)
        .bind(chapter.number)
        .bind(&chapter.title)
        .bind(&chapter.content)
        .bind(chapter.total_chars)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(ApiResponse::<()>::success_empty())
}

/**
 * 获取书籍列表
 *
 * @param title 书籍标题模糊查询
 * @returns 书籍列表
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_list(
    pool: State<'_, SqlitePool>,
    title: String,
) -> Result<ApiResponse<Vec<Books>>, String> {
    let title = title.trim().to_string();
    let books = sqlx::query_as::<_, Books>("SELECT * FROM books WHERE title LIKE ?")
        .bind(format!("%{}%", title))
        .fetch_all(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ApiResponse::success(books))
}

/**
 * 更新书籍
 * @param id 书籍ID
 * @returns 更新结果
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_edit(
    pool: State<'_, SqlitePool>,
    params: BookSaveReq,
) -> Result<ApiResponse<()>, String> {
    sqlx::query("UPDATE books SET title = ?, author = ?, cover = ?, introduction = ? WHERE id = ?")
        .bind(params.title)
        .bind(params.author)
        .bind(params.cover)
        .bind(params.introduction)
        .bind(params.id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ApiResponse::<()>::success_empty())
}

/**
 * 删除书籍
 * @param id 书籍ID
 * @returns 删除结果
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_del(pool: State<'_, SqlitePool>, id: i64) -> Result<ApiResponse<()>, String> {
    sqlx::query("DELETE FROM books WHERE id = ?")
        .bind(id)
        .execute(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ApiResponse::<()>::success_empty())
}

/**
 * 获取书籍详情
 * @param id 书籍ID
 * @returns 书籍详情
 */
#[tauri::command]
#[auto_collect_command]
pub async fn book_detail(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<ApiResponse<Books>, String> {
    let book = sqlx::query_as::<_, Books>("SELECT * FROM books WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(ApiResponse::success(book))
}

/**
 * 获取书籍章节列表
 * @param book_id 书籍ID
 * @param page 页码
 * @param limit 每页数量
 * @returns 章节列表
 */
#[tauri::command]
#[auto_collect_command]
pub async fn chapter_page(
    pool: State<'_, SqlitePool>,
    book_id: i64,
    keyword: String,
    page: i32,
    limit: i32,
) -> Result<ApiResponse<PageResult<ChaptersResp>>, String> {
    let offset = (page - 1) * limit;
    let total: i64 = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(id) FROM chapters WHERE book_id = ? AND title LIKE ?",
    )
    .bind(book_id)
    .bind(format!("%{}%", keyword))
    .fetch_one(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    let chapters = sqlx::query_as::<_, ChaptersResp>(
        "SELECT
            ch.*,
            CASE
                WHEN rp.id IS NULL THEN 0
                ELSE 1
            END AS is_read
        FROM
        chapters AS ch
        LEFT JOIN reading_progress AS rp ON ch.book_id = rp.book_id
        AND ch.id = rp.chapter_id
        AND ch.number = rp.chapter_number
        WHERE
        ch.book_id = ?
        AND ch.title LIKE ?
        LIMIT
        ?,?",
    )
    .bind(book_id)
    .bind(format!("%{}%", keyword))
    .bind(offset)
    .bind(limit)
    .fetch_all(&*pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(ApiResponse::success(PageResult {
        total,
        list: chapters,
        page,
        page_size: limit,
    }))
}

/**
 * 获取书籍章节详情
 * @param id 章节ID
 * @returns 章节详情
 */
#[tauri::command]
#[auto_collect_command]
pub async fn chapter_detail(
    pool: State<'_, SqlitePool>,
    id: i64,
) -> Result<ApiResponse<Chapters>, String> {
    let chapter = sqlx::query_as::<_, Chapters>("SELECT * FROM chapters WHERE id = ?")
        .bind(id)
        .fetch_one(&*pool)
        .await
        .map_err(|e| e.to_string())?;
    // 记录阅读进度
    record_reading_progress(&*pool, &chapter).await?;
    Ok(ApiResponse::success(chapter))
}

/**
 * 获取相邻章节
 * @param book_id 书籍ID
 * @param number 当前章节号
 * @param offset 偏移量（-1 上一章，1 下一章）
 * @returns 相邻章节，不存在则为 null
 */
#[tauri::command]
#[auto_collect_command]
pub async fn chapter_nav(
    pool: State<'_, SqlitePool>,
    book_id: i64,
    number: i32,
    offset: i32,
) -> Result<ApiResponse<Option<Chapters>>, String> {
    // 查询相邻章节
    let chapter =
        sqlx::query_as::<_, Chapters>("SELECT * FROM chapters WHERE book_id = ? AND number = ?")
            .bind(book_id)
            .bind(number + offset)
            .fetch_optional(&*pool)
            .await
            .map_err(|e| e.to_string())?;
    if let Some(chapter) = &chapter {
        record_reading_progress(&*pool, chapter).await?;
    }
    Ok(ApiResponse::success(chapter))
}

/// 记录阅读进度：更新书籍最后阅读位置 + 写入阅读记录
async fn record_reading_progress(pool: &SqlitePool, chapter: &Chapters) -> Result<(), String> {
    let is_exist_last_record = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS ( SELECT 1 FROM books WHERE id = ? AND last_read_chapter_id = ? )",
    )
    .bind(chapter.book_id)
    .bind(chapter.id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;
    let last_read_time = chrono::Utc::now().naive_utc();
    // 记录书籍最后一次阅读进度
    if is_exist_last_record {
        sqlx::query("UPDATE books SET last_read_time = ?, last_read_position = ? WHERE id = ?")
            .bind(last_read_time)
            .bind(0)
            .bind(chapter.book_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;
    } else {
        sqlx::query("UPDATE books SET last_read_chapter_id = ?, last_read_time = ?, last_read_position = ? WHERE id = ?")
        .bind(chapter.id)
        .bind(last_read_time)
        .bind(0)
        .bind(chapter.book_id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    let is_read = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS ( SELECT 1 FROM reading_progress WHERE book_id = ? AND chapter_id = ? )",
    )
    .bind(chapter.book_id)
    .bind(chapter.id)
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())?;
    if is_read {
        // 更新阅读记录
        sqlx::query(
            "UPDATE reading_progress SET read_count = read_count + 1, last_read_time = ? WHERE book_id = ? AND chapter_id = ?",
        )
        .bind(last_read_time)
        .bind(chapter.book_id)
        .bind(chapter.id)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    } else {
        // 记录阅读记录
        sqlx::query(
            "INSERT INTO reading_progress (book_id, chapter_id, chapter_number) VALUES (?, ?, ?)",
        )
        .bind(chapter.book_id)
        .bind(chapter.id)
        .bind(chapter.number)
        .execute(pool)
        .await
        .map_err(|e| e.to_string())?;
    }

    Ok(())
}
