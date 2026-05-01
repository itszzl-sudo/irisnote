@echo off
chcp 65001 >nul
echo ====================================
echo   IrisNote AI 增强版构建
echo ====================================
echo.
echo 构建带 LLM 的完整版本
echo 大小: ~25 MB (额外依赖)
echo 启动: 需 LLM 服务
echo.

REM 检查 Rust
where cargo >nul 2>&1
if %ERRORLEVEL% NEQ 0 (
    echo × 未安装 Rust
    echo 请访问 https://rustup.rs 安装
    pause
    exit /b 1
)

echo [步骤 1/4] 检查模型文件...
set MODEL_PATH=C:\Users\a\Downloads\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf
if not exist "%MODEL_PATH%" (
    echo ⚠️ 模型文件不存在
    echo.
    echo 请下载模型:
    echo https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF
    echo.
    echo 继续构建但不包含模型检查...
)

echo.
echo [步骤 2/4] 清理旧的构建产物...
cargo clean

echo.
echo [步骤 3/4] 编译 AI 增强版...
cargo build --release --features llm

if %ERRORLEVEL% NEQ 0 (
    echo × 构建失败
    pause
    exit /b 1
)

echo.
echo [步骤 4/4] 创建发布包...

REM 获取版本号
for /f "tokens=2 delims== " %%i in ('findstr "^version" Cargo.toml') do set VERSION=%%i
set VERSION=%VERSION:"=%

REM 创建 ZIP 包
set ZIP_NAME=irisnote-v%VERSION%-windows-x64-ai.zip
powershell -Command "Compress-Archive -Path 'target\release\irisnote.exe', 'README.md', 'README_CN.md', 'LICENSE', 'NOTICE', 'LLM_SETUP.md', 'start-llm-server.bat' -DestinationPath '%ZIP_NAME%' -Force"

if exist "%ZIP_NAME%" (
    echo.
    echo ✅ AI 增强版构建成功！
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
    echo   ✅ AI 智能命名
    echo.
    echo 下一步:
    echo   1. 安装 llama.cpp (参考 LLM_SETUP.md)
    echo   2. 运行 start-llm-server.bat 启动 LLM 服务
    echo   3. 运行 irisnote.exe
    echo.
) else (
    echo × 创建发布包失败
)

pause
