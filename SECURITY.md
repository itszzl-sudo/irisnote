# 安全策略

## 支持的版本

目前支持安全更新的版本：

| 版本 | 支持 |
| --- | --- |
| 0.1.x | ✅ |
| < 0.1.0 | ❌ |

## 报告安全漏洞

我们非常重视安全问题。如果您发现了安全漏洞，请**不要**通过公开的 Issue 报告。

### 报告方式

1. **GitHub Security Advisories**（推荐）
   - 访问 [Security Advisories](https://github.com/yourusername/smart-text-editor/security/advisories)
   - 点击 "Report a vulnerability"
   - 填写详细信息

2. **邮件**
   - 发送邮件至: irisverse@outlook.com
   - 主题: [SECURITY] IrisNote Vulnerability
   - 包含漏洞详情

### 报告内容

请包含以下信息：

- **漏洞类型**: 例如 XSS、代码注入、路径遍历等
- **影响范围**: 受影响的功能和版本
- **复现步骤**: 详细说明如何复现漏洞
- **概念验证**: 如果可能，提供 PoC 代码或截图
- **潜在影响**: 漏洞可能造成的影响
- **建议修复**: 如果您有修复建议

### 响应时间

- **确认**: 1-3 个工作日
- **初步评估**: 3-5 个工作日
- **修复计划**: 根据严重程度决定
  - 严重: 7 天内
  - 高危: 14 天内
  - 中危: 30 天内
  - 低危: 下个版本

## 安全最佳实践

### 对于用户

1. **保持更新**
   - 使用最新版本
   - 关注安全公告

2. **配置安全**
   - 不要打开不受信任的文件
   - 警慎处理来自网络的文件
   - 定期清理最近文件列表

3. **权限管理**
   - 不要以管理员权限运行（除非必要）
   - 注意文件关联的权限

### 对于开发者

1. **代码安全**
   - 遵循 Rust 安全最佳实践
   - 避免 `unwrap()` 和 `expect()`，使用错误处理
   - 验证所有用户输入
   - 使用安全的 API

2. **依赖安全**
   ```bash
   # 定期检查依赖安全
   cargo audit
   ```
   
   - 保持依赖更新
   - 定期运行 `cargo audit`
   - 在 CI/CD 中集成安全检查

3. **文件操作**
   - 验证文件路径
   - 防止路径遍历攻击
   - 限制文件大小
   - 处理符号链接

4. **内存安全**
   - 利用 Rust 的内存安全保证
   - 避免 `unsafe` 代码（除非必要）
   - 正确使用 `unsafe` 块（需要安全注释）

## 已知安全考虑

### 文件处理
- SVG 渲染可能包含外部资源引用
- Markdown 预览可能包含 HTML 标签
- 文件关联需要注册表权限

### 缓解措施
- SVG 渲染禁用外部资源加载
- Markdown 预览转义 HTML
- 文件关联需要用户确认

## 安全更新历史

| 日期 | 版本 | 漏洞类型 | 严重程度 | 说明 |
|------|------|---------|---------|------|
| - | - | - | - | 无已知安全漏洞 |

## 安全配置

### 推荐配置

```json
{
  "security": {
    "max_file_size": 10485760,
    "allow_external_resources": false,
    "sanitize_html": true,
    "validate_paths": true
  }
}
```

### 配置说明

- `max_file_size`: 最大文件大小（字节），默认 10MB
- `allow_external_resources`: 是否允许外部资源，默认 false
- `sanitize_html`: 是否清理 HTML，默认 true
- `validate_paths`: 是否验证路径，默认 true

## 联系方式

- **安全团队**: irisverse@outlook.com
- **项目负责人**: @yourusername
- **安全公告**: [GitHub Security](https://github.com/yourusername/smart-text-editor/security)

---

感谢您帮助保持 Smart Text Editor 的安全！
