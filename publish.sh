#!/bin/bash

# IrisNote GitHub 发布脚本

echo "================================"
echo "  IrisNote GitHub 发布准备"
echo "================================"
echo ""

# 检查 Git 状态
echo "检查 Git 状态..."
if [ -d ".git" ]; then
    echo "✓ Git 仓库已初始化"
else
    echo "× 未找到 Git 仓库"
    exit 1
fi

# 显示当前分支
BRANCH=$(git branch --show-current 2>/dev/null || echo "main")
echo "当前分支: $BRANCH"
echo ""

# 检查未提交的更改
if [ -n "$(git status --porcelain)" ]; then
    echo "发现未提交的更改:"
    git status --short
    echo ""
    read -p "是否继续提交? (y/n): " -n 1 -r
    echo ""
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "操作已取消"
        exit 1
    fi
fi

# 添加所有文件
echo "添加文件到 Git..."
git add .

# 创建提交
echo "创建初始提交..."
git commit -m "feat: initial release v0.1.0

- IrisNote: Smart text editor with auto-detection
- Markdown and SVG preview support
- Syntax highlighting for 15+ languages
- Intelligent file naming suggestions
- Windows file association
- Apache 2.0 license

🤖 Generated with CodeArts"

echo ""
echo "✓ 提交成功！"
echo ""

# 添加远程仓库
echo "添加远程仓库..."
if git remote | grep -q "origin"; then
    echo "远程仓库已存在，更新 URL..."
    git remote set-url origin https://github.com/itszzl-sudo/irisnote.git
else
    git remote add origin https://github.com/itszzl-sudo/irisnote.git
fi

echo "✓ 远程仓库已配置: https://github.com/itszzl-sudo/irisnote.git"
echo ""

# 推送到 GitHub
read -p "是否推送到 GitHub? (y/n): " -n 1 -r
echo ""
if [[ $REPLY =~ ^[Yy]$ ]]; then
    echo "推送到 GitHub..."
    git push -u origin $BRANCH
    
    if [ $? -eq 0 ]; then
        echo ""
        echo "✓ 推送成功！"
        echo ""
        
        # 创建标签
        read -p "是否创建 v0.1.0 标签? (y/n): " -n 1 -r
        echo ""
        if [[ $REPLY =~ ^[Yy]$ ]]; then
            git tag -a v0.1.0 -m "Release v0.1.0"
            git push origin v0.1.0
            
            if [ $? -eq 0 ]; then
                echo ""
                echo "✓ 标签创建成功！"
                echo ""
                echo "================================"
                echo "  发布完成！"
                echo "================================"
                echo ""
                echo "下一步:"
                echo "1. 访问 https://github.com/itszzl-sudo/irisnote"
                echo "2. 创建 Release (使用标签 v0.1.0)"
                echo "3. 上传构建的可执行文件"
                echo "4. 添加 Release 说明"
            fi
        fi
    else
        echo "× 推送失败，请检查网络连接或认证"
    fi
fi
