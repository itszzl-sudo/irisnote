# IrisNote 发布脚本

param(
    [Parameter(Position=0)]
    [ValidateSet("standard", "bitnet", "all")]
    [string]$Version = "all"
)

Write-Host "====================================" -ForegroundColor Cyan
Write-Host "  IrisNote 发布打包" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

# 检查 Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "× 未安装 Rust" -ForegroundColor Red
    Write-Host "请访问 https://rustup.rs 安装" -ForegroundColor Yellow
    Read-Host "按回车键退出"
    exit 1
}

# 获取版本号
$cargoContent = Get-Content Cargo.toml -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
    $ver = $matches[1]
} else {
    $ver = "0.1.0"
}

Write-Host "版本号: $ver" -ForegroundColor Yellow
Write-Host ""

function Create-Package {
    param(
        [string]$Name,
        [string[]]$Files
    )
    
    # 检查文件是否存在
    $missingFiles = @()
    foreach ($file in $Files) {
        if (-not (Test-Path $file)) {
            $missingFiles += $file
        }
    }
    
    if ($missingFiles.Count -gt 0) {
        Write-Host "× 文件不存在:" -ForegroundColor Red
        foreach ($file in $missingFiles) {
            Write-Host "  $file" -ForegroundColor Red
        }
        return $false
    }
    
    if (Test-Path $Name) {
        Remove-Item $Name -Force
    }
    
    try {
        Compress-Archive -Path $Files -DestinationPath $Name -CompressionLevel Optimal -ErrorAction Stop
        
        if (Test-Path $Name) {
            $size = (Get-Item $Name).Length / 1MB
            Write-Host "✅ 已创建: $Name" -ForegroundColor Green
            Write-Host "   大小: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
            return $true
        }
    }
    catch {
        Write-Host "× 创建失败: $Name" -ForegroundColor Red
        Write-Host "  错误: $($_.Exception.Message)" -ForegroundColor Red
        return $false
    }
    
    return $false
}

# 标准版发布包
if ($Version -eq "standard" -or $Version -eq "all") {
    Write-Host "创建标准版发布包..." -ForegroundColor Cyan
    
    # 先构建标准版
    Write-Host "编译标准版..." -ForegroundColor Gray
    cargo build --release
    
    if ($LASTEXITCODE -eq 0) {
        $zipName = "irisnote-v$ver-windows-x64-standard.zip"
        $files = @(
            "target\release\irisnote.exe",
            "README.md",
            "README_CN.md",
            "LICENSE",
            "NOTICE",
            "BUILD_VARIANTS.md"
        )
        
        Create-Package -Name $zipName -Files $files
    } else {
        Write-Host "× 标准版编译失败" -ForegroundColor Red
    }
    
    Write-Host ""
}

# BitNet 版发布包
if ($Version -eq "bitnet" -or $Version -eq "all") {
    Write-Host "创建 BitNet 版发布包..." -ForegroundColor Cyan
    
    # 先构建 BitNet 版本
    Write-Host "编译 BitNet 版本..." -ForegroundColor Gray
    cargo build --release --features bitnet
    
    if ($LASTEXITCODE -eq 0) {
        $zipName = "irisnote-v$ver-windows-x64-bitnet.zip"
        $files = @(
            "target\release\irisnote.exe",
            "README.md",
            "README_CN.md",
            "LICENSE",
            "NOTICE",
            "BITNET_GUIDE.md",
            "BUILD_VARIANTS.md"
        )
        
        Create-Package -Name $zipName -Files $files
    } else {
        Write-Host "× BitNet 版本编译失败" -ForegroundColor Red
    }
    
    Write-Host ""
}

Write-Host "====================================" -ForegroundColor Cyan
Write-Host "  打包完成" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan

# 列出所有发布包
Write-Host ""
Write-Host "发布包列表:" -ForegroundColor Yellow
$zips = Get-ChildItem -Filter "irisnote-v$ver-*.zip" -ErrorAction SilentlyContinue

if ($zips) {
    $zips | ForEach-Object {
        $size = $_.Length / 1MB
        Write-Host "  $($_.Name) - $([math]::Round($size, 2)) MB" -ForegroundColor White
    }
} else {
    Write-Host "  无发布包" -ForegroundColor Gray
}

Write-Host ""
Write-Host "下一步:" -ForegroundColor Cyan
Write-Host "  1. 访问 GitHub Releases" -ForegroundColor Gray
Write-Host "  2. 创建新 Release" -ForegroundColor Gray
Write-Host "  3. 上传发布包" -ForegroundColor Gray
Write-Host ""
Write-Host "GitHub: https://github.com/itszzl-sudo/irisnote/releases" -ForegroundColor Blue
