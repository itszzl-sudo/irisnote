# 依赖说明

本文档列出 Smart Text Editor 使用的所有依赖及其用途。

## 直接依赖

### GUI 框架

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `eframe` | 0.27 | Apache-2.0 | egui 的应用程序框架 |
| `egui` | 0.27 | Apache-2.0 | 即时模式 GUI 库 |
| `egui_extras` | 0.27 | Apache-2.0 | egui 额外功能 |

### 文档处理

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `pulldown-cmark` | 0.10 | MIT | Markdown 解析器 |
| `syntect` | 5.0 | MIT | 语法高亮 |

### 图形渲染

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `resvg` | 0.40 | MPL-2.0 | SVG 渲染 |
| `usvg` | 0.40 | MPL-2.0 | SVG 解析 |
| `tiny-skia` | 0.11 | BSD-3-Clause | 2D 图形库 |
| `image` | 0.25 | MIT | 图片处理 |

### 序列化

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `serde` | 1.0 | Apache-2.0 | 序列化框架 |
| `serde_json` | 1.0 | Apache-2.0 | JSON 序列化 |

### 系统集成

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `dirs` | 5.0 | MIT OR Apache-2.0 | 系统目录路径 |
| `rfd` | 0.14 | MIT OR Apache-2.0 | 文件对话框 |
| `winreg` | 0.52 | MIT | Windows 注册表 |

### 构建依赖

| 依赖 | 版本 | 许可证 | 用途 |
|------|------|--------|------|
| `winres` | 0.1 | MIT | Windows 资源嵌入 |

## 依赖树

```
smart-text-editor
├── eframe (GUI)
│   ├── egui
│   ├── egui-winit
│   └── egui_glow
├── pulldown-cmark (Markdown)
├── syntect (Syntax)
├── resvg (SVG)
│   ├── usvg
│   └── tiny-skia
├── serde (Serialization)
│   └── serde_json
├── dirs (Paths)
├── rfd (Dialogs)
└── winreg (Registry)
```

## 许可证兼容性

所有依赖的许可证都与 Apache 2.0 兼容：

- ✅ Apache-2.0
- ✅ MIT
- ✅ MPL-2.0
- ✅ BSD-3-Clause
- ✅ MIT OR Apache-2.0

## 安全审计

定期运行依赖安全审计：

```bash
# 安装 cargo-audit
cargo install cargo-audit

# 运行审计
cargo audit
```

在 CI/CD 中集成：

```yaml
- name: Security audit
  run: cargo audit
```

## 更新依赖

### 检查更新

```bash
# 安装 cargo-outdated
cargo install cargo-outdated

# 检查过时的依赖
cargo outdated
```

### 更新依赖

```bash
# 更新所有依赖到最新兼容版本
cargo update

# 更新特定依赖
cargo update -p package-name
```

### 注意事项

- 更新前检查 CHANGELOG
- 运行所有测试
- 检查是否有破坏性变更
- 更新 Cargo.lock

## 依赖大小

编译后大小（Release 模式）：

| 组件 | 大小 |
|------|------|
| egui 框架 | ~2 MB |
| SVG 渲染 | ~1 MB |
| 语法高亮 | ~3 MB |
| 其他 | ~1 MB |
| **总计** | ~7 MB |

## 性能考虑

### 启动时间

- egui 初始化: ~100ms
- 配置加载: ~10ms
- 总启动时间: ~200ms

### 内存使用

- 基础内存: ~20 MB
- 文件缓冲: 取决于文件大小
- SVG 渲染: 取决于图形复杂度

### 优化建议

1. **减小体积**
   ```toml
   [profile.release]
   opt-level = 3
   lto = true
   codegen-units = 1
   panic = "abort"
   ```

2. **加快启动**
   - 延迟加载语法高亮定义
   - 缓存渲染结果

3. **减少内存**
   - 限制撤销历史大小
   - 限制预览缓冲区大小

## 替代方案

如果需要减小依赖：

### 最小依赖集

```toml
[dependencies]
eframe = "0.27"  # 必需
pulldown-cmark = "0.10"  # Markdown
serde = { version = "1", features = ["derive"] }  # 配置
serde_json = "1"  # 配置
dirs = "5"  # 路径
```

### 可选依赖

```toml
[dependencies]
resvg = { version = "0.40", optional = true }
syntect = { version = "5", optional = true }

[features]
default = ["svg", "syntax"]
svg = ["resvg", "usvg", "tiny-skia"]
syntax = ["syntect"]
```

## 问题排查

### 依赖冲突

```bash
# 检查依赖树
cargo tree

# 检查重复依赖
cargo tree --duplicates
```

### 编译错误

如果遇到编译错误：

1. 清理并重新构建
   ```bash
   cargo clean
   cargo build
   ```

2. 更新依赖
   ```bash
   cargo update
   ```

3. 检查 Rust 版本
   ```bash
   rustc --version
   rustup update
   ```

---

依赖选择遵循以下原则：
1. 活跃维护
2. 良好文档
3. 社区信任
4. 许可证兼容
5. 性能良好
