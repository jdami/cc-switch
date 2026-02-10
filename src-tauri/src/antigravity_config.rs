use std::path::PathBuf;
use crate::config::get_home_dir;

/// 获取 Antigravity 配置目录路径 (~/.gemini/antigravity)
pub fn get_antigravity_dir() -> PathBuf {
    if let Some(custom) = crate::settings::get_antigravity_override_dir() {
        return custom;
    }
    get_home_dir().join(".gemini").join("antigravity")
}

/// 获取 Antigravity MCP 配置文件路径
pub fn get_antigravity_mcp_path() -> PathBuf {
    get_antigravity_dir().join("mcp_config.json")
}
