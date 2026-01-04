# Rust RSSHub

一个基于 Rust 开发的轻量级 RSS 生成器，支持通过配置文件快速为任意网站生成 RSS 订阅。

## 🚧 当前状态

**开发进度**: 50% (阶段5/10完成)

此项目正处于积极开发中。目前已完成核心功能的实现和测试。

### ✅ 已完成功能

- 阶段1: 项目初始化
- 阶段2: 核心数据结构定义
- 阶段3: 配置系统实现
- 阶段4: HTTP 客户端和内容获取
- 阶段5: 解析器系统实现

### 🔄 待实现功能

- 阶段6: RSS 生成器
- 阶段7: Web 服务器和路由
- 阶段8: 插件系统核心
- 阶段9: 主程序和示例配置
- 阶段10: Docker 部署配置

---

## 📊 项目统计

- **代码行数**: ~2,850
- **测试数量**: 89个 (100%通过)
- **依赖包**: 354个
- **代码质量**: clippy 检查通过，无警告

---

## 🛠️ 技术栈

- **语言**: Rust 2021 Edition
- **运行时**: Tokio (异步)
- **Web框架**: Axum
- **HTTP客户端**: Reqwest
- **HTML解析**: Scraper
- **序列化**: Serde (YAML/JSON)
- **RSS生成**: RSS、Atom Syndication

---

## 📖 文档

- [详细实施计划](DETAILED_PLAN.md) - 完整的10阶段实施计划
- [实施进度报告](IMPLEMENTATION_PROGRESS.md) - 已完成阶段的详细报告
- [测试报告](TEST_REPORT.md) - 测试执行情况和覆盖率

---

## 🧪 测试

运行所有测试：
```bash
cargo test
```

运行特定测试：
```bash
cargo test --lib           # 库测试
cargo test --tests         # 集成测试
cargo test --test parser   # 特定测试文件
```

---

## 📁 项目结构

```
rust-rsshub/
├── src/                    # 源代码
│   ├── config/             # 配置系统 ✅
│   ├── fetcher/            # HTTP客户端和缓存 ✅
│   ├── parser/             # HTML解析器 ✅
│   ├── router/             # 路由（待实现）
│   ├── generator/          # RSS生成器（待实现）
│   └── plugins/            # 插件系统（待实现）
├── configs/                # 配置文件目录 ✅
├── tests/                  # 集成测试 ✅
├── docs/                   # 文档
└── Cargo.toml              # 项目配置 ✅
```

---

## 🎯 特性

### 已实现

- ✅ YAML 驱动的配置系统
- ✅ 异步 HTTP 客户端（支持自定义headers、超时）
- ✅ 内存缓存（TTL自动过期）
- ✅ HTML解析（CSS选择器）
- ✅ 智能日期解析（支持11种格式）
- ✅ URL规范化（自动处理相对路径）
- ✅ 完整的错误处理
- ✅ 89个测试用例，100%通过

### 规划中

- ⏳ RSS 2.0 / Atom 1.0 生成
- ⏳ RESTful API
- ⏳ 插件热重载
- ⏳ 内容页解析
- ⏳ Docker 部署

---

## 🚀 快速开始（当所有功能完成时）

### 使用 Cargo 运行

```bash
# 克隆项目
git clone <repository>
cd rust-rsshub

# 运行
cargo run

# 访问 RSS
curl http://localhost:3000/rss/example
```

### 使用 Docker

```bash
# 构建镜像
docker build -t rust-rsshub .

# 运行
docker run -p 3000:3000 -v $(pwd)/configs:/app/configs rust-rsshub
```

---

## 📝 添加新的订阅源（未来功能）

创建 YAML 配置文件：

```yaml
plugin:
  name: "my-site"
  description: "My Blog RSS Feed"
  version: "1.0.0"

source:
  type: "html"
  url: "https://example.com"

parser:
  list:
    selector: "div.post"
    link_selector: "a.title"
    title_selector: "a.title"

feed:
  title: "My Blog"
  description: "My Blog Feed"
  link: "https://example.com"
```

---

## 📄 许可证

MIT License

---

## 🤝 贡献

欢迎贡献！请查看 [DETAILED_PLAN.md](DETAILED_PLAN.md) 了解实施计划。

---

**注意**: 此项目目前处于开发阶段，部分功能尚未实现。请参考 [IMPLEMENTATION_PROGRESS.md](IMPLEMENTATION_PROGRESS.md) 了解最新进度。
