# 发布指南

本文档说明如何发布 Smart Text Editor 的新版本。

## 发布前检查清单

### 1. 代码质量

- [ ] 所有测试通过
  ```bash
  cargo test
  ```

- [ ] 代码格式检查通过
  ```bash
  cargo fmt --check
  ```

- [ ] Clippy 检查通过
  ```bash
  cargo clippy -- -D warnings
  ```

- [ ] 安全审计通过
  ```bash
  cargo audit
  ```

### 2. 文档更新

- [ ] 更新 CHANGELOG.md
- [ ] 更新版本号
  - Cargo.toml
  - CITATION.cff
  - README.md (如有必要)
- [ ] 检查文档链接有效性
- [ ] 更新统计信息

### 3. 构建验证

- [ ] Windows 构建成功
  ```bash
  cargo build --release
  ```

- [ ] Linux 构建成功 (如有 CI)
- [ ] macOS 构建成功 (如有 CI)

### 4. 功能测试

- [ ] 新功能测试
- [ ] 回归测试
- [ ] 不同平台测试

## 版本号规范

遵循 [语义化版本](https://semver.org/lang/zh-CN/):

- **主版本号 (MAJOR)**: 破坏性 API 变更
- **次版本号 (MINOR)**: 新功能，向后兼容
- **修订号 (PATCH)**: Bug 修复，向后兼容

示例:
- `0.1.0` → `0.1.1`: Bug 修复
- `0.1.1` → `0.2.0`: 新功能
- `0.2.0` → `1.0.0`: 破坏性变更

## 发布流程

### 步骤 1: 准备发布

```bash
# 确保在主分支
git checkout main
git pull origin main

# 创建发布分支
git checkout -b release/vX.Y.Z
```

### 步骤 2: 更新版本

```bash
# 更新 Cargo.toml 中的版本
# 更新 CITATION.cff 中的版本
# 更新 CHANGELOG.md
```

### 步骤 3: 提交更改

```bash
git add .
git commit -m "chore: prepare release vX.Y.Z"
git push origin release/vX.Y.Z
```

### 步骤 4: 创建 PR

在 GitHub 上创建 Pull Request，标题: `Release vX.Y.Z`

### 步骤 5: 合并并打标签

```bash
# 合并到主分支
git checkout main
git merge release/vX.Y.Z

# 创建标签
git tag -a vX.Y.Z -m "Release vX.Y.Z"
git push origin main --tags
```

### 步骤 6: 构建发布版本

#### Windows

```bash
# 构建 Release 版本
cargo build --release

# 创建自签名证书
powershell -Command "$cert = New-SelfSignedCertificate -Type CodeSigningCert -Subject 'CN=Smart Text Editor' -CertStoreLocation Cert:\CurrentUser\My"

# 签名可执行文件
powershell -Command "$cert = Get-ChildItem Cert:\CurrentUser\My | Where-Object { $_.Subject -like '*Smart Text Editor*' } | Select-Object -First 1; Set-AuthenticodeSignature -FilePath '.\target\release\smart-text-editor.exe' -Certificate $cert"

# 打包
7z a smart-text-editor-vX.Y.Z-windows-x64.zip target\release\smart-text-editor.exe README*.md LICENSE
```

#### Linux

```bash
cargo build --release
tar czf smart-text-editor-vX.Y.Z-linux-x64.tar.gz target/release/smart-text-editor README*.md LICENSE
```

#### macOS

```bash
cargo build --release
tar czf smart-text-editor-vX.Y.Z-macos-x64.tar.gz target/release/smart-text-editor README*.md LICENSE
```

### 步骤 7: 创建 GitHub Release

1. 访问 [Releases](https://github.com/yourusername/smart-text-editor/releases)
2. 点击 "Draft a new release"
3. 选择标签 `vX.Y.Z`
4. 填写标题: `Smart Text Editor vX.Y.Z`
5. 填写说明（从 CHANGELOG.md 复制）
6. 上传构建文件
7. 点击 "Publish release"

## Release 说明模板

```markdown
# Smart Text Editor vX.Y.Z

## 新增 ✨
- 

## 更改 🔧
- 

## 修复 🐛
- 

## 移除 ❌
- 

## 文档 📚
- 

## 性能 ⚡
- 

## 其他
- 

---

## 安装

### Windows
下载 `smart-text-editor-vX.Y.Z-windows-x64.zip` 并解压。

### Linux
下载 `smart-text-editor-vX.Y.Z-linux-x64.tar.gz` 并解压。

### macOS
下载 `smart-text-editor-vX.Y.Z-macos-x64.tar.gz` 并解压。

## 更新说明
在此详细说明此次更新的重要变更。

---

**完整更新日志**: https://github.com/yourusername/smart-text-editor/compare/vX.Y.(Z-1)...vX.Y.Z
```

## 自动化发布

### GitHub Actions

已配置 CI/CD 自动构建:

```yaml
# .github/workflows/ci.yml
# 自动测试和构建
```

### 发布工作流 (可选)

创建 `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    # 构建各平台版本
    
  release:
    needs: build
    # 创建 GitHub Release
```

## 发布后

### 1. 更新文档

- [ ] 更新 GitHub Pages (如有)
- [ ] 更新网站 (如有)
- [ ] 更新演示 (如有)

### 2. 通知

- [ ] 发布公告
- [ ] 社交媒体通知
- [ ] 通知贡献者

### 3. 监控

- [ ] 检查 Issue
- [ ] 监控下载量
- [ ] 收集反馈

## 紧急修复

### Hotfix 流程

```bash
# 从标签创建分支
git checkout -b hotfix/vX.Y.Z vX.Y.Z

# 修复问题
# ...

# 提交
git commit -m "fix: critical bug"

# 创建新版本
# 按照正常发布流程

# 合并回主分支
git checkout main
git merge hotfix/vX.Y.Z
```

## 撤回发布

如果发现严重问题:

1. 在 GitHub Release 中标记为 pre-release
2. 删除有问题的文件
3. 发布修复版本
4. 通知用户

## 发布历史

| 版本 | 日期 | 说明 |
|------|------|------|
| 0.1.0 | 2024-05-01 | 初始版本 |

---

最后更新: 2024-05-01
