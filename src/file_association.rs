#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[cfg(target_os = "windows")]
use std::ffi::OsStr;
#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

#[cfg(target_os = "windows")]
extern "system" {
    fn SendMessageTimeoutW(
        hwnd: *mut std::ffi::c_void,
        msg: u32,
        wparam: usize,
        lparam: *const u16,
        flags: u32,
        timeout: u32,
        result: *mut usize,
    ) -> isize;
}

#[cfg(target_os = "windows")]
const HWND_BROADCAST: *mut std::ffi::c_void = 0xffff as *mut std::ffi::c_void;
#[cfg(target_os = "windows")]
const WM_SETTINGCHANGE: u32 = 0x001A;
#[cfg(target_os = "windows")]
const SMTO_ABORTIFHUNG: u32 = 0x0002;

#[cfg(target_os = "windows")]
fn broadcast_settings_change() {
    let environment: Vec<u16> = OsStr::new("Environment")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    unsafe {
        let mut result: usize = 0;
        SendMessageTimeoutW(
            HWND_BROADCAST,
            WM_SETTINGCHANGE,
            0,
            environment.as_ptr(),
            SMTO_ABORTIFHUNG,
            5000,
            &mut result,
        );
    }
}

#[cfg(target_os = "windows")]
pub fn register_extension(extension: &str) -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_path_str = exe_path.to_string_lossy();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let prog_id = format!("IrisNote.{}", extension);

    let software = hkcu
        .create_subkey("Software\\Classes")
        .map_err(|e| e.to_string())?
        .0;

    let prog_key = software
        .create_subkey(&prog_id)
        .map_err(|e| e.to_string())?
        .0;
    prog_key
        .set_value("", &format!("IrisNote {} File", extension.to_uppercase()))
        .map_err(|e| e.to_string())?;

    let shell_key = prog_key
        .create_subkey("shell\\open\\command")
        .map_err(|e| e.to_string())?
        .0;
    let cmd = format!("\"{}\" \"%1\"", exe_path_str);
    shell_key.set_value("", &cmd).map_err(|e| e.to_string())?;

    let icon_key = prog_key
        .create_subkey("DefaultIcon")
        .map_err(|e| e.to_string())?
        .0;
    icon_key
        .set_value("", &format!("{},0", exe_path_str))
        .map_err(|e| e.to_string())?;

    let ext_key = software
        .create_subkey(extension)
        .map_err(|e| e.to_string())?
        .0;
    ext_key.set_value("", &prog_id).map_err(|e| e.to_string())?;

    broadcast_settings_change();

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn register_all_extensions() -> Result<(), String> {
    let extensions = crate::file_type::get_supported_extensions();

    for ext in extensions {
        if let Err(e) = register_extension(&ext) {
            return Err(format!("注册 {} 失败: {}", ext, e));
        }
    }
    
    if let Err(e) = register_context_menu() {
        return Err(format!("注册右键菜单失败: {}", e));
    }
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn register_context_menu() -> Result<(), String> {
    let exe_path = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_path_str = exe_path.to_string_lossy();

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let classes = hkcu
        .create_subkey("Software\\Classes\\*\\shell\\IrisNote")
        .map_err(|e| e.to_string())?
        .0;
    
    classes
        .set_value("", &"用 IrisNote 打开")
        .map_err(|e| e.to_string())?;
    
    classes
        .set_value("Icon", &format!("\"{}\",0", exe_path_str))
        .map_err(|e| e.to_string())?;

    let command_key = classes
        .create_subkey("command")
        .map_err(|e| e.to_string())?
        .0;
    let cmd = format!("\"{}\" \"%1\"", exe_path_str);
    command_key.set_value("", &cmd).map_err(|e| e.to_string())?;

    broadcast_settings_change();

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn unregister_context_menu() -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    if let Ok(classes) = hkcu.open_subkey_with_flags("Software\\Classes\\*\\shell", KEY_ALL_ACCESS) {
        let _ = classes.delete_subkey_all("IrisNote");
    }

    broadcast_settings_change();

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn unregister_extension(extension: &str) -> Result<(), String> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);

    let prog_id = format!("IrisNote.{}", extension);

    if let Ok(classes) = hkcu.open_subkey_with_flags("Software\\Classes", KEY_ALL_ACCESS) {
        let _ = classes.delete_subkey_all(&prog_id);
        let _ = classes.delete_subkey_all(extension);
    }

    broadcast_settings_change();

    Ok(())
}

#[cfg(target_os = "windows")]
pub fn unregister_all_extensions() -> Result<(), String> {
    let extensions = crate::file_type::get_supported_extensions();

    for ext in extensions {
        if let Err(e) = unregister_extension(&ext) {
            return Err(format!("取消注册 {} 失败: {}", ext, e));
        }
    }
    
    let _ = unregister_context_menu();
    
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn is_extension_registered(extension: &str) -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let prog_id = format!("IrisNote.{}", extension);
    hkcu.open_subkey(format!("Software\\Classes\\{}", prog_id))
        .is_ok()
}

#[cfg(target_os = "windows")]
pub fn get_registered_extensions() -> Vec<String> {
    let all_extensions = crate::file_type::get_supported_extensions();
    all_extensions
        .into_iter()
        .filter(|ext| is_extension_registered(ext))
        .collect()
}

#[cfg(target_os = "windows")]
pub fn get_unregistered_extensions() -> Vec<String> {
    let all_extensions = crate::file_type::get_supported_extensions();
    all_extensions
        .into_iter()
        .filter(|ext| !is_extension_registered(ext))
        .collect()
}

#[cfg(not(target_os = "windows"))]
pub fn register_extension(_extension: &str) -> Result<(), String> {
    Err("文件关联仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn register_all_extensions() -> Result<(), String> {
    Err("文件关联仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn unregister_extension(_extension: &str) -> Result<(), String> {
    Err("文件关联仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn unregister_all_extensions() -> Result<(), String> {
    Err("文件关联仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn register_context_menu() -> Result<(), String> {
    Err("右键菜单集成仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn unregister_context_menu() -> Result<(), String> {
    Err("右键菜单集成仅在 Windows 上可用".to_string())
}

#[cfg(not(target_os = "windows"))]
pub fn is_extension_registered(_extension: &str) -> bool {
    false
}

#[cfg(not(target_os = "windows"))]
pub fn get_registered_extensions() -> Vec<String> {
    vec![]
}

#[cfg(not(target_os = "windows"))]
pub fn get_unregistered_extensions() -> Vec<String> {
    crate::file_type::get_supported_extensions()
}