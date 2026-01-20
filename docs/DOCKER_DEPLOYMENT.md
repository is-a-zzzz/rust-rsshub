# Docker 部署指南 (Scratch 裸镜像)

本文档介绍如何使用 Docker 部署 rust-rsshub，使用 **scratch 裸镜像**实现最小化体积。

## 为什么使用 Scratch 裸镜像？

| 特性 | Alpine | Scratch |
|-----|--------|---------|
| 镜像大小 | ~20MB | ~5MB |
| 基础文件 | 完整 OS | 仅二进制 + CA 证书 |
| 安全性 | 低（更多攻击面） | 高（最小攻击面） |
| 调试难度 | 容易（有 shell） | 困难（无 shell） |

## 静态编译说明

为了使用 scratch 镜像，项目进行了以下调整：

1. **reqwest** 改用 `rustls-tls` 替代 `native-tls`
   - 移除 OpenSSL 动态库依赖
   - 使用纯 Rust 实现的 TLS 库

2. **移除 notify** 依赖
   - inotify 需要系统调用支持
   - scratch 不支持文件系统监控

## 目录结构

```
.
├── Dockerfile           # 多阶段构建 + Scratch 裸镜像
├── docker-compose.yml   # Docker Compose 编排文件
├── .dockerignore        # Docker 构建忽略文件
├── .env.example         # 环境变量示例
└── configs/             # 配置文件目录（挂载到容器）
```

## 快速开始

### 1. 准备配置文件

确保 `configs/` 目录下有你的 YAML 配置文件：

```bash
ls configs/
# 例如: iczelia.yml, example.yml
```

### 2. 配置环境变量（可选）

```bash
cp .env.example .env
# 编辑 .env 文件修改配置
```

### 3. 启动服务

```bash
# 使用部署脚本
./scripts/deploy.sh up

# 或使用 docker-compose
docker-compose up -d --build
```

### 4. 验证服务

```bash
# 查看服务状态
docker-compose ps

# 健康检查
curl http://localhost:3001/health

# 查看日志
docker-compose logs -f
```

## 部署脚本使用

```bash
./scripts/deploy.sh build    # 仅构建镜像
./scripts/deploy.sh up       # 启动服务
./scripts/deploy.sh down     # 停止服务
./scripts/deploy.sh restart  # 重启服务
./scripts/deploy.sh logs     # 查看日志
./scripts/deploy.sh rebuild  # 重新构建并启动
./scripts/deploy.sh status   # 查看状态
./scripts/deploy.sh health   # 健康检查
```

## 配置说明

### 端口映射

| 容器端口 | 主机端口 | 说明 |
|---------|---------|------|
| 3000    | 3001    | HTTP 服务 |

### 环境变量

| 变量名        | 默认值        | 说明           |
|--------------|--------------|---------------|
| RUST_LOG     | info         | 日志级别       |
| PORT         | 3000         | 容器内服务端口  |
| CONFIGS_DIR  | /configs     | 配置文件目录   |

### 数据卷

| 容器路径     | 主机路径    | 权限 | 说明         |
|------------|-----------|------|-------------|
| /configs   | ./configs | ro   | 配置文件目录 |

## Dockerfile 说明

**多阶段构建 + scratch 裸镜像**：

### 构建阶段
- 基础镜像：`rust:1.83-alpine`
- 编译目标：musl (静态链接)
- RUSTFLAGS：`-C target-feature=+crt-static`

### 运行阶段
- 基础镜像：`scratch` (完全空白的镜像)
- 包含文件：
  - `/rust-rsshub` - 静态编译的二进制文件
  - `/etc/ssl/certs/ca-certificates.crt` - CA 证书（HTTPS 需要）

## 健康检查

由于 scratch 镜像没有 wget 等工具，健康检查配置为基本检查：

```yaml
healthcheck:
  test: ["CMD-SHELL", "exit 0"]
```

推荐使用外部监控工具（如 Prometheus + Alertmanager）进行应用级健康检查。

```bash
# 手动检查
curl http://localhost:3001/health
```

## 故障排查

### 容器无法启动

由于 scratch 镜像没有 shell，无法进入容器调试。建议：

```bash
# 查看详细日志
docker-compose logs --tail=100

# 使用 Alpine 镜像调试（临时修改 Dockerfile）
FROM alpine:latest
# 添加测试工具
RUN apk add --no-cache curl
# 启动时检查配置
CMD ["sh", "-c", "ls -la /configs && /rust-rsshub"]
```

### 构建失败

检查是否正确安装了 musl-dev：

```bash
# 检查构建日志
docker-compose build 2>&1 | grep -i error

# 常见问题
# 1. 链接错误：确保 RUSTFLAGS 正确设置
# 2. 找不到 musl-gcc：检查 Alpine 镜像版本
```

### 证书问题

如果遇到 HTTPS 请求失败：

```bash
# 检查证书文件是否复制
docker run --rm -it --entrypoint ls rust-rsshub /etc/ssl/certs/
```

## 镜像大小对比

```bash
# 构建后查看镜像大小
docker images | grep rust-rsshub

# 预期结果
# REPOSITORY       TAG        SIZE
# rust-rsshub      latest     ~5MB   # Scratch
# rust-rsshub      alpine     ~20MB  # Alpine (旧版)
```

## 生产部署建议

### 1. 镜像优化

当前镜像已优化到极致 (~5MB)，无需额外优化。

### 2. 安全加固

Scratch 镜像已是最小攻击面，额外建议：
- 使用非 root 用户（但 scratch 没有用户系统，需要通过 Docker --user 实现）
- 定期更新基础镜像和依赖
- 使用 Docker Content Trust 验证镜像

```yaml
# docker-compose.yml 中添加
user: "65534:65534"  # 使用 nobody 用户
```

### 3. 监控和日志

由于 scratch 无法进入容器，推荐：

```yaml
# 添加更详细的日志配置
logging:
  driver: "json-file"
  options:
    max-size: "10m"
    max-file: "3"
    labels: "service=rsshub"

# 外部日志收集
# 1. 使用 Loki/Promtail 收集日志
# 2. 使用 Elasticsearch + Filebeat
# 3. 使用 Docker logging driver 发送到 syslog
```

### 4. 性能优化

```yaml
# 资源限制
deploy:
  resources:
    limits:
      cpus: '1'
      memory: 256M
    reservations:
      cpus: '0.25'
      memory: 128M

# 性能监控
# 使用 Docker stats 查看
docker stats rust-rsshub
```

### 5. 部署到 Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-rsshub
spec:
  replicas: 2
  selector:
    matchLabels:
      app: rust-rsshub
  template:
    metadata:
      labels:
        app: rust-rsshub
    spec:
      containers:
      - name: rsshub
        image: your-registry/rust-rsshub:latest
        ports:
        - containerPort: 3000
        resources:
          requests:
            memory: "128Mi"
            cpu: "250m"
          limits:
            memory: "256Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 10
          periodSeconds: 30
        readinessProbe:
          httpGet:
            path: /health
            port: 3000
          initialDelaySeconds: 5
          periodSeconds: 10
        volumeMounts:
        - name: configs
          mountPath: /configs
          readOnly: true
      volumes:
      - name: configs
        configMap:
          name: rsshub-configs
```

## 构建和发布

```bash
# 构建镜像
docker build -t your-registry/rust-rsshub:latest .

# 本地测试
docker run -p 3001:3000 -v $(pwd)/configs:/configs:ro your-registry/rust-rsshub:latest

# 推送到镜像仓库
docker push your-registry/rust-rsshub:latest

# 多架构构建（需要 buildx）
docker buildx create --use
docker buildx build --platform linux/amd64,linux/arm64 -t your-registry/rust-rsshub:latest --push .
```

## 从 Alpine 迁移

如果你之前使用 Alpine 版本，迁移到 Scratch：

1. 确保配置文件路径从 `/app/configs` 改为 `/configs`
2. 检查日志是否正常输出
3. 验证 HTTPS 请求是否正常（CA 证书已包含）

## 优势和限制

### 优势
- **极小镜像**：~5MB vs Alpine ~20MB
- **更安全**：最小攻击面
- **更快部署**：更少的拉取时间

### 限制
- **无法调试**：容器内无 shell
- **健康检查受限**：需要外部工具
- **无用户系统**：通过 Docker --user 实现

## 总结

Scratch 裸镜像非常适合生产环境部署，配合外部监控和日志收集系统，可以获得最佳的性能和安全性。
