# IrisNote 统一构建脚本

param(
    [Parameter(Position=0)]
    [ValidateSet("standard", "bitnet", "all", "clean", "help")]
    [string]$Command = "standard"
)

function Show-Help {
    Write-Host @"
IrisNote 构建脚本

用法:
    .\build.ps1 [命令]

命令:
    standard    构建标准版（默认）
    bitnet      构建 BitNet 增强版
    all         构建所有版本
    clean       清理构建产物
    help        显示帮助信息

示例:
    .\build.ps1              # 构建标准版
    .\build.ps1 standard     # 构建标准版
    .\build.ps1 bitnet       # 构建 BitNet 增强版
    .\build.ps1 all          # 构建所有版本
    .\build.ps1 clean        # 清理构建产物

版本说明:
    标准版: ~15 MB, 基础功能
    BitNet 版: ~20 MB, 增强智能分析

"@
}

function Build-Standard {
    Write-Host "====================================" -ForegroundColor Cyan
    Write-Host "  构建标准版" -ForegroundColor Cyan
    Write-Host "====================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "编译中..." -ForegroundColor Green
    cargo build --release
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "× 构建失败" -ForegroundColor Red
        return $false
    }
    
    $exePath = "target\release\irisnote.exe"
    if (Test-Path $exePath) {
        $size = (Get-Item $exePath).Length / 1MB
        Write-Host "✅ 标准版构建成功" -ForegroundColor Green
        Write-Host "   大小: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
        return $true
    }
    
    return $false
}

function Build-BitNet {
    Write-Host "====================================" -ForegroundColor Cyan
    Write-Host "  构建 BitNet 增强版" -ForegroundColor Cyan
    Write-Host "====================================" -ForegroundColor Cyan
    Write-Host ""
    
    Write-Host "编译中..." -ForegroundColor Green
    cargo build --release --features bitnet
    
    if ($LASTEXITCODE -ne 0) {
        Write-Host "× 构建失败" -ForegroundColor Red
        return $false
    }
    
    $exePath = "target\release\irisnote.exe"
    if (Test-Path $exePath) {
        $size = (Get-Item $exePath).Length / 1MB
        Write-Host "✅ BitNet 增强版构建成功" -ForegroundColor Green
        Write-Host "   大小: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
        return $true
    }
    
    return $false
}

function Clean-Build {
    Write-Host "清理构建产物..." -ForegroundColor Yellow
    cargo clean
    Write-Host "✅ 清理完成" -ForegroundColor Green
}

# 主逻辑
switch ($Command) {
    "help" {
        Show-Help
    }
    
    "standard" {
        Build-Standard
    }
    
    "bitnet" {
        Build-BitNet
    }
    
    "all" {
        Write-Host "构建所有版本..." -ForegroundColor Cyan
        Write-Host ""
        
        $success = $true
        
        # 标准版
        if (-not (Build-Standard)) {
            $success = $false
        }
        
        Write-Host ""
        
        # BitNet 版
        if (-not (Build-BitNet)) {
            $success = $false
        }
        
        Write-Host ""
        if ($success) {
            Write-Host "✅ 所有版本构建成功" -ForegroundColor Green
        } else {
            Write-Host "× 部分版本构建失败" -ForegroundColor Red
        }
    }
    
    "clean" {
        Clean-Build
    }
}
