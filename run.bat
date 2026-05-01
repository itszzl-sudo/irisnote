@echo off
echo Smart Text Editor - 快速启动
echo.

REM 检查 Debug 版本
if exist target\debug\smart-text-editor.exe (
    echo 启动 Debug 版本...
    target\debug\smart-text-editor.exe
    goto :end
)

REM 检查 Release 版本
if exist target\release\smart-text-editor.exe (
    echo 启动 Release 版本...
    target\release\smart-text-editor.exe
    goto :end
)

echo 未找到可执行文件，开始构建 Debug 版本...
echo.
cargo build

if %ERRORLEVEL% EQU 0 (
    echo.
    echo 构建完成，启动编辑器...
    target\debug\smart-text-editor.exe
) else (
    echo 构建失败！
    pause
)

:end
