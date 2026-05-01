# IrisNote GitHub 发布指南

## 🎯 项目信息

- **项目名称**: IrisNote
- **GitHub 地址**: https://github.com/itszzl-sudo/irisnote
- **许可证**: Apache 2.0
- **版本**: v0.1.0

## 📋 发布前准备清单

### ✅ 已完成
- [x] 项目重命名为 IrisNote
- [x] 更新所有文档中的项目名称和地址
- [x] 更新 Cargo.toml 配置
- [x] 更新作者信息为 itszzl-sudo
- [x] 更新 GitHub 链接
- [x] Apache 2.0 许可证文件
- [x] NOTICE 文件
- [x] 双语文档 (中英文)
- [x] CI/CD 配置
- [x] Issue 和 PR 模板

### 🔄 待完成
- [ ] 构建可执行文件
- [ ] 推送到 GitHub
- [ ] 创建 Release

## 🚀 发布步骤

### 步骤 1: 构建项目

#### Windows

```bash
# Debug 版本（快速测试）
cargo build

# Release 版本（生产使用）
cargo build --release

# 或使用自动脚本（包含签名）
.\build-and-sign.bat
```

构建产物：
- Debug: `target\debug\irisnote.exe`
- Release: `target\release\irisnote.exe`

### 步骤 2: 初始化 Git 并提交

#### 方式 A: 使用自动化脚本（推荐）

```bash
.\publish.bat
```

脚本会自动：
1. 检查 Git 状态
2. 添加所有文件
3. 创建提交
4. 配置远程仓库
5. 推送到 GitHub
6. 创建标签

#### 方式 B: 手动操作

```bash
# 初始化 Git（如果还没有）
git init

# 添加所有文件
git add .

# 创建提交
git commit -m "feat: initial release v0.1.0

- IrisNote: Smart text editor with auto-detection
- Markdown and SVG preview support
- Syntax highlighting for 15+ languages
- Intelligent file naming suggestions
- Windows file association
- Apache 2.0 license

🤖 Generated with CodeArts"

# 添加远程仓库
git remote add origin https://github.com/itszzl-sudo/irisnote.git

# 推送到 GitHub
git push -u origin main

# 创建标签
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0
```

### 步骤 3: 创建 GitHub Release

1. **访问仓库**
   ```
   https://github.com/itszzl-sudo/irisnote
   ```

2. **创建 Release**
   - 点击右侧 "Releases"
   - 点击 "Draft a new release"
   - 选择标签: `v0.1.0`
   - 填写标题: `IrisNote v0.1.0`

3. **填写 Release 说明**

```markdown
# IrisNote v0.1.0

首次发布！🎉

## ✨ 功能特性

### 智能文件检测
- ✅ 自动识别 19 种文件类型
- ✅ 根据内容推荐文件名和扩展名
- ✅ 提取关键信息用于命名

### 多模式预览
- ✅ Markdown 实时渲染
- ✅ SVG 图片实时渲染
- ✅ 15+ 语言语法高亮

### 文件管理
- ✅ 最近文件列表
- ✅ 配置持久化
- ✅ 快速访问历史文件

### Windows 集成
- ✅ 文件关联
- ✅ 自签名支持
- ✅ 自定义图标

## 📦 支持的文件类型

- 文本: txt, md
- 编程语言: rs, py, js, ts, c, cpp, java, go
- 标记语言: html, css, json, xml, yaml, toml
- 图形: svg

## 🚀 快速开始

### Windows
1. 下载 `irisnote-v0.1.0-windows-x64.zip`
2. 解压到任意目录
3. 运行 `irisnote.exe`

### 从源码构建
```bash
git clone https://github.com/itszzl-sudo/irisnote.git
cd irisnote
cargo build --release
```

## 📚 文档

- [README (English)](README.md)
- [中文文档](README_CN.md)
- [构建指南](BUILD.md)
- [快速入门](QUICKSTART.md)

## 🤝 贡献

欢迎贡献！请查看 [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 许可证

Apache License 2.0 - 详见 [LICENSE](LICENSE)

---

**完整更新日志**: https://github.com/itszzl-sudo/irisnote/commits/v0.1.0
```

4. **上传构建产物**

打包命令：
```bash
# Windows
7z a irisnote-v0.1.0-windows-x64.zip target\release\irisnote.exe README*.md LICENSE

# Linux
tar czf irisnote-v0.1.0-linux-x64.tar.gz target/release/irisnote README*.md LICENSE

# macOS
tar czf irisnote-v0.1.0-macos-x64.tar.gz target/release/irisnote README*.md LICENSE
```

上传文件：
- `irisnote-v0.1.0-windows-x64.zip`
- `irisnote-v0.1.0-linux-x64.tar.gz` (如有)
- `irisnote-v0.1.0-macos-x64.tar.gz` (如有)

5. **发布 Release**
   - 点击 "Publish release"

## 📝 GitHub 仓库设置

### 基本信息

访问: https://github.com/itszzl-sudo/irisnote/settings

1. **About 部分**
   - Description: `Smart text editor with auto-detection, preview, and intelligent naming`
   - Website: 留空或添加文档网站
   - Topics: 
     - rust
     - text-editor
     - markdown
     - svg
     - syntax-highlighting
     - gui
     - egui
     - apache2

2. **功能设置**
   - ✅ Issues
   - ✅ Discussions
   - ✅ Wiki (可选)
   - ✅ Projects (可选)

3. **分支保护** (可选)
   - Settings → Branches
   - Add rule for `main`
   - Require pull request reviews

### 社区设置

1. **Issue 模板** - 已配置 ✅
2. **PR 模板** - 已配置 ✅
3. **CODEOWNERS** - 已配置 ✅

### 安全设置

1. **Security policy** - 已配置 ✅
2. **Dependabot** (可选)
   ```yaml
   # .github/dependabot.yml
   version: 2
   updates:
     - package-ecosystem: "cargo"
       directory: "/"
       schedule:
         interval: "weekly"
   ```

## 🌐 推广建议

### 徽章

项目已包含以下徽章：
- License
- Rust version
- Platform

可添加更多：
```markdown
[![GitHub stars](https://img.shields.io/github/stars/itszzl-sudo/irisnote)]()
[![GitHub issues](https://img.shields.io/github/issues/itszzl-sudo/irisnote)]()
[![GitHub release](https://img.shields.io/github/release/itszzl-sudo/irisnote)]()
```

### 分享渠道

- Rust 社区
  - r/rust
  - users.rust-lang.org
  - This Week in Rust

- 中文社区
  - Rust 中文社区
  - V2EX
  - 知乎

### 社交媒体

- Twitter
- 微博
- LinkedIn

## 📊 发布后监控

### 关注指标

1. **GitHub 统计**
   - Stars
   - Forks
   - Issues
   - Downloads

2. **用户反馈**
   - Issues
   - Discussions
   - Pull Requests

3. **持续改进**
   - 回复 Issue
   - 合并 PR
   - 发布更新

### 定期维护

- 更新依赖
- 修复 Bug
- 添加新功能
- 更新文档

## 🎉 发布检查清单

发布前确认：

- [ ] 项目构建成功
- [ ] 所有测试通过
- [ ] 文档完整准确
- [ ] 版本号正确
- [ ] 许可证文件存在
- [ ] README 显示正常
- [ ] 截图/示例完整
- [ ] Release 说明准备好
- [ ] 构建产物已打包

发布后确认：

- [ ] GitHub 页面正常
- [ ] Release 可下载
- [ ] 链接可访问
- [ ] 文档可读
- [ ] Issue 模板可用
- [ ] PR 模板可用

## 🆘 常见问题

### Q: 推送失败
A: 检查 Git 认证
```bash
# 使用 Personal Access Token
git remote set-url origin https://<token>@github.com/itszzl-sudo/irisnote.git

# 或使用 SSH
git remote set-url origin git@github.com:itszzl-sudo/irisnote.git
```

### Q: 构建失败
A: 查看 `BUILD.md` 故障排除部分

### Q: Release 文件太大
A: 使用 UPX 压缩
```bash
upx --best target/release/irisnote.exe
```

---

## 📞 需要帮助？

- GitHub Issues: https://github.com/itszzl-sudo/irisnote/issues
- GitHub Discussions: https://github.com/itszzl-sudo/irisnote/discussions

祝发布顺利！🚀
