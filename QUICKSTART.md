# 快速入门指南

## 安装要求

1. **Rust 工具链**: 从 https://rustup.rs/ 安装
2. **Visual Studio Build Tools**: 用于编译 Windows 应用
   ```bash
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

## 快速开始

### 方式 1: Debug 版本（快速构建）

```bash
cd smart-text-editor
cargo build
.\target\debug\smart-text-editor.exe
```

构建时间：5-10 分钟

### 方式 2: Release 版本（优化性能）

```bash
cd smart-text-editor
cargo build --release
.\target\release\smart-text-editor.exe
```

构建时间：10-30 分钟

## 功能演示

### 1. 创建 Markdown 文档

1. 点击"文件" -> "新建"
2. 输入 Markdown 内容：
   ```markdown
   # 我的文档
   这是**加粗**和*斜体*文本。
   ```
3. 点击"视图" -> "显示预览"
4. 保存时会自动推荐 `.md` 后缀

### 2. 编辑 SVG 图形

1. 打开 `examples/demo.svg`
2. 编辑器会自动渲染 SVG
3. 修改颜色、形状等属性
4. 实时查看渲染效果

### 3. 代码编辑

1. 打开 `examples/demo.rs`（Rust）
2. 或 `examples/demo.py`（Python）
3. 启用语法高亮视图
4. 查看关键字、字符串高亮

### 4. 智能文件名推荐

当保存新文件时，编辑器会根据内容推荐文件名：

| 内容特征 | 推荐后缀 |
|---------|---------|
| `fn main()` | `.rs` |
| `def main():` | `.py` |
| `function main()` | `.js` |
| `# 标题` | `.md` |
| `<svg>` | `.svg` |
| `{ "key": "value" }` | `.json` |

### 5. 文件关联（Windows）

在"工具"菜单中：
- **关联所有文件类型**: 一键关联所有支持的格式
- **选择性关联**: 只关联特定格式

关联后，双击文件即可用此编辑器打开。

## 示例文件

项目包含以下示例：

- `examples/demo.md` - Markdown 文档
- `examples/demo.svg` - SVG 图形
- `examples/demo.json` - JSON 数据
- `examples/demo.rs` - Rust 代码
- `examples/demo.py` - Python 代码

## 快捷键

- `Ctrl+N`: 新建文件
- `Ctrl+O`: 打开文件
- `Ctrl+S`: 保存文件
- `Ctrl+Shift+S`: 另存为
- `Ctrl+P`: 切换预览

## 性能优化

### Debug vs Release

- **Debug**: 包含调试信息，体积大，速度慢
- **Release**: 优化编译，体积小，速度快

建议：
- 开发测试：使用 Debug
- 正式使用：使用 Release

### 减小体积

Release 版本已启用：
- LTO（链接时优化）
- 优化级别 3

## 故障排除

### 编译错误

1. **链接器错误**
   ```bash
   # 安装 Visual Studio Build Tools
   winget install Microsoft.VisualStudio.2022.BuildTools
   ```

2. **依赖下载慢**
   ```bash
   # 使用清华镜像
   # 在 %USERPROFILE%\.cargo\config 添加：
   [source.crates-io]
   replace-with = 'tuna'
   
   [source.tuna]
   registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
   ```

### 运行错误

1. **缺少 DLL**
   ```bash
   winget install Microsoft.VCRedist.2015+.x64
   ```

2. **图标不显示**
   - 确认 `C:\Users\a\Pictures\iris.png` 存在
   - 重新构建项目

## Windows 自签名

### 创建证书

```powershell
$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject "CN=Smart Text Editor" -CertStoreLocation Cert:\CurrentUser\My
```

### 签名可执行文件

```powershell
$exe = ".\target\release\smart-text-editor.exe"
Set-AuthenticodeSignature -FilePath $exe -Certificate $cert
```

### 导出证书

```powershell
$certPath = "$env:USERPROFILE\smart-text-editor.cer"
Export-Certificate -Cert $cert -FilePath $certPath
```

### 信任证书

双击导出的 `.cer` 文件，安装到"受信任的根证书颁发机构"。

## 下一步

1. 尝试打开各种文件类型
2. 测试文件关联功能
3. 探索不同的预览模式
4. 自定义编辑器配置

## 获取帮助

遇到问题？查看：
- `README.md` - 项目说明
- `BUILD.md` - 详细构建指南
- GitHub Issues - 问题反馈
