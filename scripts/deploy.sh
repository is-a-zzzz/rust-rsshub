#!/bin/bash
set -e

# Docker 部署脚本
# 使用方法:
#   ./deploy.sh          - 使用 docker-compose 启动
#   ./deploy.sh build    - 仅构建镜像
#   ./deploy.sh up       - 启动服务
#   ./deploy.sh down     - 停止服务
#   ./deploy.sh logs     - 查看日志
#   ./deploy.sh rebuild  - 重新构建并启动

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

case "${1:-up}" in
    build)
        echo "构建 Docker 镜像..."
        docker-compose build
        ;;
    up)
        echo "启动服务..."
        docker-compose up -d
        echo "服务已启动！访问 http://localhost:3001"
        ;;
    down)
        echo "停止服务..."
        docker-compose down
        ;;
    restart)
        echo "重启服务..."
        docker-compose restart
        ;;
    logs)
        docker-compose logs -f
        ;;
    rebuild)
        echo "重新构建并启动服务..."
        docker-compose up -d --build
        echo "服务已启动！访问 http://localhost:3001"
        ;;
    status)
        docker-compose ps
        ;;
    health)
        curl -f http://localhost:3001/health || echo "健康检查失败"
        ;;
    *)
        echo "使用方法: $0 {build|up|down|restart|logs|rebuild|status|health}"
        exit 1
        ;;
esac
