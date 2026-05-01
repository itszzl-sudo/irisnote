@echo off
chcp 65001 >nul
echo ====================================
echo   启动 LLM 服务
echo ====================================
echo.

REM 配置
set MODEL_PATH=C:\Users\a\Downloads\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf
set SERVER_PORT=8080
set CTX_SIZE=2048
set N_GPU_LAYERS=0

REM 检查模型文件
if not exist "%MODEL_PATH%" (
    echo × 模型文件不存在: %MODEL_PATH%
    echo.
    echo 请确保已下载 Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf
    echo 下载地址: https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF
    pause
    exit /b 1
)

echo 模型路径: %MODEL_PATH%
echo 服务端口: %SERVER_PORT%
echo 上下文大小: %CTX_SIZE%
echo.

REM 检查 llama-server 是否存在
where llama-server >nul 2>&1
if %ERRORLEVEL% EQU 0 (
    echo 使用系统 llama-server
    goto :start_server
)

REM 检查本地 llama.cpp
if exist "llama.cpp\llama-server.exe" (
    echo 使用本地 llama-server
    set PATH=%CD%\llama.cpp;%PATH%
    goto :start_server
)

echo × 未找到 llama-server
echo.
echo 请安装 llama.cpp:
echo   方式 1: 使用 release 版本
echo     访问 https://github.com/ggerganov/llama.cpp/releases
echo     下载 Windows 版本并解压到当前目录
echo.
echo   方式 2: 从源码编译
echo     git clone https://github.com/ggerganov/llama.cpp
echo     cd llama.cpp
echo     cmake -B build
echo     cmake --build build --config Release
echo.
pause
exit /b 1

:start_server
echo.
echo 启动 LLM 服务器...
echo 服务地址: http://localhost:%SERVER_PORT%
echo.
echo 按 Ctrl+C 停止服务
echo.

llama-server ^
    -m "%MODEL_PATH%" ^
    --port %SERVER_PORT% ^
    --ctx-size %CTX_SIZE% ^
    --n-gpu-layers %N_GPU_LAYERS% ^
    --threads 4 ^
    --batch-size 32 ^
    --cont-batching ^
    --metrics

pause
