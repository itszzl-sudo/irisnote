# IrisNote 统一构建脚本

param(
    [Parameter(Position=0)]
    [ValidateSet("release", "debug", "clean", "help")]
    [string]$Command = "release"
)

function Show-Help {
    Write-Host @"
IrisNote 构建脚本 (BitNet 增强版)

用法:
    .\build.ps1 [命令]

命令:
    release     构建发布版（默认，包含 BitNet）
    debug       构建调试版（包含 BitNet）
    clean       清理构建产物
    help        显示帮助信息

示例:
    .\build.ps1              # 构建发布版
    .\build.ps1 release      # 构建发布版
    .\build.ps1 debug        # 构建调试版
    .\build.ps1 clean        # 清理构建产物

特性:
    BitNet 智能分析（默认启用）
    - 文件名建议
    - 内容摘要生成
    - 关键词提取

"@
}

function Build-Release {
    Write-Host "构建 IrisNote 发布版（BitNet 增强版）..." -ForegroundColor Cyan
    
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ 构建成功" -ForegroundColor Green
        Write-Host "输出: target\release\irisnote.exe" -ForegroundColor Yellow
    } else {
        Write-Host "❌ 构建失败" -ForegroundColor Red
        exit 1
    }
}

function Build-Debug {
    Write-Host "构建 IrisNote 调试版（BitNet 增强版）..." -ForegroundColor Cyan
    
    cargo build
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ 构建成功" -ForegroundColor Green
        Write-Host "输出: target\debug\irisnote.exe" -ForegroundColor Yellow
    } else {
        Write-Host "❌ 构建失败" -ForegroundColor Red
        exit 1
    }
}

function Clean-Build {
    Write-Host "清理构建产物..." -ForegroundColor Yellow
    cargo clean
    Write-Host "✅ 清理完成" -ForegroundColor Green
}

switch ($Command) {
    "release" { Build-Release }
    "debug" { Build-Debug }
    "clean" { Clean-Build }
    "help" { Show-Help }
    default { Build-Release }
}
