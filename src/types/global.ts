interface ApiResponse<T> {
  success: boolean
  code: number
  msg: string
  data: T
}

interface PageResult<T> {
  total: number
  list: T[]
  page: number
  page_size: number
}

interface Books {
  id: number
  title: string
  author: string
  cover: string
  introduction: string
  file_path: string
  total_chapters: number
  total_chars: number
  create_time: string
  last_read_time: string | null
  last_read_chapter_id: number
  last_read_position: number
}

interface Chapters {
  id: number
  book_id: number
  number: number
  title: string
  content: string
  total_chars: number
}

interface BookSaveParams {
  id: number
  title: string
  author: string
  cover: string
  instruction: string
}

export type { ApiResponse, PageResult, Books, Chapters, BookSaveParams }
