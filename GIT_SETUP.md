# Git 配置指南 - GitHub 推送设置

## 📋 当前状态

- ✅ Git 已安装: v2.52.0
- ✅ 全局配置: yum (wanquanbuhuime@163.com)
- ✅ 项目仓库已初始化
- ⏳ 需要配置 GitHub 认证

## 🔧 配置步骤

### 步骤 1: 为本项目配置作者信息

由于您的全局配置用于 Gitee，我们将为本项目单独配置：

```bash
cd C:/Users/a/Documents/codearts/smart-text-editor

# 为本项目单独设置用户信息（不影响全局配置）
git config user.name "itszzl-sudo"
git config user.email "irisverse@outlook.com"
```

### 步骤 2: 创建 GitHub Personal Access Token

1. **访问 GitHub 设置**
   ```
   https://github.com/settings/tokens
   ```

2. **创建新 Token**
   - 点击 "Generate new token"
   - 选择 "Generate new token (classic)"
   
3. **配置 Token 权限**
   - Note: `IrisNote Push Access`
   - Expiration: 选择过期时间（建议 90 days）
   - 勾选权限：
     - ✅ `repo` (完整仓库访问)
     - ✅ `workflow` (如果需要 CI/CD)
   
4. **生成并保存 Token**
   - 点击 "Generate token"
   - **⚠️ 重要**: 立即复制 Token（只显示一次！）
   - 保存到安全的地方

Token 格式示例：`ghp_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx`

### 步骤 3: 配置远程仓库

有两种方式：

#### 方式 A: 将 Token 嵌入 URL（推送时无需输入）

```bash
# 添加远程仓库（包含 Token）
git remote add origin https://<YOUR_TOKEN>@github.com/itszzl-sudo/irisnote.git
```

**示例**:
```bash
git remote add origin https://ghp_xxxxxxxxxxxx@github.com/itszzl-sudo/irisnote.git
```

#### 方式 B: 使用 Git Credential Manager（推荐，更安全）

```bash
# 添加远程仓库（不含 Token）
git remote add origin https://github.com/itszzl-sudo/irisnote.git

# 配置凭据存储
git config credential.helper manager
```

首次推送时会提示输入：
- Username: `itszzl-sudo`
- Password: `<YOUR_TOKEN>` (不是 GitHub 密码)

### 步骤 4: 验证配置

```bash
# 查看本地配置
git config --local --list

# 查看远程仓库
git remote -v

# 测试连接
git ls-remote origin
```

## 🚀 推送命令

配置完成后，执行以下命令：

```bash
# 1. 添加所有文件
git add .

# 2. 创建提交
git commit -m "feat: initial release v0.1.0

- IrisNote: Smart text editor with auto-detection
- Markdown and SVG preview support
- Syntax highlighting for 15+ languages
- Intelligent file naming suggestions
- Windows file association
- Apache 2.0 license

🤖 Generated with CodeArts"

# 3. 推送到 GitHub
git push -u origin master

# 4. 创建版本标签
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

## 🔄 如果已配置错误

如果之前配置有误，可以重新配置：

```bash
# 删除远程仓库配置
git remote remove origin

# 重新添加
git remote add origin https://<YOUR_TOKEN>@github.com/itszzl-sudo/irisnote.git
```

## 🛡️ 安全建议

1. **Token 权限**: 只勾选必要的权限
2. **Token 过期**: 设置合理的过期时间
3. **Token 存储**: 
   - ❌ 不要提交到 Git
   - ❌ 不要分享给他人
   - ✅ 使用密码管理器保存
4. **撤销 Token**: 如果泄露，立即到 GitHub 撤销

## 🆘 常见问题

### Q: 推送时提示 "Authentication failed"

**解决方案**:
```bash
# 检查远程 URL
git remote -v

# 重新配置（确保 Token 正确）
git remote set-url origin https://<YOUR_TOKEN>@github.com/itszzl-sudo/irisnote.git
```

### Q: 推送时提示 "Repository not found"

**解决方案**:
1. 确认仓库已创建：https://github.com/itszzl-sudo/irisnote
2. 如果未创建，先在 GitHub 创建仓库

### Q: Token 过期了

**解决方案**:
1. 到 GitHub 生成新 Token
2. 更新远程 URL:
```bash
git remote set-url origin https://<NEW_TOKEN>@github.com/itszzl-sudo/irisnote.git
```

### Q: 想使用 SSH 方式

**解决方案**:
1. 生成 SSH 密钥：
```bash
ssh-keygen -t ed25519 -C "wanquanbuhuime@163.com"
```

2. 添加到 GitHub：
   - 复制公钥：`cat ~/.ssh/id_ed25519.pub`
   - 访问：https://github.com/settings/keys
   - 点击 "New SSH key" 粘贴

3. 使用 SSH URL：
```bash
git remote set-url origin git@github.com:itszzl-sudo/irisnote.git
```

## 📝 快速配置脚本

如果您已经有 Token，运行以下命令：

```bash
# 进入项目目录
cd C:/Users/a/Documents/codearts/smart-text-editor

# 设置本地用户信息
git config user.name "itszzl-sudo"
git config user.email "irisverse@outlook.com"

# 添加远程仓库（请替换 YOUR_TOKEN）
git remote add origin https://YOUR_TOKEN@github.com/itszzl-sudo/irisnote.git

# 验证配置
git remote -v
```

## ✅ 配置验证清单

配置完成后检查：

- [ ] 本地用户名: `git config user.name` → 应显示 `itszzl-sudo`
- [ ] 本地邮箱: `git config user.email` → 应显示 `irisverse@outlook.com`
- [ ] 远程仓库: `git remote -v` → 应显示 origin URL
- [ ] 测试连接: `git ls-remote origin` → 应显示远程分支

全部正确后即可推送！

---

**需要帮助？**
- GitHub Token 文档: https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/creating-a-personal-access-token
- Git 配置文档: https://git-scm.com/book/en/v2/Customizing-Git-Git-Configuration
