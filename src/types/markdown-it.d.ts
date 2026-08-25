declare module "markdown-it" {
  interface MarkdownItOptions {
    html?: boolean
    xhtmlOut?: boolean
    breaks?: boolean
    langPrefix?: string
    linkify?: boolean
    typographer?: boolean
    quotes?: string | string[]
    highlight?: (str: string, lang: string, attrs: string) => string
  }

  class MarkdownIt {
    constructor(options?: MarkdownItOptions)
    set(options: MarkdownItOptions): this
    render(src: string, env?: unknown): string
    parse(src: string, env?: unknown): unknown[]
    options: MarkdownItOptions
    renderer: unknown
  }

  export default MarkdownIt
}
