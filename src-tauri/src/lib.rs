// 批注台 · AI 精读 —— 桌面版后端
// 只做一件事:把前端要存的东西读写到用户选定的本地文件夹。
// 所有命令的参数都用单字(path / contents / b64),避开 Tauri 的 snake_case→camelCase 转换坑。

use base64::{engine::general_purpose::STANDARD, Engine as _};
use std::fs;
use std::path::{Path, PathBuf};

fn ensure_parent(p: &Path) -> Result<(), String> {
    if let Some(dir) = p.parent() {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 弹出系统“选择文件夹”对话框,返回所选路径(取消则返回 null)。
/// 用 AsyncFileDialog:同步对话框从 Tauri 命令线程打开在 Windows 上可能卡死。
#[tauri::command]
async fn pick_folder() -> Option<String> {
    rfd::AsyncFileDialog::new()
        .set_title("选择文献库文件夹")
        .pick_folder()
        .await
        .map(|h| h.path().to_string_lossy().into_owned())
}

#[tauri::command]
fn path_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[tauri::command]
fn make_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn read_text(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text(path: String, contents: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    ensure_parent(&p)?;
    fs::write(&p, contents).map_err(|e| e.to_string())
}

/// 读二进制(PDF)→ base64 字符串(前端再解码成 Uint8Array)。
#[tauri::command]
fn read_b64(path: String) -> Result<String, String> {
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    Ok(STANDARD.encode(bytes))
}

/// 前端传 base64 → 解码后写入文件。
#[tauri::command]
fn write_b64(path: String, b64: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    ensure_parent(&p)?;
    let bytes = STANDARD.decode(b64.as_bytes()).map_err(|e| e.to_string())?;
    fs::write(&p, bytes).map_err(|e| e.to_string())
}

/// 删除文件或整个目录(目录递归删除)。不存在则视为成功。
#[tauri::command]
fn remove_path(path: String) -> Result<(), String> {
    let p = Path::new(&path);
    if !p.exists() {
        return Ok(());
    }
    if p.is_dir() {
        fs::remove_dir_all(p).map_err(|e| e.to_string())
    } else {
        fs::remove_file(p).map_err(|e| e.to_string())
    }
}

#[tauri::command]
fn list_dir(path: String) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let entries = fs::read_dir(&path).map_err(|e| e.to_string())?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        names.push(entry.file_name().to_string_lossy().into_owned());
    }
    Ok(names)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            pick_folder,
            path_exists,
            make_dir,
            read_text,
            write_text,
            read_b64,
            write_b64,
            remove_path,
            list_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
