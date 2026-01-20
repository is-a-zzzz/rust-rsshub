# 快速参考卡片

## 常用配置模板

### WordPress 博客

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h2.entry-title a"
    link_selector: "h2.entry-title a"
    description_selector: "div.entry-summary p"
    date_selector: "time.published"
    author_selector: "span.author a"
```

### Hugo 静态博客

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h1.post-title a"
    link_selector: "h1.post-title a"
    description_selector: "div.post-content p"
    date_selector: "time.post-date"
```

### Ghost 博客

```yaml
parser:
  list:
    selector: "article.post"
    title_selector: "h2.post-title a"
    link_selector: "h2.post-title a"
    description_selector: "section.post-excerpt p"
    date_selector: "time.post-date"
    author_selector: "span.author-name"
```

### 通用博客

```yaml
parser:
  list:
    selector: "div.post"
    title_selector: "h2 a"
    link_selector: "h2 a"
    description_selector: "p.summary"
    date_selector: "span.date"
```

---

## 常用日期格式

| 显示 | 格式 | date_format |
|------|------|-------------|
| 2025-01-14 | ISO 日期 | `%Y-%m-%d` |
| Jan 14, 2025 | 短月份 | `%b %d, %Y` |
| January 14, 2025 | 完整月份 | `%B %d, %Y` |
| 14/01/2025 | 欧洲格式 | `%d/%m/%Y` |
| 01/14/2025 | 美国格式 | `%m/%d/%Y` |
| 2025-01-14T15:30:00 | ISO 时间 | `%Y-%m-%dT%H:%M:%S` |

---

## CSS 选择器速查

| 模式 | 说明 | 示例 |
|------|------|------|
| `tag` | 标签 | `div`, `h1`, `a` |
| `.class` | 类 | `.post`, `.title` |
| `#id` | ID | `#main`, `#content` |
| `[attr]` | 属性存在 | `[data-id]` |
| `[attr=val]` | 属性等于 | `[class="post"]` |
| `[attr*=val]` | 属性包含 | `[href*="/posts/"]` |
| `[attr^=val]` | 属性开头 | `[href^="http"]` |
| `[attr$=val]` | 属性结尾 | `[href$=".html"]` |
| `parent child` | 后代 | `div.post h2` |
| `parent > child` | 直接子代 | `ul > li` |
| `sel1, sel2` | 或选择器 | `h1, h2` |
| `sel.class` | 标签+类 | `div.post` |

---

## 编码列表

| 编码 | 适用语言 |
|------|----------|
| `utf-8` | 通用（默认） |
| `gb2312` | 简体中文 |
| `gbk` | 简体中文 |
| `big5` | 繁体中文 |
| `shift_jis` | 日文 |
| `euc-jp` | 日文 |
| `euc-kr` | 韩文 |
| `iso-8859-1` | 西欧语言 |

---

## 语言代码

| 代码 | 语言 |
|------|------|
| `zh` | 中文（简体） |
| `zh-TW` | 中文（繁体） |
| `en` | 英语 |
| `ja` | 日语 |
| `ko` | 韩语 |
| `fr` | 法语 |
| `de` | 德语 |
| `es` | 西班牙语 |
| `ru` | 俄语 |
| `ar` | 阿拉伯语 |

---

## 测试命令

```bash
# 测试配置
./scripts/test-config.sh iczelia

# 手动测试
curl http://localhost:3001/rss/iczelia

# 测试 Atom 格式
curl "http://localhost:3001/rss/iczelia?format=atom"

# 查看所有插件
curl http://localhost:3001/plugins

# 健康检查
curl http://localhost:3001/health
```

---

## 调试技巧

### 1. 查看 HTML 结构

```bash
# 获取页面 HTML
curl -s https://example.com > page.html

# 查找文章列表
grep -A 10 "class=\"post\"" page.html

# 查找标题
grep -B 2 -A 2 "<h2" page.html

# 查找链接
grep -oP 'href="[^"]*"' page.html
```

### 2. 浏览器控制台

```javascript
// 测试选择器
document.querySelectorAll('article.post')

// 查看第一个文章的 HTML
document.querySelector('article.post').outerHTML

// 提取链接
document.querySelectorAll('article.post a[href]').forEach(a => console.log(a.href))
```

### 3. 常见问题

| 问题 | 解决方法 |
|------|----------|
| 无文章输出 | 检查选择器是否正确 |
| 日期解析失败 | 检查 date_format 或删除它使用自动识别 |
| 链接错误 | 确保选择器指向 `<a>` 标签 |
| 乱码 | 更改 encoding |
| 被阻止 | 添加 User-Agent 和 Referer |

---

## 最小配置模板

```yaml
plugin:
  name: "my-blog"
  description: "My blog feed"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "article.post"
    link_selector: "h2 a"
    title_selector: "h2 a"

feed:
  title: "My Blog"
  description: "Blog Feed"
  link: "https://example.com"
```

---

## 完整配置模板

```yaml
plugin:
  name: "my-blog"
  description: "Complete example"
  version: "1.0.0"
  author: "Your Name"

source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"
  request:
    method: "GET"
    timeout: 30
    headers:
      User-Agent: "Mozilla/5.0"
      Accept: "text/html"

parser:
  list:
    selector: "article.post"
    link_selector: "h2.title a"
    title_selector: "h2.title a"
    description_selector: "p.excerpt"
    date_selector: "time.date"
    date_format: "%Y-%m-%d"
    author_selector: "span.author"
    category_selector: "span.tags"

  content:
    selector: "article.main"
    content_selector: "div.content"
    cleanup_selectors:
      - "div.ads"
      - "script"

feed:
  title: "My Blog"
  description: "My RSS Feed"
  link: "https://example.com"
  language: "en"
  format: "rss"
  limit: 20
```

---

## 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `PORT` | 3001 | 服务器端口 |
| `CONFIGS_DIR` | configs | 配置目录 |
| `RUST_LOG` | info | 日志级别 |

---

## 目录结构

```
rust-rsshub/
├── configs/           # 配置文件目录
│   ├── iczelia.yml
│   └── example.yml
├── docs/              # 文档
│   ├── AI_CONFIG_GUIDE.md
│   └── QUICK_REFERENCE.md
├── scripts/           # 脚本
│   └── test-config.sh
├── src/               # 源代码
└── tests/             # 测试
```

---

## 快速开始

1. 复制模板：`cp configs/test-example.yml configs/myblog.yml`
2. 编辑配置：`vim configs/myblog.yml`
3. 测试配置：`./scripts/test-config.sh myblog`
4. 访问订阅：`curl http://localhost:3001/rss/myblog`

---

## 保留测试

### 运行所有测试

```bash
cargo test
```

### 运行特定测试

```bash
# 配置测试
cargo test config_test

# 解析器测试
cargo test parser_test

# 集成测试
cargo test integration_test
```

### 查看测试输出

```bash
# 显示输出
cargo test -- --nocapture

# 显示详细日志
RUST_LOG=debug cargo test -- --nocapture
```
