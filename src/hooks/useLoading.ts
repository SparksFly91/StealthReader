import { computed } from "vue"
import { useLoadingStore } from "@/stores/loading"

/**
 * 全局加载状态 hook
 * @example
 * const { withLoading } = useLoading()
 * await withLoading(() => BookApi.import(path), "导入中...")
 */
export function useLoading() {
  const store = useLoadingStore()

  const start = (text?: string) => store.start(text)
  const stop = () => store.stop()
  const reset = () => store.reset()

  const withLoading = async <T>(fn: () => Promise<T>, text?: string): Promise<T> => {
    store.start(text)
    try {
      return await fn()
    } finally {
      store.stop()
    }
  }

  return {
    loading: computed(() => store.loading),
    text: computed(() => store.text),
    start,
    stop,
    reset,
    withLoading,
  }
}
