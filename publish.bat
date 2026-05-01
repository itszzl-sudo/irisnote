@echo off
chcp 65001 >nul
setlocal enabledelayedexpansion

echo ================================
echo   IrisNote GitHub 发布准备
echo ================================
echo.

REM 检查 Git 状态
echo 检查 Git 状态...
git rev-parse --is-inside-work-tree >nul 2>&1
if errorlevel 1 (
    echo × 未找到 Git 仓库
    pause
    exit /b 1
)
echo ✓ Git 仓库已初始化
echo.

REM 显示当前分支
for /f "tokens=*" %%i in ('git branch --show-current 2^>nul') do set BRANCH=%%i
if "%BRANCH%"=="" set BRANCH=main
echo 当前分支: %BRANCH%
echo.

REM 检查未提交的更改
git status --porcelain >nul 2>&1
if not errorlevel 1 (
    for /f "tokens=*" %%i in ('git status --porcelain') do (
        if not "%%i"=="" (
            echo 发现未提交的更改
            git status --short
            echo.
            set /p CONTINUE="是否继续提交? (y/n): "
            if /i not "!CONTINUE!"=="y" (
                echo 操作已取消
                pause
                exit /b 1
            )
            goto :commit
        )
    )
)

:commit
REM 添加所有文件
echo 添加文件到 Git...
git add .

REM 创建提交
echo 创建初始提交...
git commit -m "feat: initial release v0.1.0" -m "" -m "- IrisNote: Smart text editor with auto-detection" -m "- Markdown and SVG preview support" -m "- Syntax highlighting for 15+ languages" -m "- Intelligent file naming suggestions" -m "- Windows file association" -m "- Apache 2.0 license" -m "" -m "🤖 Generated with CodeArts"

if errorlevel 1 (
    echo × 提交失败
    pause
    exit /b 1
)

echo.
echo ✓ 提交成功！
echo.

REM 添加远程仓库
echo 添加远程仓库...
git remote | findstr "origin" >nul
if errorlevel 1 (
    git remote add origin https://github.com/itszzl-sudo/irisnote.git
) else (
    git remote set-url origin https://github.com/itszzl-sudo/irisnote.git
)

echo ✓ 远程仓库已配置: https://github.com/itszzl-sudo/irisnote.git
echo.

REM 推送到 GitHub
set /p PUSH="是否推送到 GitHub? (y/n): "
if /i "%PUSH%"=="y" (
    echo 推送到 GitHub...
    git push -u origin %BRANCH%
    
    if errorlevel 1 (
        echo × 推送失败，请检查网络连接或认证
        pause
        exit /b 1
    )
    
    echo.
    echo ✓ 推送成功！
    echo.
    
    REM 创建标签
    set /p TAG="是否创建 v0.1.0 标签? (y/n): "
    if /i "%TAG%"=="y" (
        git tag -a v0.1.0 -m "Release v0.1.0"
        git push origin v0.1.0
        
        if errorlevel 1 (
            echo × 标签推送失败
        ) else (
            echo.
            echo ✓ 标签创建成功！
            echo.
            echo ================================
            echo   发布完成！
            echo ================================
            echo.
            echo 下一步:
            echo 1. 访问 https://github.com/itszzl-sudo/irisnote
            echo 2. 创建 Release ^(使用标签 v0.1.0^)
            echo 3. 上传构建的可执行文件
            echo 4. 添加 Release 说明
            echo.
        )
    )
)

pause
