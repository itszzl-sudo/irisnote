# ✅ Signpath 签名配置完成

## 🎉 已完成的工作

### 1. 创建配置文件

| 文件 | 说明 | 状态 |
|------|------|------|
| `SIGNPATH_SETUP.md` | Signpath 配置指南 | ✅ 已创建 |
| `RELEASE_SIGNPATH.md` | 发布指南（含签名） | ✅ 已创建 |
| `.github/workflows/release.yml` | 自动发布和签名工作流 | ✅ 已创建 |
| `.github/workflows/ci.yml` | CI 工作流（更新） | ✅ 已更新 |
| `build-and-sign-signpath.bat` | Signpath 构建脚本 | ✅ 已创建 |
| `README.md` | 英文说明（添加签名信息） | ✅ 已更新 |
| `README_CN.md` | 中文说明（添加签名信息） | ✅ 已更新 |

### 2. 推送到 GitHub

```
提交: b6085d2
分支: master
状态: ✅ 已推送
```

## 🔧 Signpath 集成说明

### 自动签名流程

创建 GitHub Release 时会自动：

1. **构建** - Windows/Linux/macOS 三个平台
2. **签名** - Windows 版本使用 Signpath 签名
3. **打包** - 创建分发包
4. **上传** - 附加到 GitHub Release

### GitHub Actions 工作流

#### CI 工作流（`.github/workflows/ci.yml`）
- 触发：Push 到 master/main/develop 分支
- 任务：测试、构建、上传构建产物

#### Release 工作流（`.github/workflows/release.yml`）
- 触发：创建 GitHub Release
- 任务：
  - 构建 Windows 版本
  - 使用 Signpath 签名
  - 构建 Linux/macOS 版本
  - 创建发布包
  - 上传到 Release

## 📋 下一步：配置 Signpath

### 步骤 1: 注册 Signpath

访问：https://signpath.io

1. 点击 "Get started for free"
2. 使用 GitHub 账户登录
3. 授权访问

### 步骤 2: 创建组织

1. 组织名称：`IrisNote` 或 `itszzl-sudo`
2. 选择免费计划

### 步骤 3: 创建项目

1. 项目名称：`irisnote`
2. GitHub 仓库：`itszzl-sudo/irisnote`
3. 项目类型：Windows Application

### 步骤 4: 配置签名策略

1. 策略名称：`release-signing`
2. 证书类型：Standard Code Signing
3. 自动签名：启用

### 步骤 5: 获取 API Token

1. 访问用户设置
2. 创建 API Token
3. 权限：Signing
4. 复制并保存 Token

### 步骤 6: 配置 GitHub Secrets

访问：https://github.com/itszzl-sudo/irisnote/settings/secrets/actions

添加以下 Secrets：

| Secret 名称 | 值 |
|------------|---|
| `SIGNPATH_API_TOKEN` | 您的 API Token |
| `SIGNPATH_ORG_ID` | 组织 ID（在 Signpath 查看） |
| `SIGNPATH_PROJECT_SLUG` | `irisnote` |

## 🚀 测试签名

### 方式 1: 创建测试 Release

```bash
# 创建测试标签
git tag -a v0.1.1-test -m "Test Signpath signing"
git push origin v0.1.1-test

# 在 GitHub 创建 Release（标记为 pre-release）
```

GitHub Actions 会自动构建并签名。

### 方式 2: 本地测试

```bash
# 设置环境变量
set SIGNPATH_API_TOKEN=your_token
set SIGNPATH_ORG_ID=your_org_id

# 运行构建脚本
.\build-and-sign-signpath.bat
```

## 📦 发布流程

配置完成后，标准发布流程：

```bash
# 1. 更新版本号
vim Cargo.toml  # version = "0.2.0"

# 2. 更新 CHANGELOG
vim CHANGELOG.md

# 3. 提交
git add .
git commit -m "chore: prepare release v0.2.0"

# 4. 打标签
git tag -a v0.2.0 -m "Release v0.2.0"

# 5. 推送
git push origin master
git push origin v0.2.0

# 6. 在 GitHub 创建 Release
# 访问：https://github.com/itszzl-sudo/irisnote/releases/new

# 7. 等待自动构建和签名（约 10-15 分钟）
```

## 🔍 验证签名

### Windows 用户验证

下载 Release 中的 Windows ZIP 包，解压后：

右键 `irisnote.exe` → 属性 → 数字签名

应显示：
- 签名者：Signpath Certificate
- 状态：✅ 此数字签名正常

### PowerShell 验证

```powershell
Get-AuthenticodeSignature irisnote.exe
# Status 应显示: Valid
```

## 📊 Signpath 免费计划

### 限制
- 每月签名次数：100 次
- 文件大小限制：100 MB
- 并发签名：1 个
- 证书类型：Standard Code Signing

### 对开源项目
- ✅ 完全够用
- ✅ 专业签名证书
- ✅ 自动化集成
- ✅ 免费使用

## 🆘 故障排除

### 签名失败

1. 检查 GitHub Secrets 是否正确
2. 检查 Signpath 组织和项目配置
3. 查看 GitHub Actions 日志
4. 访问 Signpath 控制台检查状态

### 签名警告

- 首次使用可能有 SmartScreen 警告
- 这是正常现象，需要建立声誉
- Signpath 签名比自签名更可信

## 📚 相关文档

- **SIGNPATH_SETUP.md** - 详细配置指南
- **RELEASE_SIGNPATH.md** - 发布流程说明
- **.github/workflows/release.yml** - 自动签名配置
- **build-and-sign-signpath.bat** - 本地签名脚本

## ✅ 配置清单

Signpath 集成已完成：

- [x] 创建配置文档
- [x] 更新 GitHub Actions 工作流
- [x] 创建发布工作流
- [x] 更新 README
- [x] 创建构建脚本
- [x] 推送到 GitHub

下一步（需要您操作）：

- [ ] 注册 Signpath 账户
- [ ] 创建组织和项目
- [ ] 配置签名策略
- [ ] 获取 API Token
- [ ] 配置 GitHub Secrets
- [ ] 测试签名流程

## 🎊 总结

**Signpath 签名集成已完成！**

### 当前状态
- ✅ 所有配置文件已创建
- ✅ GitHub Actions 已配置
- ✅ 自动签名工作流已就绪
- ✅ 文档已更新
- ✅ 已推送到 GitHub

### 下一步
按照 `SIGNPATH_SETUP.md` 完成 Signpath 配置，然后就可以使用自动签名功能了！

---

**项目信息**
- 名称：IrisNote
- 仓库：https://github.com/itszzl-sudo/irisnote
- 签名服务：Signpath
- 许可证：Apache 2.0

**需要帮助？**
- 配置问题：查看 `SIGNPATH_SETUP.md`
- 发布问题：查看 `RELEASE_SIGNPATH.md`
- GitHub Actions：查看工作流日志
