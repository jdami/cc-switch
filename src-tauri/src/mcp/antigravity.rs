//! Antigravity MCP 同步和导入模块

use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;

use crate::app_config::{McpApps, McpConfig, McpServer, MultiAppConfig};
use crate::error::AppError;
use crate::antigravity_config::{get_antigravity_dir, get_antigravity_mcp_path};
use crate::config::read_json_file;

use super::validation::{extract_server_spec, validate_server_spec};

fn should_sync_antigravity_mcp() -> bool {
    get_antigravity_dir().exists()
}

/// 读取 Antigravity MCP 配置中的 mcpServers 映射
pub fn read_mcp_servers_map() -> Result<HashMap<String, Value>, AppError> {
    let path = get_antigravity_mcp_path();
    if !path.exists() {
        return Ok(HashMap::new());
    }

    let v: Value = read_json_file(&path)?;
    let map = v.get("mcpServers")
        .and_then(|x| x.as_object())
        .map(|obj| {
            obj.iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        })
        .unwrap_or_default();
    
    Ok(map)
}

/// 将 mcpServers 映射写入 Antigravity 配置文件
pub fn set_mcp_servers_map(map: &HashMap<String, Value>) -> Result<(), AppError> {
    let path = get_antigravity_mcp_path();
    
    // 确保目录存在
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| AppError::io(parent, e))?;
    }

    let mut root = if path.exists() {
        read_json_file::<Value>(&path).unwrap_or(json!({}))
    } else {
        json!({})
    };

    root["mcpServers"] = json!(map);

    crate::config::write_json_file(&path, &root)
}

/// 返回已启用的 MCP 服务器（过滤 enabled==true）
fn collect_enabled_servers(cfg: &McpConfig) -> HashMap<String, Value> {
    let mut out = HashMap::new();
    for (id, entry) in cfg.servers.iter() {
        let enabled = entry
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !enabled {
            continue;
        }
        match extract_server_spec(entry) {
            Ok(spec) => {
                out.insert(id.clone(), spec);
            }
            Err(err) => {
                log::warn!("跳过无效的 MCP 条目 '{id}': {err}");
            }
        }
    }
    out
}

/// 将 config.json 中 Antigravity 的 enabled==true 项写入配置
pub fn sync_enabled_to_antigravity(config: &MultiAppConfig) -> Result<(), AppError> {
    if !should_sync_antigravity_mcp() {
        return Ok(());
    }
    let enabled = collect_enabled_servers(&config.mcp.antigravity);
    set_mcp_servers_map(&enabled)
}

/// 从 Antigravity MCP 配置导入到统一结构
pub fn import_from_antigravity(config: &mut MultiAppConfig) -> Result<usize, AppError> {
    let map = read_mcp_servers_map()?;
    if map.is_empty() {
        return Ok(0);
    }

    // 确保新结构存在
    let servers = config.mcp.servers.get_or_insert_with(HashMap::new);

    let mut changed = 0;
    let mut errors = Vec::new();

    for (id, spec) in map.iter() {
        if let Err(e) = validate_server_spec(spec) {
            log::warn!("跳过无效 MCP 服务器 '{id}': {e}");
            errors.push(format!("{id}: {e}"));
            continue;
        }

        if let Some(existing) = servers.get_mut(id) {
            if !existing.apps.antigravity {
                existing.apps.antigravity = true;
                changed += 1;
            }
        } else {
            servers.insert(
                id.clone(),
                McpServer {
                    id: id.clone(),
                    name: id.clone(),
                    server: spec.clone(),
                    apps: McpApps {
                        claude: false,
                        codex: false,
                        gemini: false,
                        opencode: false,
                        antigravity: true,
                    },
                    description: None,
                    homepage: None,
                    docs: None,
                    tags: Vec::new(),
                },
            );
            changed += 1;
        }
    }

    Ok(changed)
}

pub fn sync_single_server_to_antigravity(
    _config: &MultiAppConfig,
    id: &str,
    server_spec: &Value,
) -> Result<(), AppError> {
    if !should_sync_antigravity_mcp() {
        return Ok(());
    }
    let mut current = read_mcp_servers_map()?;
    current.insert(id.to_string(), server_spec.clone());
    set_mcp_servers_map(&current)
}

pub fn remove_server_from_antigravity(id: &str) -> Result<(), AppError> {
    if !should_sync_antigravity_mcp() {
        return Ok(());
    }
    let mut current = read_mcp_servers_map()?;
    current.remove(id);
    set_mcp_servers_map(&current)
}
