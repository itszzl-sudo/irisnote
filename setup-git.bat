@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo ====================================
echo   IrisNote Git 配置脚本
echo ====================================
echo.

REM 进入项目目录
cd /d "%~dp0"

echo 当前目录: %CD%
echo.

REM 步骤 1: 配置本地用户信息
echo [步骤 1/4] 配置本地 Git 用户信息...
echo.
echo 全局配置 (用于其他项目):
git config --global user.name
git config --global user.email
echo.
echo 为本项目设置专属配置...
git config user.name "itszzl-sudo"
git config user.email "irisverse@outlook.com"
echo.
echo ✓ 本地配置完成
echo   用户名: itszzl-sudo
echo   邮箱: irisverse@outlook.com
echo.

REM 步骤 2: 检查远程仓库
echo [步骤 2/4] 配置远程仓库...
echo.
git remote | findstr "origin" >nul
if not errorlevel 1 (
    echo 当前远程仓库配置:
    git remote -v
    echo.
    set /p RECONFIG="远程仓库已存在，是否重新配置? (y/n): "
    if /i "!RECONFIG!"=="y" (
        git remote remove origin
        goto :add_remote
    ) else (
        goto :step3
    )
) else (
    goto :add_remote
)

:add_remote
echo.
echo 需要配置 GitHub Personal Access Token
echo.
echo 如何获取 Token:
echo 1. 访问 https://github.com/settings/tokens
echo 2. 点击 "Generate new token (classic)"
echo 3. 勾选 'repo' 权限
echo 4. 生成并复制 Token
echo.
set /p TOKEN="请输入您的 GitHub Token: "

if "!TOKEN!"=="" (
    echo × Token 不能为空
    pause
    exit /b 1
)

echo.
echo 正在添加远程仓库...
git remote add origin https://!TOKEN!@github.com/itszzl-sudo/irisnote.git

if errorlevel 1 (
    echo × 添加远程仓库失败
    pause
    exit /b 1
)

echo ✓ 远程仓库已添加
git remote -v
echo.

:step3
REM 步骤 3: 验证配置
echo [步骤 3/4] 验证配置...
echo.
echo 本地配置:
echo   user.name = 
git config user.name
echo   user.email = 
git config user.email
echo.
echo 远程仓库:
git remote -v
echo.

REM 步骤 4: 测试连接
echo [步骤 4/4] 测试 GitHub 连接...
echo.
set /p TEST_CONN="是否测试连接? (y/n): "
if /i "!TEST_CONN!"=="y" (
    echo 正在测试连接...
    git ls-remote origin >nul 2>&1
    if not errorlevel 1 (
        echo ✓ 连接成功！可以推送到 GitHub
    ) else (
        echo × 连接失败，请检查 Token 是否正确
        echo   可能原因:
        echo   1. Token 无效或已过期
        echo   2. 仓库不存在 (请先在 GitHub 创建)
        echo   3. Token 权限不足
    )
    echo.
)

echo ====================================
echo   配置完成！
echo ====================================
echo.
echo 下一步:
echo   1. 运行 publish.bat 推送到 GitHub
echo   2. 或手动执行:
echo      git add .
echo      git commit -m "Initial commit"
echo      git push -u origin master
echo.
echo 详细说明请查看: GIT_SETUP.md
echo.

pause
