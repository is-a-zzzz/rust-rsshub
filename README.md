# Rust RSSHub

一个基于 Rust 开发的轻量级 RSS 生成器，支持通过配置文件快速为任意网站生成 RSS 订阅。

## 📚 文档

- **[AI 配置生成指南](docs/AI_CONFIG_GUIDE.md)** - 为 AI 设计的完整配置文档，无需阅读代码即可生成配置
- **配置示例** - 查看 `configs/` 目录中的示例文件

## 特性

- ✅ **零配置启动**: 无需预加载配置，访问时动态加载
- ✅ **配置驱动**: 通过 YAML 文件定义解析规则，无需编写代码
- ✅ **自动热重载**: 修改配置文件后自动生效，无需重启
- ✅ **多格式支持**: 支持 RSS 2.0 和 Atom 1.0
- ✅ **智能缓存**: 内置内存缓存，提升性能
- ✅ **高性能**: 基于 Rust 和 Tokio 异步运行时
- ✅ **Docker 部署**: 一键部署，易于扩展

## 快速开始

### 使用 Cargo 运行

```bash
# 克隆项目
git clone <repository>
cd rust-rsshub

# 运行
cargo run

# 访问
curl http://localhost:3001/rss/observationalhazard
```

### 使用 Docker

```bash
# 构建镜像
docker build -t rust-rsshub .

# 运行
docker run -p 3000:3000 -v $(pwd)/configs:/app/configs rust-rsshub
```

## 添加新的订阅源

### 方法 1: 使用 AI 配置指南（推荐）

📖 查看 **[AI 配置生成指南](docs/AI_CONFIG_GUIDE.md)** - 该文档包含：

- 完整的配置字段说明
- 常见博客平台的选择器模式
- 多个完整配置示例
- 故障排查指南
- 最佳实践建议

无需阅读代码，直接参考文档即可快速创建配置。

### 方法 2: 手动配置

1. 在 `configs/` 目录创建 YAML 配置文件:

```bash
vim configs/mysite.yml
```

2. 配置内容:

```yaml
plugin:
  name: "mysite"
  description: "My Blog RSS Feed"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"
  encoding: "utf-8"

parser:
  list:
    selector: "div.post"
    link_selector: "a.title"
    title_selector: "a.title"
    description_selector: "p.summary"
    date_selector: "span.date"
    date_format: "%Y-%m-%d"

cache:
  enabled: true
  ttl: 3600

feed:
  title: "My Blog"
  description: "My Blog Feed"
  link: "https://example.com"
  language: "zh"
  format: "rss"
  limit: 20
```

3. **立即访问**，无需重启:

```bash
curl http://localhost:3001/rss/mysite
```

## API 端点

| 端点 | 方法 | 描述 |
|------|------|------|
| `/` | GET | 欢迎页面 |
| `/health` | GET | 健康检查 |
| `/plugins` | GET | 列出所有插件 |
| `/rss/:name` | GET | 获取 RSS 订阅 |
| `/rss/:name?format=atom` | GET | 获取 Atom 订阅 |

## 环境变量

- `PORT`: 服务器端口（默认: 3001）
- `CONFIGS_DIR`: 配置文件目录（默认: configs）
- `RUST_LOG`: 日志级别（默认: info）

## 项目结构

```
rust-rsshub/
├── src/
│   ├── main.rs           # 主程序
│   ├── lib.rs            # 库入口
│   ├── error.rs          # 错误处理
│   ├── config/           # 配置系统
│   ├── fetcher/          # HTTP客户端和缓存
│   ├── parser/           # HTML解析器
│   ├── generator/        # RSS生成器
│   ├── router/           # Web路由
│   └── plugins/          # 插件系统
├── configs/              # 配置文件目录
├── tests/                # 测试
├── Cargo.toml            # 项目配置
└── README.md             # 本文件
```

## 技术栈

- **语言**: Rust 2021 Edition
- **运行时**: Tokio (异步)
- **Web框架**: Axum
- **HTTP客户端**: Reqwest
- **HTML解析**: Scraper
- **序列化**: Serde (YAML/JSON)
- **RSS生成**: RSS、Atom Syndication

## 开发状态

当前开发进度: **70%** (阶段 7/10 完成)

### ✅ 已完成
- 项目初始化
- 核心数据结构定义
- 配置系统实现
- HTTP 客户端和内容获取
- 解析器系统实现
- RSS 生成器实现
- Web 服务器和路由
- 插件系统核心
- 主程序实现

### 🔄 待完成
- 更多示例配置
- Docker 部署配置
- 完整文档

## 测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test --lib           # 库测试
cargo test --tests         # 集成测试
```

## 许可证

MIT License
