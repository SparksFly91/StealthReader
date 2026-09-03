import { invoke } from "@tauri-apps/api/core"
import { ApiResponse, FeedbackSendParams } from "@/types/global"

const FeedbackApi = {
  /**
   * 发送意见反馈邮件
   * @param params 反馈参数（邮箱、富文本 HTML、纯文本、内联图片）
   * @returns 发送结果
   */
  send: async (params: FeedbackSendParams) => {
    return await invoke<ApiResponse<null>>("feedback_send", { req: params })
  },
}

export default FeedbackApi
