# 构建说明

由于项目依赖较多，首次构建可能需要较长时间（10-30分钟，取决于网络速度）。

## 分步构建

### 1. 安装 Rust 工具链（如果还没有）

```bash
# 访问 https://rustup.rs/ 下载安装
# 或者使用以下命令
winget install Rustlang.Rustup
```

### 2. 构建项目

```bash
cd smart-text-editor

# Debug 版本（构建快，但性能较低）
cargo build

# Release 版本（构建慢，但性能高）
cargo build --release
```

### 3. Windows 自签名

构建完成后，运行 PowerShell 脚本进行自签名：

```powershell
# 创建自签名证书
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=Smart Text Editor" -CertStoreLocation Cert:\CurrentUser\My

# 导出证书
$certPath = "$env:USERPROFILE\smart-text-editor.cer"
Export-Certificate -Cert $cert -FilePath $certPath
Write-Host "证书已导出到: $certPath"

# 签名可执行文件
$exePath = ".\target\release\smart-text-editor.exe"
if (Test-Path $exePath) {
    Set-AuthenticodeSignature -FilePath $exePath -Certificate $cert
    Write-Host "签名成功！"
} else {
    Write-Host "可执行文件不存在，请先构建项目"
}
```

### 4. 运行程序

```bash
# Debug 版本
.\target\debug\smart-text-editor.exe

# Release 版本
.\target\release\smart-text-editor.exe
```

## 快速测试

如果只想快速测试，可以先构建 Debug 版本：

```bash
cargo build
```

这将显著减少构建时间。

## 故障排除

### 构建错误

1. **链接器错误**: 确保安装了 Visual Studio Build Tools
   ```bash
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

2. **依赖下载失败**: 使用国内镜像
   ```bash
   # 在 %USERPROFILE%\.cargo\config 中添加：
   [source.crates-io]
   replace-with = 'tuna'
   
   [source.tuna]
   registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
   ```

### 运行时错误

1. **缺少 DLL**: 安装 Visual C++ Redistributable
   ```bash
   winget install Microsoft.VCRedist.2015+.x64
   ```

## 项目结构

```
smart-text-editor/
├── Cargo.toml          # 依赖配置
├── build.rs            # 构建脚本（设置图标）
├── src/
│   ├── main.rs         # 主程序入口
│   ├── file_type.rs    # 文件类型检测
│   ├── preview.rs      # 预览渲染
│   ├── config.rs       # 配置管理
│   └── file_association.rs  # Windows 文件关联
├── build-and-sign.bat  # 自动构建和签名脚本
└── README.md           # 使用说明
```

## 主要依赖

- `eframe` / `egui`: GUI 框架
- `pulldown-cmark`: Markdown 解析
- `resvg` / `usvg`: SVG 渲染
- `syntect`: 语法高亮
- `serde` / `serde_json`: 序列化
- `winreg`: Windows 注册表操作
- `rfd`: 文件对话框
