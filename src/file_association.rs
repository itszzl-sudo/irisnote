#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[cfg(target_os = "windows")]
pub fn register_extension(extension: &str) {
    let exe_path = std::env::current_exe().unwrap();
    let exe_path_str = exe_path.to_string_lossy();
    
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let prog_id = format!("IrisNote.{}", extension);
    
    if let Ok(software) = hkcu.create_subkey("Software\\Classes") {
        let (classes, _) = software;
        
        if let Ok(prog_key) = classes.create_subkey(&prog_id) {
            let (key, _) = prog_key;
            let _ = key.set_value("", &format!("IrisNote {} File", extension.to_uppercase()));
            
            if let Ok(shell_key) = key.create_subkey("shell\\open\\command") {
                let (cmd_key, _) = shell_key;
                let cmd = format!("\"{}\" \"%1\"", exe_path_str);
                let _ = cmd_key.set_value("", &cmd);
            }
            
            if let Ok(icon_key) = key.create_subkey("DefaultIcon") {
                let (icon, _) = icon_key;
                let _ = icon.set_value("", &format!("{},0", exe_path_str));
            }
        }
        
        if let Ok(ext_key) = classes.create_subkey(extension) {
            let (key, _) = ext_key;
            let _ = key.set_value("", &prog_id);
        }
    }
}

#[cfg(target_os = "windows")]
pub fn register_all_extensions() {
    let extensions = crate::file_type::get_supported_extensions();
    
    for ext in extensions {
        register_extension(&ext);
    }
}

#[cfg(target_os = "windows")]
pub fn unregister_extension(extension: &str) {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let prog_id = format!("IrisNote.{}", extension);
    
    if let Ok(classes) = hkcu.open_subkey("Software\\Classes") {
        let _ = classes.delete_subkey_all(&prog_id);
        let _ = classes.delete_subkey_all(extension);
    }
}

#[cfg(target_os = "windows")]
pub fn is_extension_registered(extension: &str) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    
    let prog_id = format!("IrisNote.{}", extension);
    
    hkcu.open_subkey(format!("Software\\Classes\\{}", prog_id)).is_ok()
}

#[cfg(not(target_os = "windows"))]
pub fn register_extension(_extension: &str) {
    println!("文件关联仅在 Windows 上可用");
}

#[cfg(not(target_os = "windows"))]
pub fn register_all_extensions() {
    println!("文件关联仅在 Windows 上可用");
}

#[cfg(not(target_os = "windows"))]
pub fn unregister_extension(_extension: &str) {
    println!("文件关联仅在 Windows 上可用");
}

#[cfg(not(target_os = "windows"))]
pub fn is_extension_registered(_extension: &str) -> bool {
    false
}
