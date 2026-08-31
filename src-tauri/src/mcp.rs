//! MCP 服务配置面板（V2 §8 P1 第 4 条）。
//! 职责：dsh-mcp.json 读写合并（保留用户手工条目）、env 密值本地加密存储
//! （Windows Credential Manager，经 keyring crate）、stdio/http 连通性预检、
//! 单服务启用/禁用（禁用条目暂存工具私有文件，可随时恢复）。
//!
//! 纪律：dsh-mcp.json 由 DSH 运行时消费，env 值仅在用户显式「写入配置」时落盘；
//! 面板展示一律掩码；merge 语义与 bundle.rs::merge_mcp_servers 一致（既有条目零丢失）。

use crate::error::{AppError, AppResult};
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

/// dsh-mcp.json 位置（测试可用 DSH_MCP_PATH 覆盖，与 bundle.rs 同规则）
pub fn config_path() -> AppResult<PathBuf> {
    match std::env::var("DSH_MCP_PATH") {
        Ok(p) if !p.trim().is_empty() => Ok(PathBuf::from(p)),
        _ => {
            let home = dirs::home_dir().ok_or_else(|| AppError::Other("无法定位用户主目录".into()))?;
            Ok(home.join(".dsh").join("dsh-mcp.json"))
        }
    }
}

/// 禁用条目暂存文件（工具私有目录，非敏感：被禁用的就是明文配置）
fn disabled_path() -> AppResult<PathBuf> {
    let base = dirs::data_dir().ok_or_else(|| AppError::Other("无法定位应用数据目录".into()))?;
    Ok(base.join("dsh-plugin-updater").join("mcp-disabled.json"))
}

/// 读取 dsh-mcp.json（不存在视为空配置）
fn read_config() -> AppResult<serde_json::Map<String, serde_json::Value>> {
    let path = config_path()?;
    if !path.exists() {
        return Ok(serde_json::Map::new());
    }
    let content = std::fs::read_to_string(&path)?;
    let v: serde_json::Value = serde_json::from_str(&content)
        .map_err(|e| AppError::Other(format!("dsh-mcp.json 解析失败: {}", e)))?;
    let servers = v.get("mcpServers").cloned().unwrap_or_else(|| serde_json::json!({}));
    Ok(match servers {
        serde_json::Value::Object(o) => o,
        _ => serde_json::Map::new(),
    })
}

/// 原子写回 mcpServers（保留 root 其它键与用户手工条目）
fn write_config(servers: serde_json::Map<String, serde_json::Value>) -> AppResult<()> {
    let path = config_path()?;
    let mut root: serde_json::Value = if path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&path)?)
            .map_err(|e| AppError::Other(format!("dsh-mcp.json 解析失败: {}", e)))?
    } else {
        serde_json::json!({})
    };
    if !root.is_object() {
        root = serde_json::json!({});
    }
    root["mcpServers"] = serde_json::Value::Object(servers);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(&root)?)?;
    std::fs::rename(&tmp, &path)?;
    Ok(())
}

/// keyring v3：Entry 即一条凭据（service 固定，account=完整键）。
fn vault_key(server_id: &str, key: &str) -> String {
    format!("mcp/{}/{}", server_id, key)
}

fn vault_entry(account: &str) -> AppResult<keyring::Entry> {
    keyring::Entry::new("dsh-plugin-updater", account)
        .map_err(|e| AppError::Other(format!("凭据库不可用: {}", e)))
}

// ---------- 面板数据结构 ----------

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpEnvKeyInfo {
    pub key: String,
    /// true = vault 中已有该键的密值（面板显示掩码）
    pub has_secret: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpEntryInfo {
    pub server_id: String,
    pub name: String,
    pub transport: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    pub env_keys: Vec<McpEnvKeyInfo>,
    pub description: String,
    /// 配置文件 env 中是否已有非空值（可能来自其他工具写入）
    pub has_plain_value: bool,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpListResult {
    pub enabled: Vec<McpEntryInfo>,
    pub disabled: Vec<McpEntryInfo>,
    pub config_exists: bool,
}

// ---------- 命令实现 ----------

/// 列出全部 MCP 服务（启用中 + 已禁用），env 值一律掩码。
pub fn list() -> AppResult<McpListResult> {
    let servers = read_config()?;
    let mut enabled = Vec::new();
    for (sid, v) in &servers {
        let transport = v.get("transport").and_then(|t| t.as_str()).unwrap_or("stdio").to_string();
        let mut env_keys = Vec::new();
        let mut has_plain = false;
        if let Some(env) = v.get("env").and_then(|e| e.as_object()) {
            for (k, val) in env {
                let plain = val.as_str().unwrap_or("");
                if !plain.trim().is_empty() {
                    has_plain = true;
                }
                let has_secret = vault_entry(&vault_key(sid, k))
                    .and_then(|e| {
                        e.get_password()
                            .map_err(|e2| AppError::Other(format!("凭据库读取失败: {}", e2)))
                    })
                    .map(|s| !s.is_empty())
                    .unwrap_or(false);
                env_keys.push(McpEnvKeyInfo { key: k.clone(), has_secret });
            }
        }
        enabled.push(McpEntryInfo {
            server_id: sid.clone(),
            name: v.get("name").and_then(|n| n.as_str()).unwrap_or(sid).to_string(),
            transport: if transport == "streamable-http" { "streamable-http".into() } else { "stdio".into() },
            command: v.get("command").and_then(|c| c.as_str()).map(|s| s.to_string()),
            args: v.get("args").and_then(|a| a.as_array()).map(|a| {
                a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect()
            }),
            url: v.get("url").and_then(|u| u.as_str()).map(|s| s.to_string()),
            env_keys,
            description: v.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
            has_plain_value: has_plain,
        });
    }
    enabled.sort_by(|a, b| a.server_id.cmp(&b.server_id));

    let mut disabled = Vec::new();
    let dpath = disabled_path()?;
    if dpath.exists() {
        if let Ok(content) = std::fs::read_to_string(&dpath) {
            if let Ok(serde_json::Value::Object(o)) = serde_json::from_str::<serde_json::Value>(&content) {
                for (sid, v) in o {
                    let transport = v.get("transport").and_then(|t| t.as_str()).unwrap_or("stdio").to_string();
                    let env_keys: Vec<McpEnvKeyInfo> = v
                        .get("env")
                        .and_then(|e| e.as_object())
                        .map(|m| m.keys().map(|k| McpEnvKeyInfo { key: k.clone(), has_secret: false }).collect())
                        .unwrap_or_default();
                    disabled.push(McpEntryInfo {
                        server_id: sid.clone(),
                        name: v.get("name").and_then(|n| n.as_str()).unwrap_or(&sid).to_string(),
                        transport: if transport == "streamable-http" { "streamable-http".into() } else { "stdio".into() },
                        command: v.get("command").and_then(|c| c.as_str()).map(|s| s.to_string()),
                        args: v.get("args").and_then(|a| a.as_array()).map(|a| {
                            a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect()
                        }),
                        url: v.get("url").and_then(|u| u.as_str()).map(|s| s.to_string()),
                        env_keys,
                        description: v.get("description").and_then(|d| d.as_str()).unwrap_or("").to_string(),
                        has_plain_value: false,
                    });
                }
            }
        }
    }
    disabled.sort_by(|a, b| a.server_id.cmp(&b.server_id));

    Ok(McpListResult {
        enabled,
        disabled,
        config_exists: config_path()?.exists(),
    })
}

/// 保存 env 密值到系统凭据库（加密存储，不落 dsh-mcp.json）。
pub fn save_env(server_id: &str, key: &str, value: &str) -> AppResult<()> {
    let sid = server_id.trim();
    let k = key.trim();
    if sid.is_empty() || k.is_empty() {
        return Err(AppError::Other("server_id / key 不能为空".into()));
    }
    vault_entry(&vault_key(sid, k))
        .and_then(|e| {
            e.set_password(value)
                .map_err(|e2| AppError::Other(format!("凭据库写入失败: {}", e2)))
        })
}

/// 把 vault 中的密值写入 dsh-mcp.json 对应 env（运行时消费需要明文，用户显式触发）。
/// 返回实际写入的键数量。仅处理配置文件中当前存在的条目。
pub fn apply_config() -> AppResult<usize> {
    let mut servers = read_config()?;
    let mut applied = 0usize;
    for (sid, v) in servers.iter_mut() {
        let env = match v.get("env").and_then(|e| e.as_object()) {
            Some(e) => e.clone(),
            None => continue,
        };
        let mut new_env = env.clone();
        for k in env.keys() {
            if let Ok(secret) = vault_entry(&vault_key(sid, k)).and_then(|e| {
                e.get_password()
                    .map_err(|e2| AppError::Other(format!("凭据库读取失败: {}", e2)))
            }) {
                if !secret.is_empty() {
                    new_env.insert(k.clone(), serde_json::Value::String(secret));
                    applied += 1;
                }
            }
        }
        v["env"] = serde_json::Value::Object(new_env);
    }
    if applied > 0 {
        write_config(servers)?;
    }
    Ok(applied)
}

/// 探活结果
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct McpProbeResult {
    pub ok: bool,
    pub detail: String,
    pub latency_ms: u64,
}

/// 连通性预检：stdio=spawn 探活（能启动即可执行），streamable-http=POST 探测。
pub async fn probe(server_id: &str, http: reqwest::Client) -> AppResult<McpProbeResult> {
    let sid = server_id.trim();
    let servers = read_config()?;
    let v = servers
        .get(sid)
        .ok_or_else(|| AppError::Other(format!("服务不存在: {}", sid)))?;
    let transport = v.get("transport").and_then(|t| t.as_str()).unwrap_or("stdio").to_string();
    let started = std::time::Instant::now();

    if transport == "streamable-http" {
        let url = v
            .get("url")
            .and_then(|u| u.as_str())
            .ok_or_else(|| AppError::Other("streamable-http 条目缺少 url".into()))?
            .to_string();
        let resp = http
            .post(&url)
            .timeout(std::time::Duration::from_secs(4))
            .header("content-type", "application/json")
            .body(
                serde_json::json!({
                    "jsonrpc": "2.0",
                    "id": 1,
                    "method": "initialize",
                    "params": { "protocolVersion": "2025-03-26", "capabilities": {}, "clientInfo": { "name": "dsh-plugin-updater", "version": "1.0" } }
                })
                .to_string(),
            )
            .send()
            .await;
        let latency = started.elapsed().as_millis() as u64;
        return match resp {
            Ok(r) => {
                let status = r.status().as_u16();
                // 2xx/4xx 都说明有服务在响应（401/403=鉴权未配，服务本身可达）
                let ok = status < 500;
                Ok(McpProbeResult {
                    ok,
                    detail: if ok { format!("HTTP {}，服务可达", status) } else { format!("HTTP {}，服务异常", status) },
                    latency_ms: latency,
                })
            }
            Err(e) => Ok(McpProbeResult {
                ok: false,
                detail: format!("连接失败: {}", e),
                latency_ms: latency,
            }),
        };
    }

    // stdio：命令存在且能启动即视为可用（真实握手由 DSH 运行时完成）
    let command = v
        .get("command")
        .and_then(|c| c.as_str())
        .ok_or_else(|| AppError::Other("stdio 条目缺少 command".into()))?
        .to_string();
    let args: Vec<String> = v
        .get("args")
        .and_then(|a| a.as_array())
        .map(|a| a.iter().filter_map(|x| x.as_str().map(|s| s.to_string())).collect())
        .unwrap_or_default();

    let mut cmd = tokio::process::Command::new(&command);
    cmd.args(&args)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());
    #[cfg(windows)]
    cmd.creation_flags(0x08000000); // CREATE_NO_WINDOW
    let child = cmd.spawn();
    let latency = started.elapsed().as_millis() as u64;
    match child {
        Ok(mut c) => {
            // 给 300ms 判断是否秒退（命令存在但立即崩溃）
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            match c.try_wait() {
                Ok(Some(status)) => {
                    if status.success() {
                        Ok(McpProbeResult { ok: true, detail: "命令可执行（已正常退出）".into(), latency_ms: latency })
                    } else {
                        Ok(McpProbeResult {
                            ok: false,
                            detail: format!("进程启动后立即退出（code={}），检查命令与参数", status.code().unwrap_or(-1)),
                            latency_ms: latency,
                        })
                    }
                }
                Ok(None) => {
                    let _ = c.start_kill();
                    Ok(McpProbeResult { ok: true, detail: "进程正常拉起（探活后已终止）".into(), latency_ms: latency })
                }
                Err(e) => Ok(McpProbeResult { ok: false, detail: format!("进程状态读取失败: {}", e), latency_ms: latency }),
            }
        }
        Err(e) => Ok(McpProbeResult {
            ok: false,
            detail: format!("无法启动命令 {}: {}", command, e),
            latency_ms: latency,
        }),
    }
}

/// 单服务启用/禁用：禁用=条目移入工具私有暂存文件；启用=写回 dsh-mcp.json。
/// 两条路径都保留其余条目原样（含用户手工条目）。
pub fn toggle(server_id: &str, enable: bool) -> AppResult<()> {
    let sid = server_id.trim();
    if sid.is_empty() {
        return Err(AppError::Other("server_id 不能为空".into()));
    }
    let dpath = disabled_path()?;
    let mut disabled: serde_json::Map<String, serde_json::Value> = if dpath.exists() {
        serde_json::from_str(&std::fs::read_to_string(&dpath)?)
            .map_err(|e| AppError::Other(format!("mcp-disabled.json 解析失败: {}", e)))?
    } else {
        serde_json::Map::new()
    };

    if enable {
        let entry = match disabled.remove(sid) {
            Some(e) => e,
            None => return Err(AppError::Other(format!("暂存区中没有 {}（可能已启用）", sid))),
        };
        let mut servers = read_config()?;
        if !servers.contains_key(sid) {
            servers.insert(sid.to_string(), entry);
        }
        write_config(servers)?;
    } else {
        let mut servers = read_config()?;
        let entry = servers
            .remove(sid)
            .ok_or_else(|| AppError::Other(format!("服务不存在: {}", sid)))?;
        disabled.insert(sid.to_string(), entry);
        write_config(servers)?;
    }

    if let Some(parent) = dpath.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let tmp = dpath.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(&serde_json::Value::Object(disabled))?)?;
    std::fs::rename(&tmp, &dpath)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    #[serial]
    fn mcp_list_roundtrip_and_toggle() {
        let base = std::env::temp_dir().join(format!("dsh-mcp-test-{}", std::process::id()));
        std::fs::create_dir_all(&base).unwrap();
        let mcp_path = base.join("dsh-mcp.json");
        std::env::set_var("DSH_MCP_PATH", &mcp_path);

        // 空配置起步
        let r = list().unwrap();
        assert_eq!(r.enabled.len(), 0);
        assert!(!r.config_exists);

        // 模拟 merge 写入一条（复用 bundle 的 merge 走同一文件）
        let servers = vec![crate::bundle::BundleMcpServerDef {
            server_id: "mcp-test-a".into(),
            name: "Test A".into(),
            transport: "stdio".into(),
            command: "cmd".into(),
            args: vec!["/c".into(), "echo".into(), "hi".into()],
            env_keys: vec!["TOKEN".into()],
            optional: false,
            description: "test".into(),
        }];
        crate::bundle::merge_mcp_servers(&servers).unwrap();

        let r = list().unwrap();
        assert_eq!(r.enabled.len(), 1);
        assert_eq!(r.enabled[0].server_id, "mcp-test-a");
        assert_eq!(r.enabled[0].env_keys.len(), 1);

        // 禁用 → 出现在 disabled；启用 → 回到 enabled
        toggle("mcp-test-a", false).unwrap();
        let r = list().unwrap();
        assert_eq!(r.enabled.len(), 0);
        assert_eq!(r.disabled.len(), 1);
        toggle("mcp-test-a", true).unwrap();
        let r = list().unwrap();
        assert_eq!(r.enabled.len(), 1);
        assert_eq!(r.disabled.len(), 0);

        std::env::remove_var("DSH_MCP_PATH");
        let _ = std::fs::remove_dir_all(&base);
    }
}


