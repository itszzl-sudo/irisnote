fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        
        let icon_paths = [
            "iris.ico",
            r"C:\Users\a\Pictures\iris.ico",
            r"C:\Users\a\Pictures\iris.png",
        ];
        
        let mut icon_found = false;
        for icon_path in &icon_paths {
            if std::path::Path::new(icon_path).exists() {
                if icon_path.ends_with(".ico") {
                    res.set_icon(icon_path);
                    icon_found = true;
                    println!("cargo:warning=使用图标: {}", icon_path);
                    break;
                }
            }
        }
        
        if !icon_found {
            println!("cargo:warning=未找到 ICO 格式图标，使用默认图标");
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
