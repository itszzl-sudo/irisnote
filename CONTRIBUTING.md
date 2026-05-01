# 贡献指南

感谢您考虑为 Smart Text Editor 做出贡献！

## 📋 行为准则

本项目采用贡献者公约作为行为准则。参与此项目即表示您同意遵守其条款。请阅读 [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) 了解详情。

## 🤔 我可以如何贡献？

### 报告 Bug

在提交 Bug 报告之前，请先：
1. 检查 [Issues](https://github.com/yourusername/smart-text-editor/issues) 中是否已有相同问题
2. 确认您使用的是最新版本
3. 阅读文档确认不是使用方式问题

提交 Bug 报告时，请包含：
- **清晰的标题**：简明扼要地描述问题
- **详细描述**：问题发生的完整描述
- **复现步骤**：一步步说明如何复现问题
- **预期行为**：您期望发生什么
- **实际行为**：实际发生了什么
- **环境信息**：
  - 操作系统及版本
  - Rust 版本 (`rustc --version`)
  - 项目版本
- **截图**：如果适用，添加截图帮助解释问题
- **日志**：相关的错误日志或输出

### 建议新功能

我们欢迎新功能建议！请在 Issue 中详细说明：
- **功能描述**：清晰描述您想要的功能
- **使用场景**：说明这个功能如何帮助您
- **可能的实现**：如果您有实现思路，请分享

### 改进文档

文档改进包括但不限于：
- 修正拼写或语法错误
- 添加缺失的文档
- 改进文档结构
- 添加更多示例
- 翻译文档

### 提交代码

## 🔧 开发流程

### 1. Fork 并克隆仓库

```bash
git clone https://github.com/yourusername/smart-text-editor.git
cd smart-text-editor
```

### 2. 创建分支

```bash
git checkout -b feature/your-feature-name
# 或
git checkout -b fix/your-bug-fix
```

分支命名规范：
- `feature/` - 新功能
- `fix/` - Bug 修复
- `docs/` - 文档更新
- `refactor/` - 代码重构
- `test/` - 测试相关

### 3. 设置开发环境

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装开发工具
rustup component add clippy rustfmt

# 构建项目
cargo build
```

### 4. 进行更改

- 遵循代码规范（见下文）
- 添加必要的测试
- 更新相关文档
- 保持提交历史清晰

### 5. 运行测试

```bash
# 运行测试
cargo test

# 运行代码检查
cargo clippy

# 格式化代码
cargo fmt

# 检查格式
cargo fmt --check
```

确保所有检查都通过。

### 6. 提交更改

提交信息格式：
```
<type>(<scope>): <subject>

<body>

<footer>
```

类型：
- `feat`: 新功能
- `fix`: Bug 修复
- `docs`: 文档更新
- `style`: 代码格式（不影响功能）
- `refactor`: 代码重构
- `test`: 测试相关
- `chore`: 构建或辅助工具变更

示例：
```
feat(preview): add support for PDF preview

- Integrate pdf-renderer library
- Add PDF preview mode
- Update preview.rs module

Closes #123
```

### 7. 推送并创建 PR

```bash
git push origin feature/your-feature-name
```

然后在 GitHub 上创建 Pull Request。

## 📝 Pull Request 指南

### PR 标题

使用清晰的标题，格式同提交信息：
```
feat(preview): add PDF preview support
```

### PR 描述

PR 描述应包含：

**更改类型**
- [ ] Bug 修复
- [ ] 新功能
- [ ] 破坏性变更
- [ ] 文档更新

**描述**
清晰描述您的更改及其原因。

**相关 Issue**
```
Fixes #123
Closes #456
```

**测试**
说明如何测试这些更改。

**截图**
如果是 UI 相关更改，请提供前后对比截图。

**清单**
- [ ] 代码遵循项目规范
- [ ] 已添加测试
- [ ] 所有测试通过
- [ ] 已更新文档
- [ ] 提交历史清晰

## 🎨 代码规范

### Rust 代码规范

遵循 Rust 官方代码规范：
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 避免不必要的 `unwrap()`，优先使用 `?` 或模式匹配
- 添加适当的注释和文档注释 (`///`)
- 保持函数简短和单一职责
- 使用有意义的变量和函数名

### 示例

```rust
/// Detects file type from content and path
/// 
/// # Arguments
/// 
/// * `content` - The file content as string
/// * `path` - Optional file path for extension hint
/// 
/// # Returns
/// 
/// The detected file type
pub fn detect_file_type(content: &str, path: Option<&Path>) -> FileType {
    if let Some(p) = path {
        if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
            return match ext.to_lowercase().as_str() {
                "md" => FileType::Markdown,
                "rs" => FileType::Rust,
                // ...
                _ => detect_from_content(content),
            };
        }
    }
    detect_from_content(content)
}
```

### 文档规范

- 使用 Markdown 格式
- 保持简洁明了
- 提供代码示例
- 更新相关文档

### 提交规范

- 保持提交小而聚焦
- 使用清晰的提交信息
- 引用相关 Issue
- 避免提交无关更改

## 🏗️ 项目结构

```
src/
├── main.rs             # 主程序入口
├── file_type.rs        # 文件类型检测
├── preview.rs          # 预览渲染
├── config.rs           # 配置管理
└── file_association.rs # 文件关联
```

## 🧪 测试

### 运行测试

```bash
cargo test
```

### 添加测试

为新功能添加单元测试：

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_rust_code() {
        let content = "fn main() { println!(\"Hello\"); }";
        let result = detect_from_content(content);
        assert_eq!(result, FileType::Rust);
    }
}
```

## 📚 文档

### 构建文档

```bash
cargo doc --open
```

### 文档位置

- `README.md` - 英文说明
- `README_CN.md` - 中文说明
- `BUILD.md` - 构建指南
- `QUICKSTART.md` - 快速入门
- `CONTRIBUTING.md` - 本文件

## ❓ 获取帮助

- [GitHub Issues](https://github.com/yourusername/smart-text-editor/issues) - Bug 报告和功能建议
- [GitHub Discussions](https://github.com/yourusername/smart-text-editor/discussions) - 问题和讨论

## 📄 许可证

通过贡献代码，您同意您的贡献将根据 Apache License 2.0 进行许可。

---

再次感谢您的贡献！🎉
