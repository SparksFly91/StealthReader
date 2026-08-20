use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tauri::State;
use tauri_helper::auto_collect_command;

use crate::models::{ApiResponse, Books, Chapters, PageResult};
use crate::services::parser::parse_book;

#[derive(Debug, Deserialize, Serialize)]
pub struct BookSaveReq {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub cover: String,
    pub introduction: String,
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
    page: i32,
    limit: i32,
) -> Result<ApiResponse<PageResult<Chapters>>, String> {
    let offset = (page - 1) * limit;
    let total: i64 =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(id) FROM chapters WHERE book_id = ?")
            .bind(book_id)
            .fetch_one(&*pool)
            .await
            .map_err(|e| e.to_string())?;
    let chapters =
        sqlx::query_as::<_, Chapters>("SELECT * FROM chapters WHERE book_id = ? LIMIT ?,?")
            .bind(book_id)
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
