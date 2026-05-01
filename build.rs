fn main() {
    if cfg!(target_os = "windows") {
        let mut res = winres::WindowsResource::new();
        res.set_icon(r"C:\Users\a\Pictures\iris.png");
        res.set("ProductName", "IrisNote");
        res.set("FileDescription", "IrisNote - Smart Text Editor with auto-detection");
        res.compile().unwrap();
    }
}
