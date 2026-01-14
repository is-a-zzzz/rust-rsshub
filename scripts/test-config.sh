#!/bin/bash

# 配置测试脚本
# 用于快速验证 RSSHub 配置文件

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# 默认配置
SERVER_URL="${SERVER_URL:-http://localhost:3001}"
CONFIG_NAME=""

# 帮助信息
show_help() {
    echo "配置测试脚本 - Rust RSSHub"
    echo ""
    echo "用法:"
    echo "  $0 [选项] <配置名称>"
    echo ""
    echo "选项:"
    echo "  -u, --url URL      服务器 URL (默认: http://localhost:3001)"
    echo "  -h, --help         显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 iczelia                    # 测试 iczelia 配置"
    echo "  $0 -u http://localhost:8080 my-blog  # 测试自定义端口"
    echo ""
}

# 解析参数
while [[ $# -gt 0 ]]; do
    case $1 in
        -u|--url)
            SERVER_URL="$2"
            shift 2
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        -*)
            echo -e "${RED}错误: 未知选项 $1${NC}"
            show_help
            exit 1
            ;;
        *)
            CONFIG_NAME="$1"
            shift
            ;;
    esac
done

# 检查配置名称
if [ -z "$CONFIG_NAME" ]; then
    echo -e "${RED}错误: 请提供配置名称${NC}"
    echo ""
    show_help
    exit 1
fi

# 检查配置文件是否存在
if [ ! -f "configs/${CONFIG_NAME}.yml" ]; then
    echo -e "${RED}错误: 配置文件 configs/${CONFIG_NAME}.yml 不存在${NC}"
    echo ""
    echo "可用的配置文件:"
    ls -1 configs/*.yml 2>/dev/null | sed 's|configs/||' | sed 's|\.yml||' || echo "  (无)"
    exit 1
fi

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}测试配置: ${CONFIG_NAME}${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# 1. 检查服务器是否运行
echo -e "${YELLOW}[1/6] 检查服务器状态...${NC}"
if ! curl -s -f "${SERVER_URL}/health" > /dev/null 2>&1; then
    echo -e "${RED}错误: 无法连接到服务器 ${SERVER_URL}${NC}"
    echo "请确保服务器正在运行: cargo run"
    exit 1
fi
echo -e "${GREEN}✓ 服务器运行正常${NC}"
echo ""

# 2. 列出所有插件
echo -e "${YELLOW}[2/6] 检查插件是否加载...${NC}"
PLUGINS=$(curl -s "${SERVER_URL}/plugins")
if echo "$PLUGINS" | grep -q "\"$CONFIG_NAME\""; then
    echo -e "${GREEN}✓ 插件已加载${NC}"
else
    echo -e "${RED}✗ 插件未加载${NC}"
    echo ""
    echo "已加载的插件:"
    echo "$PLUGINS" | grep -o '"[^"]*"' | tr -d '"' | sed 's/^/  - /'
    exit 1
fi
echo ""

# 3. 测试 RSS 输出
echo -e "${YELLOW}[3/6] 测试 RSS 输出...${NC}"
RSS_OUTPUT=$(curl -s "${SERVER_URL}/rss/${CONFIG_NAME}")
if echo "$RSS_OUTPUT" | grep -q "<rss version"; then
    echo -e "${GREEN}✓ RSS 输出成功${NC}"
else
    echo -e "${RED}✗ RSS 输出失败${NC}"
    echo ""
    echo "输出内容:"
    echo "$RSS_OUTPUT" | head -20
    exit 1
fi
echo ""

# 4. 提取并显示文章数量
echo -e "${YELLOW}[4/6] 检查文章数量...${NC}"
ARTICLE_COUNT=$(echo "$RSS_OUTPUT" | grep -o "<item>" | wc -l)
if [ "$ARTICLE_COUNT" -gt 0 ]; then
    echo -e "${GREEN}✓ 找到 ${ARTICLE_COUNT} 篇文章${NC}"
else
    echo -e "${YELLOW}⚠ 警告: 没有找到文章${NC}"
    echo "可能的原因:"
    echo "  1. 选择器不匹配网站结构"
    echo "  2. 网站需要 JavaScript 渲染"
    echo "  3. 网站有反爬虫机制"
fi
echo ""

# 5. 显示前 3 篇文章
if [ "$ARTICLE_COUNT" -gt 0 ]; then
    echo -e "${YELLOW}[5/6] 显示前 3 篇文章...${NC}"
    # 移除换行符，然后提取文章标题
    echo "$RSS_OUTPUT" | tr -d '\n' | \
        grep -oP '<item>.*?</item>' | \
        sed -n 's/.*<title>\(.*\)<\/title>.*/  \1/p' | \
        head -3 | nl -w2 -s'. '
    echo ""
fi

# 6. 测试 Atom 输出
echo -e "${YELLOW}[6/6] 测试 Atom 输出...${NC}"
ATOM_OUTPUT=$(curl -s "${SERVER_URL}/rss/${CONFIG_NAME}?format=atom")
if echo "$ATOM_OUTPUT" | grep -q "<feed xmlns"; then
    echo -e "${GREEN}✓ Atom 输出成功${NC}"
else
    echo -e "${YELLOW}⚠ Atom 输出可能有问题${NC}"
fi
echo ""

# 总结
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}测试完成！${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "RSS 订阅地址:"
echo "  RSS:  ${SERVER_URL}/rss/${CONFIG_NAME}"
echo "  Atom: ${SERVER_URL}/rss/${CONFIG_NAME}?format=atom"
echo ""
echo "在 RSS 阅读器中使用以上地址即可订阅。"
