fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        
        let icon_path = r"C:\Users\a\Pictures\iris.png";
        
        if std::path::Path::new(icon_path).exists() {
            if icon_path.ends_with(".ico") {
                res.set_icon(icon_path);
            } else {
                println!("cargo:warning=图标文件 {} 不是 ICO 格式，跳过图标设置", icon_path);
                println!("cargo:warning=请将 PNG 转换为 ICO 格式或使用默认图标");
            }
        } else {
            println!("cargo:warning=图标文件不存在: {}", icon_path);
        }
        
        res.set("ProductName", "IrisNote");
        res.set("FileDescription", "IrisNote - Smart Text Editor with auto-detection");
        res.set("LegalCopyright", "Copyright © 2024 itszzl-sudo");
        res.set("OriginalFilename", "irisnote.exe");
        
        if let Err(e) = res.compile() {
            println!("cargo:warning=资源编译失败: {}", e);
            println!("cargo:warning=继续构建但不包含自定义资源");
        }
    }
}
