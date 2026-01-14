# AI 配置生成指南

本文档专为 AI 设计，提供快速生成 RSSHub 配置文件所需的全部信息，无需阅读源代码。

## 目录

1. [快速开始](#快速开始)
2. [配置文件结构](#配置文件结构)
3. [字段详解](#字段详解)
4. [常见选择器模式](#常见选择器模式)
5. [完整示例](#完整示例)
6. [测试配置](#测试配置)
7. [故障排查](#故障排查)
8. [最佳实践](#最佳实践)

---

## 快速开始

### 基本流程

1. 获取目标网站的 HTML 源代码
2. 使用浏览器开发者工具分析文章列表的 HTML 结构
3. 根据结构填写 YAML 配置文件
4. 将配置文件保存到 `configs/` 目录
5. 访问 `http://localhost:3001/rss/[配置文件名]` 测试

### 最小配置示例

```yaml
plugin:
  name: "example"
  description: "Example blog RSS feed"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "article.post"
    link_selector: "a.title"
    title_selector: "a.title"

feed:
  title: "Example Blog"
  description: "Example Blog Feed"
  link: "https://example.com"
```

---

## 配置文件结构

配置文件是 YAML 格式，包含 5 个主要部分：

```yaml
plugin:      # 插件元数据
source:      # 数据源配置
parser:      # 解析规则
cache:       # 缓存配置（可选）
feed:        # Feed 输出配置
```

---

## 字段详解

### 1. plugin（插件元数据）

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `name` | string | ✅ | 插件唯一标识符，只能包含字母、数字、连字符 |
| `description` | string | ✅ | 插件简短描述（1-2 句话） |
| `version` | string | ❌ | 版本号（默认: "1.0.0"） |
| `author` | string | ❌ | 作者名称 |

**示例：**
```yaml
plugin:
  name: "my-blog"
  description: "My personal blog about technology"
  version: "1.0.0"
  author: "John Doe"
```

---

### 2. source（数据源配置）

#### 2.1 通用字段

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `type` | string | ✅ | 数据源类型：`html`、`json`、`xml` |
| `url` | string | ✅ | 目标网站 URL |
| `request` | object | ❌ | HTTP 请求配置 |

#### 2.2 HTML 数据源

```yaml
source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"              # 默认: utf-8
  user_agent: "Mozilla/5.0 ..."  # 可选
  request:
    method: "GET"                # 默认: GET
    timeout: 30                  # 默认: 30（秒）
    headers:                     # 自定义请求头
      Accept: "text/html"
      Cookie: "session=abc123"
```

#### 2.3 JSON 数据源

```yaml
source:
  type: "json"
  url: "https://api.example.com/posts"
  request:
    headers:
      Authorization: "Bearer token123"
```

#### 2.4 XML 数据源

```yaml
source:
  type: "xml"
  url: "https://example.com/feed.xml"
```

---

### 3. parser（解析规则）

#### 3.1 list（列表页解析）

所有选择器都是 CSS 选择器语法。

| 字段 | 类型 | 必填 | 说明 |
|------|------|------|------|
| `selector` | string | ✅ | 文章列表容器选择器 |
| `item_selector` | string | ❌ | 单个文章项选择器（默认从 selector 中选择） |
| `title_selector` | string | ✅ | 标题选择器 |
| `link_selector` | string | ✅ | 链接选择器（必须包含 href 属性） |
| `description_selector` | string | ❌ | 描述/摘要选择器 |
| `date_selector` | string | ❌ | 日期选择器 |
| `date_format` | string | ❌ | 日期格式（见下方说明） |
| `author_selector` | string | ❌ | 作者选择器 |
| `category_selector` | string | ❌ | 分类/标签选择器 |

**日期格式说明：**

| 格式 | 示例 | 说明 |
|------|------|------|
| `%Y-%m-%d` | 2025-01-14 | ISO 日期 |
| `%B %d, %Y` | January 14, 2025 | 英文月份 |
| `%Y-%m-%dT%H:%M:%S` | 2025-01-14T15:30:00 | ISO 时间 |
| `%d/%m/%Y` | 14/01/2025 | 欧洲格式 |

**自动识别格式：**
如果不指定 `date_format`，系统会自动尝试以下格式：
- `%B %d, %Y`
- `%Y-%m-%d`
- `%Y-%m-%dT%H:%M:%S%z`
- `%Y-%m-%dT%H:%M:%SZ`
- `%Y-%m-%dT%H:%M:%S`
- `%a, %d %b %Y %H:%M:%S %z`
- `%a, %d %b %Y %H:%M:%S GMT`
- `%d %b %Y`
- `%d/%m/%Y`
- `%m/%d/%Y`

#### 3.2 content（内容页解析 - 可选）

如果需要获取完整文章内容：

```yaml
parser:
  list:
    # ... 列表配置 ...
  content:
    selector: "article.post"              # 内容页容器
    content_selector: "div.article-body"  # 正文内容选择器
    cleanup_selectors:                    # 要移除的元素
      - "div.ads"
      - "div.related-posts"
      - "script"
```

---

### 4. cache（缓存配置）

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `enabled` | boolean | true | 是否启用缓存 |
| `ttl` | integer | 3600 | 缓存时间（秒） |

**示例：**
```yaml
cache:
  enabled: true
  ttl: 1800  # 30 分钟
```

---

### 5. feed（Feed 输出配置）

| 字段 | 类型 | 默认值 | 说明 |
|------|------|--------|------|
| `title` | string | - | Feed 标题（必填） |
| `description` | string | - | Feed 描述（必填） |
| `link` | string | - | 网站 URL（必填） |
| `language` | string | "en" | 语言代码（zh, en, ja 等） |
| `format` | string | "rss" | 输出格式（rss 或 atom） |
| `limit` | integer | 20 | 最多文章数 |

**示例：**
```yaml
feed:
  title: "My Awesome Blog"
  description: "Latest posts from My Awesome Blog"
  link: "https://example.com"
  language: "zh"
  format: "rss"
  limit: 10
```

---

## 常见选择器模式

### CSS 选择器基础

| 模式 | 说明 | 示例 |
|------|------|------|
| `tag` | 标签选择器 | `div`, `article`, `h1` |
| `.class` | 类选择器 | `.post`, `.article-title` |
| `#id` | ID 选择器 | `#main-content` |
| `[attr]` | 属性选择器 | `[data-id]`, `[href^="/post"]` |
| `parent child` | 后代选择器 | `div.post h2 a` |
| `parent > child` | 直接子代 | `ul > li` |
| `class1.class2` | 多类 | `btn.primary` |

### 常见博客平台模式

#### WordPress

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h2.entry-title a"
    link_selector: "h2.entry-title a"
    description_selector: "div.entry-summary"
    date_selector: "time.published"
    date_format: "%B %d, %Y"
    author_selector: "span.author a"
```

#### Hugo（静态网站）

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h1.post-title a"
    link_selector: "h1.post-title a"
    description_selector: "div.post-content"
    date_selector: "time.post-date"
    date_format: "%Y-%m-%d"
```

#### Ghost

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h2.post-title a"
    link_selector: "h2.post-title a"
    description_selector: "section.post-excerpt"
    date_selector: "time.post-date"
    date_format: "%Y-%m-%d"
    author_selector: "span.author-name"
```

#### 自定义博客

```yaml
parser:
  list:
    selector: "div.blog-post"
    title_selector: "h3.title a"
    link_selector: "h3.title a"
    description_selector: "p.excerpt"
    date_selector: "span.date"
    date_format: "%d/%m/%Y"
```

### 复杂选择器示例

```yaml
# 使用 data 属性
title_selector: "[data-title]"

# 使用部分匹配
link_selector: "a[href*='/posts/']"

# 组合选择器
title_selector: "article.post h2.title a"

# 多个备选（使用第一个匹配的）
# 注意：当前不支持，需要使用最通用的选择器
```

---

## 完整示例

### 示例 1：简单博客（iczelia.yml）

```yaml
plugin:
  name: "iczelia"
  description: "Kamila Szewczyk's personal blog RSS feed"
  version: "1.0.0"

source:
  type: "html"
  url: "https://iczelia.net/"
  encoding: "utf-8"
  request:
    method: "GET"
    timeout: 30
    headers:
      User-Agent: "Mozilla/5.0 (compatible; RSSHub/1.0)"

parser:
  list:
    selector: "article.post.on-list"
    link_selector: "h1.post-title a"
    title_selector: "h1.post-title a"
    description_selector: "div.post-content"
    date_selector: "time.post-date"
    date_format: "%Y-%m-%d"

cache:
  enabled: true
  ttl: 3600

feed:
  title: "Kamila Szewczyk's Blog"
  description: "Trivia for people who already know everything"
  link: "https://iczelia.net/"
  language: "en"
  format: "rss"
  limit: 20
```

**HTML 结构：**
```html
<article class="post on-list">
  <h1 class="post-title">
    <a href="https://iczelia.net/posts/example">Post Title</a>
  </h1>
  <time class="post-date">2025-01-14</time>
  <div class="post-content">
    Post description here...
  </div>
</article>
```

### 示例 2：多字段博客（observationalhazard.yml）

```yaml
plugin:
  name: "observationalhazard"
  description: "David Kopec's blog RSS feed"
  version: "1.0.0"

source:
  type: "html"
  url: "https://www.observationalhazard.com/"
  encoding: "utf-8"

parser:
  list:
    selector: "article.post.hentry"
    link_selector: "h2.post-title.entry-title a"
    title_selector: "h2.post-title.entry-title a"
    description_selector: ".post-body.entry-content"
    date_selector: ".entry-date"
    date_format: "%B %d, %Y"

feed:
  title: "Observational Hazard"
  description: "David Kopec's blog about programming and technology"
  link: "https://www.observationalhazard.com/"
  language: "en"
  format: "rss"
  limit: 20
```

### 示例 3：带完整内容抓取

```yaml
plugin:
  name: "tech-blog"
  description: "Tech Blog with full content"

source:
  type: "html"
  url: "https://techblog.com"

parser:
  list:
    selector: "article.post"
    link_selector: "h2.title a"
    title_selector: "h2.title a"
    description_selector: "p.excerpt"
    date_selector: "time.published"
    date_format: "%Y-%m-%d"

  content:
    selector: "article.main-content"
    content_selector: "div.article-body"
    cleanup_selectors:
      - "div.advertisement"
      - "div.sidebar"
      - "script"
      - "iframe"

cache:
  enabled: true
  ttl: 1800

feed:
  title: "Tech Blog"
  description: "Latest technology articles"
  link: "https://techblog.com"
  language: "en"
  limit: 15
```

### 示例 4：JSON API 数据源

```yaml
plugin:
  name: "json-feed"
  description: "JSON API to RSS"

source:
  type: "json"
  url: "https://api.example.com/posts"
  request:
    headers:
      Authorization: "Bearer your_token_here"

parser:
  list:
    selector: "$.posts[*]"  # JSONPath
    title_selector: "title"
    link_selector: "url"
    description_selector: "summary"
    date_selector: "published_at"
    date_format: "%Y-%m-%dT%H:%M:%S"

feed:
  title: "API Feed"
  description: "Feed from JSON API"
  link: "https://example.com"
  limit: 20
```

---

## 测试配置

### 1. 语法验证

保存配置文件后，启动服务会自动验证：

```bash
cargo run
```

查看输出，如果有配置错误会显示详细的错误信息。

### 2. 功能测试

```bash
# 测试 RSS 输出
curl http://localhost:3001/rss/your-config-name

# 测试 Atom 输出
curl http://localhost:3001/rss/your-config-name?format=atom

# 查看所有可用插件
curl http://localhost:3001/plugins

# 健康检查
curl http://localhost:3001/health
```

### 3. 验证输出

使用在线 RSS 验证工具：
- https://validator.w3.org/feed/
- https://www.rssboard.org/rss-validator/

### 4. 单元测试（可选）

在 `tests/` 目录创建测试文件：

```rust
// tests/my_config_test.rs
use rust_rsshub::config::parser::load_config;
use std::path::Path;

#[tokio::test]
async fn test_my_config() {
    let config = load_config(Path::new("configs/my-config.yml"))
        .await
        .expect("Failed to load config");

    assert_eq!(config.plugin.name, "my-config");
    assert_eq!(config.feed.title, "My Blog");
}
```

运行测试：
```bash
cargo test my_config
```

---

## 故障排查

### 问题 1：没有文章输出

**可能原因：**
1. 选择器不匹配 HTML 结构
2. 动态加载内容（JavaScript 渲染）
3. 网站反爬虫机制

**解决方法：**

```bash
# 1. 获取实际 HTML
curl -s https://example.com > page.html

# 2. 检查 HTML 结构
grep -A 10 "class=\"post\"" page.html

# 3. 测试选择器
# 在浏览器控制台运行：
document.querySelectorAll('article.post')

# 4. 添加 User-Agent
source:
  request:
    headers:
      User-Agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
```

### 问题 2：日期解析失败

**可能原因：**
- 日期格式不匹配
- 日期包含额外字符

**解决方法：**

```bash
# 1. 查看实际日期格式
curl -s https://example.com | grep -oP '<time[^>]*>.*?</time>'

# 2. 调整 date_format
date_selector: "time.post-date"
date_format: "%Y-%m-%d ::"  # 包含额外字符

# 3. 或者使用自动识别（不指定 date_format）
date_selector: "time.post-date"
```

### 问题 3：链接不正确

**可能原因：**
- 相对链接未正确处理
- 链接选择器错误

**解决方法：**

```yaml
# 1. 确保选择器指向 <a> 标签
link_selector: "h2.title a"  # ✓ 正确
link_selector: "h2.title"    # ✗ 错误

# 2. 检查 HTML 中的 href
# <a href="/post/123"> -> 自动处理
# <a href="https://other.com/post"> -> 保持原样
```

### 问题 4：特殊字符乱码

**解决方法：**

```yaml
source:
  encoding: "gb2312"  # 中文网站
  # 或
  encoding: "big5"    # 繁体中文
  # 或
  encoding: "shift_jis"  # 日文
```

### 问题 5：被反爬虫阻止

**解决方法：**

```yaml
source:
  request:
    headers:
      User-Agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"
      Accept: "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8"
      Accept-Language: "en-US,en;q=0.9"
      Accept-Encoding: "gzip, deflate"
      Connection: "keep-alive"
      Referer: "https://example.com"
    timeout: 60  # 增加超时时间

cache:
  ttl: 7200  # 减少请求频率（2 小时）
```

---

## 最佳实践

### 1. 选择器优化

```yaml
# ✓ 使用最具体但不过度限制的选择器
selector: "article.post"           # 好
selector: "div#main div.post"     # 太具体
selector: "div"                    # 太通用

# ✓ 优先使用类选择器而非标签
selector: ".post-item"             # 好
selector: "div.post-item"          # 也可以

# ✓ 避免使用索引（CSS 不支持）
# ✗ selector: "article.post:first-child"
# ✓ selector: "article.post"
```

### 2. 性能优化

```yaml
# 限制文章数量
feed:
  limit: 10  # 只取前 10 篇

# 使用缓存
cache:
  enabled: true
  ttl: 3600  # 1 小时缓存

# 设置超时
source:
  request:
    timeout: 30  # 30 秒超时
```

### 3. 错误处理

```yaml
# 提供描述性信息
plugin:
  description: "If this fails, the site structure may have changed"

# 使用可选字段
parser:
  list:
    description_selector: "p.summary"  # 如果没有，不会报错
```

### 4. 命名规范

```yaml
# 文件名使用小写和连字符
configs/my-awesome-blog.yml

plugin:
  name: "my-awesome-blog"  # 与文件名一致（去掉 .yml）
```

### 5. 文档注释

```yaml
# 虽然不支持注释，但可以使用 description
plugin:
  name: "blog"
  description: "Source: https://example.com | Last tested: 2025-01-14"
```

---

## 快速参考卡片

### 常用日期格式

| 格式 | date_format |
|------|-------------|
| 2025-01-14 | `%Y-%m-%d` |
| January 14, 2025 | `%B %d, %Y` |
| 14/01/2025 | `%d/%m/%Y` |
| 2025-01-14T15:30:00 | `%Y-%m-%dT%H:%M:%S` |

### 常用语言代码

| 语言 | code |
|------|------|
| 中文（简体） | zh |
| 中文（繁体） | zh-TW |
| 英语 | en |
| 日语 | ja |
| 韩语 | ko |

### CSS 选择器优先级

1. ID 选择器：`#id`
2. 类选择器：`.class`
3. 标签选择器：`div`
4. 属性选择器：`[attr]`

---

## 附录

### A. 支持的编码

- utf-8 (默认)
- gb2312 (简体中文)
- gbk (简体中文)
- big5 (繁体中文)
- shift_jis (日文)
- euc-jp (日文)
- euc-kr (韩文)
- iso-8859-1 (西欧)

### B. HTTP 方法

- GET (默认)
- POST
- PUT
- DELETE
- PATCH

### C. 环境变量

- `PORT`: 服务器端口（默认: 3001）
- `CONFIGS_DIR`: 配置文件目录（默认: configs）
- `RUST_LOG`: 日志级别（info, debug, warn, error）

---

## 联系与反馈

如有问题或建议，请：
1. 检查本文档的故障排查部分
2. 查看现有配置文件示例
3. 运行测试验证配置

**记住：配置文件热加载生效，修改后无需重启服务！**
