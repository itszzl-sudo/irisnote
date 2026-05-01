# IrisNote

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-green.svg)]()

A smart text editor built with Rust that features automatic file type detection, intelligent file naming, and multi-mode preview capabilities.

**[中文文档](README_CN.md)**

## ✨ Features

### 🎯 Smart File Detection
- **Auto Type Recognition**: Intelligently identifies file types based on content and extension
- **Intelligent Naming**: Recommends appropriate filenames and extensions based on content analysis
- Extracts function names, class names, titles, and other key information for naming

### 👁️ Multi-Mode Preview
- **Markdown**: Real-time rendering with support for headers, lists, code blocks, quotes, etc.
- **SVG**: Instant image rendering to visualize graphics directly
- **Code Files**: Syntax highlighting with keyword, string, and comment coloring

### 📁 File Management
- **Recent Files**: Automatically remembers recently opened file locations
- **Quick Access**: Dropdown menu for fast file selection from history
- **Persistent Config**: Automatically saves user preferences

### 🔧 Windows Integration
- **File Association**: One-click association for all supported file types
- **Selective Association**: Choose specific formats to associate
- **Self-Signing**: Automatic certificate creation and executable signing
- **Custom Icon**: Support for custom application icons

## 📦 Supported File Types

| Category | Extensions | Special Features |
|----------|------------|------------------|
| Text | `.txt` | - |
| Markdown | `.md` | Live preview rendering |
| Rust | `.rs` | Syntax highlighting |
| Python | `.py` | Syntax highlighting |
| JavaScript | `.js` | Syntax highlighting |
| TypeScript | `.ts` | Syntax highlighting |
| HTML | `.html`, `.htm` | Syntax highlighting |
| CSS | `.css` | Syntax highlighting |
| JSON | `.json` | Syntax highlighting |
| XML | `.xml` | Syntax highlighting |
| YAML | `.yaml`, `.yml` | Syntax highlighting |
| TOML | `.toml` | Syntax highlighting |
| SVG | `.svg` | Image rendering preview |
| C | `.c` | Syntax highlighting |
| C++ | `.cpp`, `.cc`, `.cxx` | Syntax highlighting |
| Java | `.java` | Syntax highlighting |
| Go | `.go` | Syntax highlighting |

## 🚀 Quick Start

### Prerequisites

- Rust 1.70 or higher
- Windows: Visual Studio Build Tools
- Linux: build-essential
- macOS: Xcode Command Line Tools

### Install Rust

```bash
# Visit https://rustup.rs/
# Or use the following command
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build

#### Debug Build (Fast, for testing)

```bash
cd irisnote
cargo build
./target/debug/irisnote  # Linux/macOS
.\target\debug\irisnote.exe  # Windows
```

#### Release Build (Optimized, for production)

```bash
cd irisnote
cargo build --release
./target/release/irisnote  # Linux/macOS
.\target\release\irisnote.exe  # Windows
```

### Windows Signing

**Option 1: Signpath (Recommended for Release)**

IrisNote uses Signpath free code signing service for professional signing.

```bash
# Signpath is automatically used in GitHub Actions
# For local signing, configure Signpath first:
# See SIGNPATH_SETUP.md for details

.\build-and-sign-signpath.bat
```

**Option 2: Self-Signing (Development)**

```bash
.\build-and-sign.bat
```

This will automatically:
1. Compile the release version
2. Create a self-signed certificate
3. Sign the executable

### One-Click Launch

```bash
.\run.bat  # Windows
```

## 📖 Usage Guide

### Basic Operations

1. **New File**: `File` → `New`
2. **Open File**: `File` → `Open` or select from recent files list
3. **Save File**: `File` → `Save` (automatically uses smart naming)
4. **Save As**: `File` → `Save As` (customize filename)

### Preview Modes

Switch via `View` menu:
- **Editor**: Plain text editing mode
- **Show Preview**: Enable split-screen preview
- **Markdown Preview**: Render Markdown format
- **Image Preview**: Render SVG graphics
- **Syntax Highlighting**: Code syntax coloring

### File Association (Windows)

In the `Tools` menu:
- **Associate All Supported Types**: One-click association for all formats
- **Selective Association**: Choose specific file types to associate

After association, double-click files to open them with this editor.

## 🎨 Smart Filename Examples

| Content Pattern | Suggested Filename | Extension |
|----------------|-------------------|-----------|
| `fn main() { ... }` | `main.rs` | `.rs` |
| `def process():` | `process.py` | `.py` |
| `class MyClass:` | `MyClass.py` | `.py` |
| `function init()` | `init.js` | `.js` |
| `# Title` | `title.md` | `.md` |
| `<svg>...</svg>` | `untitled.svg` | `.svg` |
| `{"name": "value"}` | `data.json` | `.json` |
| `[package]` | `config.toml` | `.toml` |

## 📂 Project Structure

```
smart-text-editor/
├── Cargo.toml              # Dependencies configuration
├── build.rs                # Build script (icon setup)
├── src/
│   ├── main.rs             # Main entry point
│   ├── file_type.rs        # File type detection & suggestion
│   ├── preview.rs          # Preview rendering (Markdown/SVG/Syntax)
│   ├── config.rs           # Configuration management
│   └── file_association.rs # Windows file association
├── examples/               # Example files
│   ├── demo.md             # Markdown example
│   ├── demo.svg            # SVG example
│   ├── demo.json           # JSON example
│   ├── demo.rs             # Rust code example
│   └── demo.py             # Python code example
├── build-and-sign.bat      # Windows build & sign script
├── run.bat                 # Quick launch script
├── LICENSE                 # Apache 2.0 License
├── README.md               # English documentation
├── README_CN.md            # Chinese documentation
├── BUILD.md                # Detailed build guide
└── QUICKSTART.md           # Quick start guide
```

## ⚙️ Configuration

Config file location:
- Windows: `%APPDATA%\irisnote\config.json`
- Linux: `~/.config/irisnote/config.json`
- macOS: `~/Library/Application Support/irisnote/config.json`

Configuration content:
```json
{
  "recent_paths": [
    "/path/to/recent/file1.md",
    "/path/to/recent/file2.rs"
  ],
  "default_save_dir": "/path/to/default",
  "auto_detect_type": true
}
```

## 🔨 Development

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Run Tests

```bash
cargo test
```

### Code Quality

```bash
cargo clippy
cargo fmt --check
```

## 🐛 Troubleshooting

### Build Errors

**Linker Error (Windows)**:
```bash
winget install Microsoft.VisualStudio.2022.BuildTools
```

**Slow Dependency Download (China)**:
```bash
# Add to %USERPROFILE%\.cargo\config:
[source.crates-io]
replace-with = 'tuna'

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
```

### Runtime Errors

**Missing DLL (Windows)**:
```bash
winget install Microsoft.VCRedist.2015+.x64
```

**Icon Not Displaying**:
- Verify the icon file path is correct
- Rebuild the project

## 🤝 Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Create a Pull Request

### Contribution Guidelines

- Follow Rust code conventions
- Add necessary tests
- Update relevant documentation
- Ensure all tests pass

## 📄 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

This project uses the following open-source libraries:

- [egui](https://github.com/emilk/egui) - Immediate mode GUI framework
- [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) - Markdown parser
- [resvg](https://github.com/RazrFalcon/resvg) - SVG rendering library
- [syntect](https://github.com/trishume/syntect) - Syntax highlighting library

## 📮 Contact

- Issue Tracker: [GitHub Issues](https://github.com/itszzl-sudo/irisnote/issues)
- Feature Requests: [GitHub Discussions](https://github.com/itszzl-sudo/irisnote/discussions)

## 🗺️ Roadmap

- [ ] Support for more file types
- [ ] Theme customization
- [ ] Plugin system
- [ ] Collaborative editing
- [ ] Cloud sync support

---

**⭐ If this project helps you, please give it a Star!**
