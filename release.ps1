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

# 获取版本号
$cargoContent = Get-Content Cargo.toml -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
    $version = $matches[1]
} else {
    $version = "0.1.0"
}

Write-Host "版本号: $version" -ForegroundColor Yellow
Write-Host ""

function Create-Package {
    param(
        [string]$Name,
        [string[]]$Files
    )
    
    if (Test-Path $Name) {
        Remove-Item $Name -Force
    }
    
    Compress-Archive -Path $Files -DestinationPath $Name -CompressionLevel Optimal
    
    if (Test-Path $Name) {
        $size = (Get-Item $Name).Length / 1MB
        Write-Host "✅ 已创建: $name" -ForegroundColor Green
        Write-Host "   大小: $([math]::Round($size, 2)) MB" -ForegroundColor Gray
        return $true
    }
    
    Write-Host "× 创建失败: $name" -ForegroundColor Red
    return $false
}

# 标准版发布包
if ($Version -eq "standard" -or $Version -eq "all") {
    Write-Host "创建标准版发布包..." -ForegroundColor Cyan
    
    $zipName = "irisnote-v$version-windows-x64-standard.zip"
    $files = @(
        "target\release\irisnote.exe",
        "README.md",
        "README_CN.md",
        "LICENSE",
        "NOTICE",
        "BUILD_VARIANTS.md"
    )
    
    Create-Package -Name $zipName -Files $files
    Write-Host ""
}

# BitNet 版发布包
if ($Version -eq "bitnet" -or $Version -eq "all") {
    Write-Host "创建 BitNet 版发布包..." -ForegroundColor Cyan
    
    # 先构建 BitNet 版本
    Write-Host "编译 BitNet 版本..." -ForegroundColor Gray
    cargo build --release --features bitnet
    
    if ($LASTEXITCODE -eq 0) {
        $zipName = "irisnote-v$version-windows-x64-bitnet.zip"
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
Get-ChildItem -Filter "irisnote-v$version-*.zip" | ForEach-Object {
    $size = $_.Length / 1MB
    Write-Host "  $($_.Name) - $([math]::Round($size, 2)) MB" -ForegroundColor White
}

Write-Host ""
Write-Host "下一步:" -ForegroundColor Cyan
Write-Host "  1. 访问 GitHub Releases" -ForegroundColor Gray
Write-Host "  2. 创建新 Release" -ForegroundColor Gray
Write-Host "  3. 上传发布包" -ForegroundColor Gray
