# IrisNote 快速启动脚本

Write-Host "====================================" -ForegroundColor Cyan
Write-Host "  IrisNote 快速启动" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Release 版本
$releaseExe = "target\release\irisnote.exe"
$debugExe = "target\debug\irisnote.exe"

if (Test-Path $releaseExe) {
    Write-Host "启动 Release 版本..." -ForegroundColor Green
    & $releaseExe
} elseif (Test-Path $debugExe) {
    Write-Host "启动 Debug 版本..." -ForegroundColor Yellow
    & $debugExe
} else {
    Write-Host "未找到可执行文件，开始构建..." -ForegroundColor Yellow
    Write-Host ""
    
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host ""
        Write-Host "构建完成，启动编辑器..." -ForegroundColor Green
        & $releaseExe
    } else {
        Write-Host "× 构建失败" -ForegroundColor Red
        Read-Host "按回车键退出"
    }
}
