import { invoke } from "@tauri-apps/api/core"
import { ApiResponse, PageResult, Books, Chapters, BookSaveParams } from "@/types/global"

const BookApi = {
    /**
     * 导入书籍
     * @param path 书籍文件路径
     * @returns 导入结果
     */
    import: async (path: string) => {
        return await invoke<ApiResponse<any>>("book_import", { path })
    },
  /**
   * 获取书籍列表
   * @param title 书籍标题模糊查询
   * @returns 书籍列表
   */
  list: async (title: string) => {
    return await invoke<ApiResponse<Books[]>>("book_list", { title })
  },
  /**
   * 编辑书籍
   * @param params 书籍参数
   * @returns 编辑后的书籍
   */
  edit: async (params: BookSaveParams) => {
    return await invoke<ApiResponse<Books>>("book_edit", { params })
  },
  /**
   * 删除书籍
   * @param id 书籍ID
   * @returns 删除结果
   */
  del: async (id: number) => {
    return await invoke<ApiResponse<any>>("book_del", { id })
  },
  /**
   * 获取书籍详情
   * @param id 书籍ID
   * @returns 书籍详情
   */
  detail: async (id: number) => {
    return await invoke<ApiResponse<Books>>("book_detail", { id })
  },
  /**
   * 获取书籍章节列表
   * @param bookId 书籍ID
   * @param page 页码
   * @param limit 每页数量
   * @returns 书籍章节列表
   */
  chapters: async (bookId: number, page: number, limit: number) => {
    return await invoke<ApiResponse<PageResult<Chapters>>>("chapter_page", { bookId, page, limit })
  },
}

export default BookApi
