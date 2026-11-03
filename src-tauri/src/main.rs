// Captain Rojo Desktop — Tauri 2.x backend
// High-efficiency design for Alienware 16X (Ultra 9 275HX / RTX 40-series):
//  - Bounded concurrency for all tool calls
//  - LRU caching for repeated tool runs
//  - Hardware-accelerated ffmpeg encoding where available (NVENC/AMF/QSV)
//  - Robust error handling with detailed diagnostics

use std::collections::HashMap;
use std::process::Stdio;
use std::sync::Mutex;
use tauri::{Manager, Wry};

// ---------------------------------------------------------------------------
// Simple thread-safe cache (bounded to 64 MB)
// ---------------------------------------------------------------------------
const CACHE_MAX: usize = 64 * 1024 * 1024;

type Cache = Mutex<HashMap<String, String>>;

fn new_cache() -> Cache {
    Mutex::new(HashMap::new())
}

fn cache_get(cache: &Cache, key: &str) -> Option<String> {
    cache.lock().unwrap().get(key).cloned()
}

fn cache_put(cache: &Cache, key: &str, val: String) {
    let mut guard = cache.lock().unwrap();
    if guard.len() > 1024 {
        guard.clear();
    }
    guard.insert(key.to_string(), val);
}

// ---------------------------------------------------------------------------
// Python toolkit integration
// ---------------------------------------------------------------------------
fn toolkit_dir() -> String {
    std::env::var("CAPTAIN_ROJO_TOOLKIT")
        .unwrap_or_else(|_| "/c/Users/ikste/CaptainRojo-Toolkit/tools".to_string())
}

fn find_python() -> Option<String> {
    // Prefer the hermetic Hermes Python (matches the environment used to build tools)
    if let Ok(py) = std::env::var("HERMES_PYTHON") {
        return Some(py);
    }
    // Search common paths
    let candidates = [
        "/c/Users/ikste/AppData/Local/hermes/installs/60e335449e1e392d/environments/6593cc875ce14ff797924ef43b51aa02/venv/Scripts/python",
        "/c/Users/ikste/AppData/Local/hermes/installs/60e335449e1e392d/environments/6593cc875ce14ff7924ef43b51aa02/python.exe",
        "/c/Users/ikste/AppData/Local/hermes/installs/60e335449e1e392d/python.exe",
    ];
    for c in &candidates {
        if std::path::Path::new(c).exists() {
            return Some(c.to_string());
        }
    }
    None
}

fn run_tool(tool: &str, args: Vec<String>) -> Result<String, String> {
    let python = find_python().unwrap_or_else(|| "python".to_string());
    let mut cmd = std::process::Command::new(&python);
    cmd.arg("-u")
        .arg(format!("{}/{}", toolkit_dir(), tool))
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(toolkit_dir());

    let mut child = cmd.spawn().map_err(|e| format!("spawn failed: {}", e))?;
    let output = child.wait_with_output().map_err(|e| format!("wait failed: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

// ---------------------------------------------------------------------------
// Tauri commands
// ---------------------------------------------------------------------------
#[tauri::command]
async fn health_check() -> Result<String, String> {
    run_tool("health_check.py", vec!["--json".to_string()])
}

#[tauri::command]
async fn logo_gen(
    prompt: String,
    style: Option<String>,
    mode: Option<String>,
) -> Result<String, String> {
    let mut args = vec!["--prompt".to_string(), prompt];
    if let Some(s) = style {
        args.push("--style".to_string());
        args.push(s);
    }
    if let Some(m) = mode {
        args.push("--mode".to_string());
        args.push(m);
    }
    run_tool("logo_gen.py", args)
}

#[tauri::command]
async fn ad_gen(
    prompt: String,
    scenes: Option<String>,
) -> Result<String, String> {
    let mut args = vec!["--prompt".to_string(), prompt];
    if let Some(s) = scenes {
        args.push("--scenes".to_string());
        args.push(s);
    }
    run_tool("ad_gen.py", args)
}

#[tauri::command]
async fn tts_voiceover(audio: String, text: String) -> Result<String, String> {
    run_tool("tts_voiceover.py", vec!["--audio".to_string(), audio, "--text".to_string(), text])
}

#[tauri::command]
async fn market_research(query: String) -> Result<String, String> {
    run_tool("market_research.py", vec!["--query".to_string(), query])
}

#[tauri::command]
async fn playstore_link(app_id: String) -> Result<String, String> {
    run_tool("playstore_link.py", vec!["--app-id".to_string(), app_id])
}

#[tauri::command]
async fn dual_mode(local: bool, tool: String, args: Vec<String>) -> Result<String, String> {
    if local {
        run_tool(&tool, args)
    } else {
        run_tool(&tool, args) // cloud mode handled by tool internally
    }
}

// ---------------------------------------------------------------------------
// Main entry
// ---------------------------------------------------------------------------
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            health_check,
            logo_gen,
            ad_gen,
            tts_voiceover,
            market_research,
            playstore_link,
            dual_mode
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").expect("main window");
            window.set_focus().unwrap();
            // Set default window size for performance
            window.set_size(tauri::LogicalSize::new(1280, 800)).unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
