# IrisNote 发布指南 (使用 Signpath)

## 🎯 发布流程

IrisNote 使用 **Signpath 免费签名服务** 对 Windows 可执行文件进行签名。

### ✨ 签名优势

- ✅ 免费使用（每月 100 次签名）
- ✅ 专业代码签名证书
- ✅ 自动化签名流程
- ✅ Windows SmartScreen 友好
- ✅ 提升用户信任度

## 📋 发布前准备

### 1. 配置 Signpath（首次）

如果还没有配置 Signpath，请参考 `SIGNPATH_SETUP.md` 完成：

1. 注册 Signpath 账户
2. 创建组织和项目
3. 配置签名策略
4. 获取 API Token
5. 配置 GitHub Secrets

### 2. 配置 GitHub Secrets

访问：https://github.com/itszzl-sudo/irisnote/settings/secrets/actions

添加以下 Secrets：

| Secret 名称 | 说明 | 获取方式 |
|------------|------|---------|
| `SIGNPATH_API_TOKEN` | Signpath API Token | Signpath 用户设置 |
| `SIGNPATH_ORG_ID` | 组织 ID | Signpath 组织设置 |
| `SIGNPATH_PROJECT_SLUG` | 项目 Slug | 通常是 `irisnote` |

## 🚀 发布步骤

### 方式 A: 自动发布（推荐）

#### 步骤 1: 更新版本号

编辑 `Cargo.toml`：

```toml
[package]
version = "0.2.0"  # 更新版本号
```

#### 步骤 2: 更新 CHANGELOG

编辑 `CHANGELOG.md`，添加新版本说明：

```markdown
## [0.2.0] - 2024-05-02

### 新增
- ✨ 新功能...

### 修复
- 🐛 Bug 修复...
```

#### 步骤 3: 提交并打标签

```bash
# 提交更改
git add .
git commit -m "chore: release v0.2.0"

# 创建标签
git tag -a v0.2.0 -m "Release v0.2.0"

# 推送到 GitHub
git push origin master
git push origin v0.2.0
```

#### 步骤 4: 创建 GitHub Release

1. 访问：https://github.com/itszzl-sudo/irisnote/releases/new
2. 选择标签：`v0.2.0`
3. 填写 Release 说明
4. 点击 "Publish release"

#### 步骤 5: 自动构建和签名

创建 Release 后，GitHub Actions 会自动：

1. ✅ 构建 Windows/Linux/macOS 版本
2. ✅ 使用 Signpath 签名 Windows 版本
3. ✅ 创建发布包
4. ✅ 上传到 Release

大约 10-15 分钟完成。

### 方式 B: 手动发布

#### 步骤 1: 本地构建和签名

```bash
# 使用 Signpath 脚本
.\build-and-sign-signpath.bat

# 或设置环境变量后使用
set SIGNPATH_API_TOKEN=your_token
set SIGNPATH_ORG_ID=your_org_id
.\build-and-sign-signpath.bat
```

#### 步骤 2: 手动上传

1. 创建 GitHub Release
2. 上传生成的 ZIP 文件
3. 填写 Release 说明

## 📦 发布产物

每次 Release 会生成以下文件：

### Windows
```
irisnote-vX.Y.Z-windows-x64.zip
├── irisnote.exe          # 已签名的可执行文件
├── README.md             # 英文说明
├── README_CN.md          # 中文说明
├── LICENSE               # Apache 2.0 许可证
└── NOTICE                # 版权声明
```

### Linux
```
irisnote-vX.Y.Z-linux-x64.tar.gz
├── irisnote              # 可执行文件
├── README.md
├── README_CN.md
├── LICENSE
└── NOTICE
```

### macOS
```
irisnote-vX.Y.Z-macos-x64.tar.gz
├── irisnote              # 可执行文件
├── README.md
├── README_CN.md
├── LICENSE
└── NOTICE
```

## 🔍 验证签名

### Windows 用户

下载后右键点击 `irisnote.exe` → 属性 → 数字签名：

应显示：
- 签名者：Signpath Certificate
- 时间戳：签名时间
- 状态：✅ 此数字签名正常

### 命令行验证

```powershell
# PowerShell
Get-AuthenticodeSignature irisnote.exe

# 应显示: Valid
```

## 📊 Release 模板

```markdown
# IrisNote vX.Y.Z

## ✨ 新增功能
- 

## 🔧 更改
- 

## 🐛 Bug 修复
- 

## 📚 文档
- 

---

## 📦 下载

### Windows
下载 `irisnote-vX.Y.Z-windows-x64.zip`

### Linux
下载 `irisnote-vX.Y.Z-linux-x64.tar.gz`

### macOS
下载 `irisnote-vX.Y.Z-macos-x64.tar.gz`

## 🔒 安全

Windows 版本已通过 Signpath 代码签名，可放心使用。

---

**完整更新日志**: https://github.com/itszzl-sudo/irisnote/compare/vX.Y.(Z-1)...vX.Y.Z
```

## 🛠️ 故障排除

### Q: Signpath 签名失败

**检查**：
1. GitHub Secrets 是否正确配置
2. Signpath 组织和项目是否创建
3. 签名策略是否配置
4. API Token 是否有效

**解决方案**：
- 查看 GitHub Actions 日志
- 访问 Signpath 控制台检查状态
- 参考 `SIGNPATH_SETUP.md` 重新配置

### Q: GitHub Actions 失败

**查看日志**：
1. 访问仓库的 Actions 页面
2. 点击失败的工作流
3. 查看具体错误信息

**常见原因**：
- 编译错误：检查代码
- 签名失败：检查 Signpath 配置
- 上传失败：检查 Release 权限

### Q: 签名后的文件仍有警告

**说明**：
- 自签名证书首次使用会有 SmartScreen 警告
- 这是正常现象，需要建立声誉
- Signpath 签名比自签名更可信

## 📈 发布统计

可以在以下位置查看：

- **GitHub Release**: https://github.com/itszzl-sudo/irisnote/releases
- **Signpath 控制台**: https://signpath.io (签名统计)
- **GitHub Actions**: 构建历史

## 🔄 版本规划

### 版本号规则

遵循语义化版本：
- **主版本号 (MAJOR)**: 破坏性变更
- **次版本号 (MINOR)**: 新功能
- **修订号 (PATCH)**: Bug 修复

示例：
- `0.1.0` → `0.1.1`: Bug 修复
- `0.1.1` → `0.2.0`: 新功能
- `0.2.0` → `1.0.0`: 破坏性变更

## 📝 发布检查清单

### 发布前

- [ ] 更新 Cargo.toml 版本号
- [ ] 更新 CHANGELOG.md
- [ ] 更新 CITATION.cff 版本
- [ ] 运行测试：`cargo test`
- [ ] 检查代码：`cargo clippy`
- [ ] 格式化代码：`cargo fmt`

### 发布时

- [ ] 创建 Git 标签
- [ ] 推送到 GitHub
- [ ] 创建 GitHub Release
- [ ] 等待自动构建和签名

### 发布后

- [ ] 验证 Release 产物
- [ ] 下载并测试签名
- [ ] 更新文档网站（如有）
- [ ] 通知社区

## 🌟 最佳实践

### 1. 发布频率

- **Patch 版本**: 随时发布（Bug 修复）
- **Minor 版本**: 每月或每季度（新功能）
- **Major 版本**: 谨慎规划（破坏性变更）

### 2. Release 说明

- 清晰说明变更内容
- 列出新增功能
- 说明破坏性变更
- 感谢贡献者

### 3. 签名验证

- 发布后立即验证签名
- 在不同 Windows 版本测试
- 检查 SmartScreen 行为

## 📚 相关文档

- `SIGNPATH_SETUP.md` - Signpath 配置指南
- `CONTRIBUTING.md` - 贡献指南
- `CHANGELOG.md` - 更新日志
- `.github/workflows/release.yml` - 自动发布配置

---

## 🎉 快速发布命令

```bash
# 1. 更新版本
vim Cargo.toml  # 修改 version

# 2. 更新日志
vim CHANGELOG.md  # 添加版本说明

# 3. 提交
git add .
git commit -m "chore: release v0.2.0"

# 4. 打标签
git tag -a v0.2.0 -m "Release v0.2.0"

# 5. 推送
git push origin master
git push origin v0.2.0

# 6. 在 GitHub 创建 Release
# 访问：https://github.com/itszzl-sudo/irisnote/releases/new
```

**自动构建和签名会在创建 Release 后开始！**
