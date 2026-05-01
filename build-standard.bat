@echo off
chcp 65001 >nul
echo ====================================
echo   IrisNote 标准版构建
echo ====================================
echo.
echo 构建无 LLM 依赖的轻量版本
echo 大小: ~15 MB
echo 启动: 快速
echo.

REM 检查 Rust
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo × 未安装 Rust
    echo 请访问 https://rustup.rs 安装
    pause
    exit /b 1
)

echo [步骤 1/3] 清理旧的构建产物...
cargo clean

echo.
echo [步骤 2/3] 编译标准版...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo × 构建失败
    pause
    exit /b 1
)

echo.
echo [步骤 3/3] 创建发布包...

REM 获取版本号
for /f "tokens=2 delims== " %%i in ('findstr "^version" Cargo.toml') do set VERSION=%%i
set VERSION=%VERSION:"=%

REM 创建 ZIP 包
set ZIP_NAME=irisnote-v%VERSION%-windows-x64-standard.zip
powershell -Command "Compress-Archive -Path 'target\release\irisnote.exe', 'README.md', 'README_CN.md', 'LICENSE', 'NOTICE' -DestinationPath '%ZIP_NAME%' -Force"

if exist "%ZIP_NAME%" (
    echo.
    echo ✅ 标准版构建成功！
    echo.
    echo 可执行文件: target\release\irisnote.exe
    echo 发布包: %ZIP_NAME%
    echo.
    for %%I in (%ZIP_NAME%) do echo 大小: %%~zI 字节
    echo.
    echo 功能:
    echo   ✅ 文件类型检测
    echo   ✅ 语法高亮
    echo   ✅ Markdown 预览
    echo   ✅ SVG 渲染
    echo   ✅ Windows 文件关联
    echo   ❌ AI 智能命名 (需 AI 增强版)
    echo.
) else (
    echo × 创建发布包失败
)

pause
