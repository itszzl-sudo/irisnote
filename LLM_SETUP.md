# LLM 服务配置指南

## 📋 概述

IrisNote 集成了本地大语言模型（LLM）服务，用于智能总结文件内容，提供更准确的文件标题建议。

### 模型信息
- **模型**: Qwen2.5-Coder-0.5B-Instruct
- **格式**: GGUF (Q4_K_M 量化)
- **大小**: ~400 MB
- **用途**: 总结文件内容，生成标题建议
- **优势**: 
  - ✅ 本地运行，隐私安全
  - ✅ 无需联网
  - ✅ 免费使用
  - ✅ 轻量级（0.5B 参数）

## 🔧 配置步骤

### 步骤 1: 下载模型文件

模型已存在于：
```
C:\Users\a\Downloads\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf
```

如果需要重新下载：
1. 访问：https://huggingface.co/Qwen/Qwen2.5-Coder-0.5B-Instruct-GGUF
2. 下载 `qwen2.5-coder-0.5b-instruct-q4_k_m.gguf`
3. 放到 `C:\Users\a\Downloads\` 目录

### 步骤 2: 安装 llama.cpp

#### 方式 A: 使用预编译版本（推荐）

1. 访问：https://github.com/ggerganov/llama.cpp/releases
2. 下载最新的 Windows 版本（如 `llama-*-win-x64.zip`）
3. 解压到项目的 `llama.cpp` 目录

#### 方式 B: 从源码编译

```bash
# 克隆仓库
git clone https://github.com/ggerganov/llama.cpp
cd llama.cpp

# 编译
mkdir build
cd build
cmake .. -DCMAKE_BUILD_TYPE=Release
cmake --build . --config Release

# 复制可执行文件
copy Release\*.exe ..
```

#### 方式 C: 使用包管理器

```bash
# 使用 scoop
scoop install llama.cpp

# 或使用 chocolatey
choco install llama.cpp
```

### 步骤 3: 启动 LLM 服务

#### 手动启动

```bash
.\start-llm-server.bat
```

服务将在 `http://localhost:8080` 启动。

#### 自动启动（Windows 服务）

创建 Windows 服务：

```powershell
# 创建服务
sc.exe create "IrisNote-LLM" binPath= "\"C:\path\to\start-llm-server.bat\"" start= auto

# 启动服务
sc.exe start IrisNote-LLM
```

### 步骤 4: 验证服务

访问：http://localhost:8080/health

或使用命令：
```bash
curl http://localhost:8080/health
```

应返回：`{"status": "ok"}`

## 🚀 使用方式

### 自动使用

当现有方式无法确定文件标题时，IrisNote 会自动调用 LLM 服务：

1. 检测文件类型失败
2. 无法提取关键信息（函数名、类名等）
3. 文件内容需要总结

### 手动触发

在编辑器中：
- 点击 "工具" → "AI 智能命名"
- 或使用快捷键（可配置）

### API 调用

```rust
use llm_service::LLMService;

let llm = LLMService::new();

// 总结内容
let title = llm.summarize_content(
    "This is a document about machine learning algorithms..."
).await?;

println!("建议标题: {}", title);
// 输出: "ml algorithms overview"
```

## ⚙️ 配置选项

编辑 `config.json`：

```json
{
  "llm": {
    "enabled": true,
    "model_path": "C:\\Users\\a\\Downloads\\Qwen2.5-Coder-0.5B-Instruct-Q4_K_M.gguf",
    "server_url": "http://localhost:8080",
    "max_tokens": 50,
    "temperature": 0.7
  }
}
```

### 配置说明

| 参数 | 说明 | 默认值 |
|------|------|--------|
| `enabled` | 是否启用 LLM | `true` |
| `model_path` | 模型文件路径 | 见上方 |
| `server_url` | LLM 服务地址 | `http://localhost:8080` |
| `max_tokens` | 最大生成令牌数 | `50` |
| `temperature` | 生成温度 (0-1) | `0.7` |

### 性能调优

#### GPU 加速

如果有 NVIDIA GPU：

```bash
# 启动时添加 GPU 层
llama-server -m model.gguf --n-gpu-layers 35 ...
```

#### 多线程

```bash
# 使用更多线程
llama-server -m model.gguf --threads 8 ...
```

#### 批处理

```bash
# 启用连续批处理
llama-server -m model.gguf --cont-batching ...
```

## 📊 性能指标

### 模型性能（CPU）

| 硬件 | 推理速度 | 内存使用 |
|------|---------|---------|
| 4 核心 CPU | ~30 tokens/s | ~600 MB |
| 8 核心 CPU | ~50 tokens/s | ~600 MB |
| GPU (RTX 3060) | ~200 tokens/s | ~800 MB |

### 响应时间

- 冷启动（首次）：~2-5 秒
- 热请求：~0.1-0.5 秒
- 内存占用：~600 MB

## 🔍 工作原理

### 流程图

```
文件内容
    ↓
现有方式检测
    ↓
  失败？
    ↓
调用 LLM 服务
    ↓
发送内容片段（前 500 字符）
    ↓
LLM 总结核心内容
    ↓
生成标题建议（2-5 词）
    ↓
返回给用户
```

### Prompt 设计

```
Please provide a concise title (2-5 words) 
that summarizes the main topic of this content:

<content>

Title:
```

## 🛠️ 故障排除

### 服务无法启动

**检查**：
1. 模型文件是否存在
2. llama-server 是否在 PATH 中
3. 端口 8080 是否被占用

**解决**：
```bash
# 检查端口
netstat -ano | findstr :8080

# 更换端口
set SERVER_PORT=8081
.\start-llm-server.bat
```

### 请求超时

**原因**：
- 模型加载慢
- CPU 性能不足
- 并发请求过多

**解决**：
```bash
# 减小上下文
set CTX_SIZE=1024

# 使用 GPU
set N_GPU_LAYERS=35
```

### 生成质量差

**调整**：
```json
{
  "llm": {
    "temperature": 0.5,  // 降低随机性
    "max_tokens": 30     // 限制长度
  }
}
```

## 🔄 替代方案

### 方案 A: 使用其他模型

Qwen 系列：
- Qwen2.5-0.5B：更快，质量稍低
- Qwen2.5-1.5B：平衡
- Qwen2.5-7B：最佳质量，需更多资源

### 方案 B: 使用 OpenAI API

```json
{
  "llm": {
    "provider": "openai",
    "api_key": "sk-...",
    "model": "gpt-3.5-turbo"
  }
}
```

### 方案 C: 使用 Ollama

```bash
# 安装 Ollama
# https://ollama.ai

# 拉取模型
ollama pull qwen2.5:0.5b

# 启动服务
ollama serve
```

## 📚 相关资源

- **Qwen 模型**: https://github.com/QwenLM/Qwen2.5
- **llama.cpp**: https://github.com/ggerganov/llama.cpp
- **GGUF 格式**: https://huggingface.co/docs/hub/en/gguf

## 🌟 最佳实践

### 1. 服务管理

- **开发环境**：手动启动，按需使用
- **生产环境**：作为系统服务自动启动

### 2. 资源配置

- **低配置电脑**：使用 0.5B 模型，CPU 推理
- **中配置电脑**：使用 1.5B 模型，可选 GPU
- **高配置电脑**：使用 7B 模型，GPU 加速

### 3. 成本优化

- 只在必要时调用 LLM
- 缓存生成结果
- 限制内容长度（前 500 字符）

## ✅ 配置检查清单

使用前请确认：

- [ ] 模型文件已下载
- [ ] llama.cpp 已安装
- [ ] LLM 服务已启动
- [ ] 服务健康检查通过
- [ ] IrisNote 配置正确

---

**配置完成后，IrisNote 将具备 AI 智能命名能力！**
