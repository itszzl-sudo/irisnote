# 更新日志

本项目遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

所有重要的更改都将记录在此文件中。

## [未发布]

### 计划新增
- 支持更多文件类型
- 主题定制功能
- 插件系统
- 协作编辑功能
- 云同步支持

## [0.1.0] - 2024-05-01

### 新增

#### 核心功能
- ✨ 智能文件类型自动检测
- ✨ 基于内容的文件名智能推荐
- ✨ 最近文件列表和快速访问
- ✨ 配置持久化存储

#### 预览功能
- ✨ Markdown 实时预览渲染
  - 支持标题、列表、代码块、引用
  - 支持粗体、斜体、链接
  - 支持代码块语法标识
- ✨ SVG 图片实时渲染
  - 支持渐变、形状、文本
  - 支持复杂的 SVG 特性
- ✨ 代码语法高亮
  - 支持 15+ 种编程语言
  - 关键字、字符串、注释着色

#### 文件支持
- ✅ 文本文件 (`.txt`)
- ✅ Markdown (`.md`)
- ✅ Rust (`.rs`)
- ✅ Python (`.py`)
- ✅ JavaScript (`.js`)
- ✅ TypeScript (`.ts`)
- ✅ HTML (`.html`, `.htm`)
- ✅ CSS (`.css`)
- ✅ JSON (`.json`)
- ✅ XML (`.xml`)
- ✅ YAML (`.yaml`, `.yml`)
- ✅ TOML (`.toml`)
- ✅ SVG (`.svg`)
- ✅ C (`.c`)
- ✅ C++ (`.cpp`, `.cc`, `.cxx`)
- ✅ Java (`.java`)
- ✅ Go (`.go`)

#### Windows 集成
- ✨ 一键文件关联功能
- ✨ 选择性文件关联
- ✨ 自签名证书生成
- ✨ 可执行文件签名
- ✨ 自定义图标支持

#### 文档
- 📚 中英文 README
- 📚 详细构建指南
- 📚 快速入门指南
- 📚 贡献指南
- 📚 Apache 2.0 许可证

### 技术栈

- **GUI 框架**: egui 0.27 + eframe 0.27
- **Markdown 解析**: pulldown-cmark 0.10
- **SVG 渲染**: resvg 0.40 + usvg 0.40 + tiny-skia 0.11
- **语法高亮**: syntect 5
- **序列化**: serde 1 + serde_json 1
- **文件对话框**: rfd 0.14
- **Windows 注册表**: winreg 0.52

### 性能优化

- ⚡ Release 构建启用 LTO
- ⚡ 优化级别 3
- ⚡ SVG 渲染使用 tiny-skia

## 版本格式说明

- **[主要版本]**: 破坏性 API 变更
- **[次要版本]**: 新功能，向后兼容
- **[修订版本]**: Bug 修复，向后兼容

### 变更类型

- `新增` ✨ - 新功能
- `更改` 🔧 - 功能变更
- `弃用` ⚠️ - 即将移除的功能
- `移除` ❌ - 已移除的功能
- `修复` 🐛 - Bug 修复
- `安全` 🔒 - 安全相关
- `性能` ⚡ - 性能优化
- `文档` 📚 - 文档更新

---

[未发布]: https://github.com/yourusername/smart-text-editor/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/yourusername/smart-text-editor/releases/tag/v0.1.0
