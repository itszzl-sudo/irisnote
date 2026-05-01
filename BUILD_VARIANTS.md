# IrisNote 构建指南

## 📦 构建版本说明

IrisNote 提供两种构建版本，可根据需求选择：

### 标准版（推荐大多数用户）

**特点**：
- ✅ 无需额外依赖
- ✅ 体积小（~15 MB）
- ✅ 启动快
- ✅ 即下即用

**功能**：
- ✅ 文件类型自动检测（19 种）
- ✅ 语法高亮（15+ 语言）
- ✅ Markdown 实时预览
- ✅ SVG 图片渲染
- ✅ 智能文件命名（传统方法）
- ✅ Windows 文件关联
- ❌ AI 智能命名

**适合**：
- 日常文本编辑
- 代码查看和编辑
- Markdown 文档编写
- 不需要 AI 功能的用户

### AI 增强版

**特点**：
- ⚠️ 需要 llama.cpp
- ⚠️ 体积较大（~25 MB）
- ⚠️ 需要模型文件（~400 MB）
- ⚠️ 需要启动 LLM 服务

**功能**：
- ✅ 所有标准版功能
- ✅ **AI 智能文件命名**
- ✅ 内容智能总结
- ✅ 更准确的标题建议

**适合**：
- 需要智能文件命名的用户
- 处理大量未知类型文件
- 追求更准确的命名建议

## 🚀 快速构建

### 方式 1: 标准版（推荐）

```bash
# 使用构建脚本
.\build-standard.bat

# 或手动构建
cargo build --release
```

**输出**：
- `target\release\irisnote.exe` (~15 MB)
- `irisnote-v0.1.0-windows-x64-standard.zip`

### 方式 2: AI 增强版

```bash
# 使用构建脚本
.\build-ai.bat

# 或手动构建
cargo build --release --features llm
```

**输出**：
- `target\release\irisnote.exe` (~25 MB)
- `irisnote-v0.1.0-windows-x64-ai.zip`

## 📋 详细构建步骤

### 标准版构建

#### 步骤 1: 安装 Rust

```bash
# 访问 https://rustup.rs
# 或使用包管理器

# Windows (winget)
winget install Rustlang.Rustup

# macOS/Linux
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### 步骤 2: 克隆仓库

```bash
git clone https://github.com/itszzl-sudo/irisnote.git
cd irisnote
```

#### 步骤 3: 构建

```bash
# Debug 版本（快速，用于测试）
cargo build

# Release 版本（优化，用于生产）
cargo build --release
```

#### 步骤 4: 运行

```bash
# Debug
.\target\debug\irisnote.exe

# Release
.\target\release\irisnote.exe
```

构建时间：
- Debug: 3-5 分钟
- Release: 5-10 分钟

### AI 增强版构建

#### 步骤 1-3: 同标准版

#### 步骤 4: 下载模型

```bash
# 访问
https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF

# 下载
qwen2.5-coder-0.5b-instruct-q4_k_m.gguf

# 保存到
C:\Users\a\Downloads\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf
```

#### 步骤 5: 构建 AI 版本

```bash
cargo build --release --features llm
```

#### 步骤 6: 安装 llama.cpp

```bash
# 下载预编译版本
https://github.com/ggerganov/llama.cpp/releases

# 解压到项目的 llama.cpp 目录
```

#### 步骤 7: 启动 LLM 服务

```bash
.\start-llm-server.bat
```

#### 步骤 8: 运行

```bash
.\target\release\irisnote.exe
```

## 📊 版本对比

| 特性 | 标准版 | AI 增强版 |
|------|--------|-----------|
| 文件大小 | ~15 MB | ~25 MB |
| 内存占用 | ~50 MB | ~650 MB |
| 启动时间 | < 1 秒 | 1-2 秒 |
| 构建时间 | 5-10 分钟 | 8-15 分钟 |
| 外部依赖 | 无 | llama.cpp + 模型 |
| 文件类型检测 | ✅ | ✅ |
| 语法高亮 | ✅ | ✅ |
| Markdown 预览 | ✅ | ✅ |
| SVG 渲染 | ✅ | ✅ |
| 传统命名 | ✅ | ✅ |
| AI 命名 | ❌ | ✅ |

## 🔧 构建配置

### 默认特性

```toml
[features]
default = []
llm = ["tokio", "reqwest", "async-trait"]
```

### 选择特性构建

```bash
# 无特性（标准版）
cargo build --release

# LLM 特性（AI 增强版）
cargo build --release --features llm

# 所有特性
cargo build --release --all-features
```

### 优化选项

编辑 `Cargo.toml`：

```toml
[profile.release]
opt-level = 3        # 最高优化
lto = true           # 链接时优化
codegen-units = 1    # 更好的优化
strip = true         # 减小体积
```

## 🛠️ 开发构建

### Debug 构建

```bash
# 快速构建，包含调试信息
cargo build

# 运行
.\target\debug\irisnote.exe
```

特点：
- 编译快（3-5 分钟）
- 包含调试信息
- 体积大（~100 MB）
- 性能低

### 检查模式

```bash
# 快速检查，不生成可执行文件
cargo check
```

用途：快速发现编译错误

### 测试

```bash
# 运行测试
cargo test

# 运行特定测试
cargo test --test test_name
```

## 📦 发布构建

### 标准版发布

```bash
# 构建
cargo build --release

# 打包
7z a irisnote-v0.1.0-windows-x64-standard.zip ^
    target\release\irisnote.exe ^
    README.md README_CN.md ^
    LICENSE NOTICE
```

### AI 增强版发布

```bash
# 构建
cargo build --release --features llm

# 打包
7z a irisnote-v0.1.0-windows-x64-ai.zip ^
    target\release\irisnote.exe ^
    README.md README_CN.md ^
    LICENSE NOTICE ^
    LLM_SETUP.md ^
    start-llm-server.bat
```

## 🌍 跨平台构建

### Windows

```bash
cargo build --release
```

### Linux

```bash
cargo build --release

# 打包
tar czf irisnote-v0.1.0-linux-x64.tar.gz \
    target/release/irisnote \
    README.md README_CN.md \
    LICENSE NOTICE
```

### macOS

```bash
cargo build --release

# 打包
tar czf irisnote-v0.1.0-macos-x64.tar.gz \
    target/release/irisnote \
    README.md README_CN.md \
    LICENSE NOTICE
```

## 🔍 故障排除

### 问题 1: 编译错误

**症状**：`cargo build` 失败

**检查**：
```bash
# Rust 版本
rustc --version  # 需要 1.70+

# 更新 Rust
rustup update

# 清理并重建
cargo clean
cargo build
```

### 问题 2: 链接错误

**症状**：`linker 'link.exe' not found`

**解决**：
```bash
# 安装 Visual Studio Build Tools
winget install Microsoft.VisualStudio.2022.BuildTools

# 或安装完整的 Visual Studio
```

### 问题 3: 依赖下载慢

**症状**：下载依赖超时

**解决**：使用国内镜像

```bash
# 创建或编辑 %USERPROFILE%\.cargo\config
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### 问题 4: AI 版本编译失败

**症状**：启用 `llm` 特性后编译失败

**解决**：
```bash
# 确保所有依赖都支持该特性
cargo update

# 清理并重建
cargo clean
cargo build --release --features llm
```

### 问题 5: 内存不足

**症状**：编译时内存不足

**解决**：
```bash
# 减少并行编译
cargo build -j 2

# 或使用更少的代码生成单元
# 编辑 .cargo/config.toml
[profile.release]
codegen-units = 16
```

## 📚 相关文档

- `build-standard.bat` - 标准版构建脚本
- `build-ai.bat` - AI 增强版构建脚本
- `LLM_SETUP.md` - LLM 配置指南
- `README.md` - 项目说明

## ✅ 构建检查清单

### 标准版

- [ ] 安装 Rust
- [ ] 克隆仓库
- [ ] 运行 `cargo build --release`
- [ ] 测试运行
- [ ] 打包发布

### AI 增强版

- [ ] 安装 Rust
- [ ] 克隆仓库
- [ ] 下载模型文件
- [ ] 运行 `cargo build --release --features llm`
- [ ] 安装 llama.cpp
- [ ] 启动 LLM 服务
- [ ] 测试 AI 功能
- [ ] 打包发布

## 🎯 推荐选择

### 选择标准版，如果：

- ✅ 只需要基本编辑功能
- ✅ 不想安装额外依赖
- ✅ 追求快速启动
- ✅ 文件体积敏感

### 选择 AI 增强版，如果：

- ✅ 需要智能文件命名
- ✅ 处理大量未知文件
- ✅ 追求更准确的建议
- ✅ 不介意额外依赖

---

## 💡 建议

**首次使用**：推荐先试用标准版

**需要 AI**：再升级到 AI 增强版

**开发测试**：使用 Debug 构建

**正式发布**：使用 Release 构建

---

**开始构建**：

```bash
# 标准版
.\build-standard.bat

# AI 增强版
.\build-ai.bat
```
