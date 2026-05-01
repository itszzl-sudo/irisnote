# IrisNote 标准版构建脚本

Write-Host "====================================" -ForegroundColor Cyan
Write-Host "  IrisNote 标准版构建" -ForegroundColor Cyan
Write-Host "====================================" -ForegroundColor Cyan
Write-Host ""

Write-Host "构建无外部依赖的轻量版本" -ForegroundColor Yellow
Write-Host "大小: ~15 MB" -ForegroundColor Gray
Write-Host "启动: 快速" -ForegroundColor Gray
Write-Host ""

# 检查 Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "× 未安装 Rust" -ForegroundColor Red
    Write-Host "请访问 https://rustup.rs 安装" -ForegroundColor Yellow
    Read-Host "按回车键退出"
    exit 1
}

Write-Host "[步骤 1/3] 清理旧的构建产物..." -ForegroundColor Green
cargo clean

Write-Host ""
Write-Host "[步骤 2/3] 编译标准版..." -ForegroundColor Green
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "× 构建失败" -ForegroundColor Red
    Read-Host "按回车键退出"
    exit 1
}

Write-Host ""
Write-Host "[步骤 3/3] 创建发布包..." -ForegroundColor Green

# 获取版本号
$cargoContent = Get-Content Cargo.toml -Raw
if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
    $version = $matches[1]
} else {
    $version = "0.1.0"
}

$zipName = "irisnote-v$version-windows-x64-standard.zip"

# 创建 ZIP 包
$filesToZip = @(
    "target\release\irisnote.exe",
    "README.md",
    "README_CN.md",
    "LICENSE",
    "NOTICE"
)

if (Test-Path $zipName) {
    Remove-Item $zipName -Force
}

Compress-Archive -Path $filesToZip -DestinationPath $zipName -CompressionLevel Optimal

if (Test-Path $zipName) {
    $fileSize = (Get-Item $zipName).Length
    $fileSizeMB = [math]::Round($fileSize / 1MB, 2)
    
    Write-Host ""
    Write-Host "✅ 标准版构建成功！" -ForegroundColor Green
    Write-Host ""
    Write-Host "可执行文件: target\release\irisnote.exe" -ForegroundColor White
    Write-Host "发布包: $zipName" -ForegroundColor White
    Write-Host ""
    Write-Host "大小: $fileSizeMB MB" -ForegroundColor Gray
    Write-Host ""
    Write-Host "功能:" -ForegroundColor Yellow
    Write-Host "  ✅ 文件类型检测" -ForegroundColor Green
    Write-Host "  ✅ 语法高亮" -ForegroundColor Green
    Write-Host "  ✅ Markdown 预览" -ForegroundColor Green
    Write-Host "  ✅ SVG 渲染" -ForegroundColor Green
    Write-Host "  ✅ Windows 文件关联" -ForegroundColor Green
    Write-Host "  ✅ BitNet 智能分析" -ForegroundColor Green
    Write-Host ""
} else {
    Write-Host "× 创建发布包失败" -ForegroundColor Red
}

Read-Host "按回车键退出"
