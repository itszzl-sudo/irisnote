# IrisNote

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-green.svg)]()

一个用 Rust 编写的智能文本编辑器，支持多种文件类型的自动检测、预览和编辑。

## ✨ 功能特性

### 🎯 智能文件检测
- **自动类型识别**: 根据文件内容和扩展名智能识别文件类型
- **智能命名推荐**: 保存时根据内容自动推荐合适的文件名和扩展名
- 支持识别函数名、类名、标题等关键信息用于命名

### 👁️ 多模式预览
- **Markdown**: 实时渲染预览，支持标题、列表、代码块、引用等
- **SVG**: 图片实时渲染，可直接查看图形效果
- **代码文件**: 语法高亮，支持关键字、字符串、注释着色

### 📁 文件管理
- **最近文件记录**: 自动记住最近打开的文件位置
- **快速访问**: 下拉菜单快速选择历史文件
- **配置持久化**: 自动保存用户偏好设置

### 🔧 Windows 集成
- **文件关联**: 一键关联所有支持的文件类型
- **选择性关联**: 按需选择特定格式进行关联
- **自签名支持**: 构建时自动创建和签名可执行文件
- **自定义图标**: 支持自定义应用程序图标

## 📦 支持的文件类型

| 类型 | 扩展名 | 特殊功能 |
|------|--------|----------|
| 文本 | `.txt` | - |
| Markdown | `.md` | 实时预览渲染 |
| Rust | `.rs` | 语法高亮 |
| Python | `.py` | 语法高亮 |
| JavaScript | `.js` | 语法高亮 |
| TypeScript | `.ts` | 语法高亮 |
| HTML | `.html`, `.htm` | 语法高亮 |
| CSS | `.css` | 语法高亮 |
| JSON | `.json` | 语法高亮 |
| XML | `.xml` | 语法高亮 |
| YAML | `.yaml`, `.yml` | 语法高亮 |
| TOML | `.toml` | 语法高亮 |
| SVG | `.svg` | 图片渲染预览 |
| C | `.c` | 语法高亮 |
| C++ | `.cpp`, `.cc`, `.cxx` | 语法高亮 |
| Java | `.java` | 语法高亮 |
| Go | `.go` | 语法高亮 |

## 🚀 快速开始

### 系统要求

- Rust 1.70 或更高版本
- Windows: Visual Studio Build Tools
- Linux: build-essential
- macOS: Xcode Command Line Tools

### 安装 Rust

```bash
# 访问 https://rustup.rs/ 
# 或使用以下命令
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 构建项目

#### Debug 版本（快速构建，用于测试）

```bash
cd smart-text-editor
cargo build
./target/debug/smart-text-editor  # Linux/macOS
.\target\debug\smart-text-editor.exe  # Windows
```

#### Release 版本（优化编译，用于生产）

```bash
cd smart-text-editor
cargo build --release
./target/release/smart-text-editor  # Linux/macOS
.\target\release\smart-text-editor.exe  # Windows
```

### Windows 自签名

运行提供的批处理文件：

```bash
.\build-and-sign.bat
```

这将自动：
1. 编译 Release 版本
2. 创建自签名证书
3. 对可执行文件进行签名

### 一键启动

```bash
.\run.bat  # Windows
```

## 📖 使用指南

### 基本操作

1. **新建文件**: `文件` → `新建`
2. **打开文件**: `文件` → `打开` 或从最近文件列表选择
3. **保存文件**: `文件` → `保存`（自动使用智能推荐的文件名）
4. **另存为**: `文件` → `另存为`（可自定义文件名）

### 预览模式

通过 `视图` 菜单切换：
- **编辑器**: 纯文本编辑模式
- **显示预览**: 开启分屏预览
- **Markdown 预览**: 渲染 Markdown 格式
- **图片预览**: 渲染 SVG 图形
- **语法高亮**: 代码语法着色

### 文件关联 (Windows)

在 `工具` 菜单中：
- **关联所有支持的文件类型**: 一键关联所有格式
- **选择性关联**: 选择特定文件类型进行关联

关联后，双击文件即可用本编辑器打开。

## 🎨 智能文件名推荐示例

| 文件内容特征 | 推荐文件名 | 推荐后缀 |
|-------------|-----------|---------|
| `fn main() { ... }` | `main.rs` | `.rs` |
| `def process():` | `process.py` | `.py` |
| `class MyClass:` | `MyClass.py` | `.py` |
| `function init()` | `init.js` | `.js` |
| `# 标题` | `标题.md` | `.md` |
| `<svg>...</svg>` | `untitled.svg` | `.svg` |
| `{"name": "value"}` | `data.json` | `.json` |
| `[package]` | `config.toml` | `.toml` |

## 📂 项目结构

```
smart-text-editor/
├── Cargo.toml              # 依赖配置
├── build.rs                # 构建脚本（设置图标）
├── src/
│   ├── main.rs             # 主程序入口
│   ├── file_type.rs        # 文件类型检测与推荐
│   ├── preview.rs          # 预览渲染（Markdown/SVG/语法高亮）
│   ├── config.rs           # 配置管理
│   └── file_association.rs # Windows 文件关联
├── examples/               # 示例文件
│   ├── demo.md             # Markdown 示例
│   ├── demo.svg            # SVG 示例
│   ├── demo.json           # JSON 示例
│   ├── demo.rs             # Rust 代码示例
│   └── demo.py             # Python 代码示例
├── build-and-sign.bat      # Windows 构建和签名脚本
├── run.bat                 # 快速启动脚本
├── LICENSE                 # Apache 2.0 许可证
├── README.md               # 英文文档
├── README_CN.md            # 中文文档（本文件）
├── BUILD.md                # 详细构建指南
└── QUICKSTART.md           # 快速入门指南
```

## ⚙️ 配置

配置文件位置：
- Windows: `%APPDATA%\smart-text-editor\config.json`
- Linux: `~/.config/smart-text-editor/config.json`
- macOS: `~/Library/Application Support/smart-text-editor/config.json`

配置内容：
```json
{
  "recent_paths": [
    "/path/to/recent/file1.md",
    "/path/to/recent/file2.rs"
  ],
  "default_save_dir": "/path/to/default",
  "auto_detect_type": true
}
```

## 🔨 开发

### 构建 Debug 版本

```bash
cargo build
```

### 构建 Release 版本

```bash
cargo build --release
```

### 运行测试

```bash
cargo test
```

### 代码检查

```bash
cargo clippy
cargo fmt --check
```

## 🐛 故障排除

### 构建错误

**链接器错误（Windows）**:
```bash
winget install Microsoft.VisualStudio.2022.BuildTools
```

**依赖下载慢（中国大陆）**:
```bash
# 在 %USERPROFILE%\.cargo\config 添加：
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### 运行错误

**缺少 DLL（Windows）**:
```bash
winget install Microsoft.VCRedist.2015+.x64
```

**图标不显示**:
- 确认图标文件路径正确
- 重新构建项目

## 🤝 贡献

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 贡献指南

- 遵循 Rust 代码规范
- 添加必要的测试
- 更新相关文档
- 确保所有测试通过

## 📄 许可证

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

本项目使用以下开源库：

- [egui](https://github.com/emilk/egui) - 即时模式 GUI 框架
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown 解析器
- [resvg](https://github.com/RazrFalcon/resvg) - SVG 渲染库
- [syntect](https://github.com/trishume/syntect) - 语法高亮库

## 📮 联系方式

- 问题反馈: [GitHub Issues](https://github.com/itszzl-sudo/irisnote/issues)
- 功能建议: [GitHub Discussions](https://github.com/itszzl-sudo/irisnote/discussions)

## 🗺️ 路线图

- [ ] 支持更多文件类型
- [ ] 主题定制功能
- [ ] 插件系统
- [ ] 协作编辑功能
- [ ] 云同步支持

---

**⭐ 如果这个项目对你有帮助，请给一个 Star！**
