# 构建阶段
FROM rust:1.83-alpine AS builder

# 安装依赖
RUN apk add --no-cache musl-dev pkgconfig

# 设置工作目录
WORKDIR /app

# 复制 Cargo 文件
COPY Cargo.toml Cargo.lock ./

# 创建虚拟目录结构
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    touch src/lib.rs

# 构建依赖(利用缓存)
RUN cargo build --release && \
    rm -rf src

# 复制源代码
COPY src ./src

# 构建实际项目
RUN touch src/main.rs && \
    cargo build --release

# 运行阶段
FROM alpine:latest

# 安装运行时依赖
RUN apk add --no-cache ca-certificates

# 创建用户
RUN addgroup -S rsshub && \
    adduser -S rsshub -G rsshub

# 设置工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/release/rust-rsshub /app/rust-rsshub

# 创建配置目录
RUN mkdir -p /app/configs && \
    chown -R rsshub:rsshub /app

# 切换用户
USER rsshub

# 暴露端口
EXPOSE 3000

# 设置环境变量
ENV CONFIGS_DIR=/app/configs
ENV PORT=3000

# 启动应用
CMD ["/app/rust-rsshub"]
