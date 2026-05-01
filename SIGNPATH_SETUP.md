# Sighpath 签名配置指南

## 📋 Sighpath 简介

Sighpath 是一个免费的代码签名服务，用于对 Windows 可执行文件进行数字签名，使应用程序更具可信度。

### 官方资源
- 官网：https://signpath.io
- 文档：https://signpath.io/docs
- GitHub 集成：https://signpath.io/docs/integration/github-actions

## 🔧 配置步骤

### 步骤 1: 注册 Signpath 账户

1. 访问 https://signpath.io
2. 点击 "Get started for free"
3. 使用 GitHub 账户登录
4. 授权 Signpath 访问您的 GitHub 账户

### 步骤 2: 创建组织

1. 登录后，创建新组织
2. 组织名称：`IrisNote` 或 `itszzl-sudo`
3. 选择免费计划

### 步骤 3: 创建签名项目

1. 在组织中创建新项目
2. 项目名称：`IrisNote`
3. 项目类型：Windows Application
4. GitHub 仓库：`itszzl-sudo/irisnote`

### 步骤 4: 配置签名策略

1. 创建签名策略
2. 配置：
   - 代码签名证书类型：Standard
   - 自动签名：启用
   - 触发条件：Release 创建时

### 步骤 5: 获取 API Token

1. 在用户设置中创建 API Token
2. Token 名称：`GitHub Actions`
3. 权限：Signing
4. 保存 Token（只显示一次）

### 步骤 6: 配置 GitHub Secrets

在 GitHub 仓库中配置：

1. 访问：https://github.com/itszzl-sudo/irisnote/settings/secrets/actions
2. 添加以下 Secrets：
   - `SIGNPATH_API_TOKEN`: Signpath API Token
   - `SIGNPATH_ORG_ID`: 组织 ID（在 Signpath 组织设置中查看）
   - `SIGNPATH_PROJECT_SLUG`: 项目 Slug（通常是 `irisnote`）

## 🔨 配置文件更新

### 1. GitHub Actions 工作流

项目已配置自动签名，在 `.github/workflows/ci.yml` 中：

```yaml
- name: Sign with Signpath
  if: startsWith(github.ref, 'refs/tags/')
  uses: signpath/github-action@v1
  with:
    api-token: ${{ secrets.SIGNPATH_API_TOKEN }}
    organization-id: ${{ secrets.SIGNPATH_ORG_ID }}
    project-slug: ${{ secrets.SIGNPATH_PROJECT_SLUG }}
    signing-policy-slug: release-signing
    artifacts: target/release/irisnote.exe
```

### 2. 构建脚本

`build-and-sign-signpath.bat` - 使用 Signpath 签名：

```batch
@echo off
echo 正在使用 Signpath 签名...

REM 构建
cargo build --release

REM 上传到 Signpath 签名
REM 需要先配置 Signpath CLI
signpath sign --org-id %SIGNPATH_ORG_ID% --project irisnote --policy release-signing target\release\irisnote.exe
```

## 📝 签名流程

### 自动签名（推荐）

当创建 GitHub Release 时，自动触发签名：

1. 创建标签并推送：
```bash
git tag -a v0.1.1 -m "Release v0.1.1"
git push origin v0.1.1
```

2. 在 GitHub 创建 Release

3. GitHub Actions 自动：
   - 构建项目
   - 上传到 Signpath
   - 等待签名完成
   - 下载签名后的文件
   - 附加到 Release

### 手动签名

使用 Signpath CLI：

```bash
# 安装 Signpath CLI
# 下载：https://signpath.io/docs/cli

# 签名
signpath sign \
  --org-id YOUR_ORG_ID \
  --project irisnote \
  --policy release-signing \
  target/release/irisnote.exe
```

## 🔍 验证签名

### Windows

右键点击 `irisnote.exe` → 属性 → 数字签名：

应显示：
- 签名者：Signpath Certificate
- 时间戳：签名时间
- 验证：✅ 此数字签名正常

### 命令行

```bash
# Windows
signtool verify /pa irisnote.exe

# PowerShell
Get-AuthenticodeSignature irisnote.exe
```

## 📊 Signpath 免费计划限制

- 每月签名次数：100 次
- 文件大小限制：100 MB
- 并发签名：1 个
- 证书类型：Standard Code Signing

对于开源项目完全够用！

## 🛡️ 安全建议

1. **API Token 保护**
   - 不要提交到代码仓库
   - 使用 GitHub Secrets
   - 定期轮换 Token

2. **签名策略**
   - 只在 Release 时签名
   - 限制签名次数
   - 启用审计日志

3. **证书管理**
   - 由 Signpath 托管证书
   - 启用时间戳
   - 定期更新证书

## 🆘 常见问题

### Q: 签名失败

**检查**：
1. API Token 是否正确
2. 组织 ID 和项目 Slug 是否匹配
3. 签名策略是否配置
4. 文件大小是否超限

### Q: 签名超时

**解决方案**：
- Signpath 免费计划并发限制
- 等待前一个签名完成
- 或升级付费计划

### Q: 证书不受信任

**说明**：
- Signpath 使用标准代码签名证书
- 首次使用需要建立信任
- Windows SmartScreen 可能仍有警告（属于正常）

## 📚 相关文档

- Signpath 官方文档：https://signpath.io/docs
- GitHub Action 集成：https://github.com/signpath/github-action
- 代码签名最佳实践：https://signpath.io/docs/best-practices

## ✅ 配置清单

使用 Signpath 前请确认：

- [ ] 注册 Signpath 账户
- [ ] 创建组织和项目
- [ ] 配置签名策略
- [ ] 获取 API Token
- [ ] 配置 GitHub Secrets
- [ ] 更新 GitHub Actions 工作流
- [ ] 测试签名流程

---

**配置完成后，每次 Release 都会自动签名！**
