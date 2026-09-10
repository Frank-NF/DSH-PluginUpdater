//! 本地 DSH Web 服务器管理
//!
//! 用户已从 DSH 桌面版迁移到 WEB 源码版（DeepSeek Harness），
//! 本模块负责对该本地服务器执行：启动 / 停止 / 重启 / 状态查询。
//!
//! 两种运行形态（2026-09-07 实测）：
//!   A. 编译版：cwd=<DSH_WEB_DIR>/apps/cli
//!      cmd = node --expose-internals lib/bin.js --profile web --no-open --port <port>
//!   B. 开发态：cwd=<DSH_WEB_DIR>（仓库根，tsx 直跑 TS 源码）
//!      cmd = node --expose-internals --import tsx/esm apps/cli/src/bin.ts web --no-open --port <port>
//! 进程识别：node.exe 且命令行包含 `web` 子命令特征（bin.js+--profile 或 src/bin.ts web）。
//! 端口不固定（用户常用 3081，DSH 官方默认 3080，旧版误用 8787），状态查询时动态探测。
//!
//! 路径探测顺序（去硬编码）：
//!   1. 环境变量 DSH_WEB_DIR（精确指定）
//!   2. ~/.dsh/web-dir（上次探测缓存）
//!   3. 常见候选路径自动探测（兼容 Windows/macOS/Linux 多环境）
//!
//! 状态检测优先级：
//!   1. 动态探测常见端口 + 扫描 node 进程命令行提取 --port 参数
//!   2. sysinfo 扫描 node 进程命令行特征（端口未知时兜底）

use serde::Serialize;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;
use std::time::Duration;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, RefreshKind, System, UpdateKind};

/// 快速构建「只扫进程命令行」的 System（跳过内存/磁盘/网络/CPU 采集）。
/// System::new_all() 在 Windows 上要采全量信息，实测可达数秒，是面板卡顿的主因。
fn fast_process_system() -> System {
    System::new_with_specifics(
        RefreshKind::new()
            .with_processes(ProcessRefreshKind::new().with_cmd(UpdateKind::Always)),
    )
}

/// DSH Web 源码仓库路径候选（按探测顺序排列，保留供未来 UI 使用）
#[allow(dead_code)]
pub const DSH_WEB_ROOT_CANDIDATES: &[&str] = &[
    // 环境变量精确指定（最高优先级）
    // 注：通过 DSH_WEB_DIR 环境变量设置，运行时读取；此处仅为兜底候选列表
];
/// CLI 入口子目录（始终相对 DSH_WEB_ROOT）
pub const DSH_CLI_SUBDIR: &str = "apps/cli";
/// 默认端口（用户实际使用的端口；DSH 官方默认是 3080，8787 为历史遗留仅作兼容探测）
pub const DSH_WEB_PORT_DEFAULT: u16 = 3081;
/// 兼容探测端口（旧版本曾误用 8787 启动，停掉旧进程时需要检查）
pub const DSH_WEB_PORT_LEGACY: u16 = 8787;

/// 探测 DSH Web 仓库根目录（环境变量 > 上次缓存 > 候选路径 > 返回 None）
pub fn dsh_web_root() -> Option<std::path::PathBuf> {
    // 1. 环境变量精确指定
    if let Ok(p) = std::env::var("DSH_WEB_DIR") {
        if !p.is_empty() && std::path::Path::new(&p).join(DSH_CLI_SUBDIR).exists() {
            return Some(std::path::PathBuf::from(p));
        }
    }
    // 2. 上次探测缓存（用户通过 UI 保存的路径，存储在 ~/.dsh/web-dir）
    if let Some(home) = dirs::home_dir() {
        let cached = home.join(".dsh/web-dir");
        if cached.exists() {
            let root = std::fs::read_to_string(&cached).ok()?.trim().to_string();
            if std::path::Path::new(&root).join(DSH_CLI_SUBDIR).exists() {
                return Some(std::path::PathBuf::from(root));
            }
        }
    }
    // 3. 常见候选路径自动探测（兼容不同机器的安装习惯）
    let home = dirs::home_dir()?;
    for cand in [
        home.join("DeepSeek_Harness"),       // 常用手动 clone 位置
        home.join("Program Files/DeepSeek Harness"),
        home.join("Program Files (x86)/DeepSeek Harness"),
    ] {
        if cand.join(DSH_CLI_SUBDIR).exists() {
            // 缓存下次启动的探测结果
            if let Some(cached_dir) = home.join(".dsh").parent() {
                let _ = std::fs::create_dir_all(cached_dir);
                let _ = std::fs::write(home.join(".dsh/web-dir"), cand.to_string_lossy().as_ref());
            }
            return Some(cand);
        }
    }
    // Windows 专属 G: 盘习惯保留（向后兼容开发机路径，其他机器若存在也会优先被 home 候选覆盖）
    #[cfg(target_os = "windows")]
    {
        for cand in ["G:/DeepSeek_Harness", "D:/DeepSeek_Harness"] {
            if std::path::Path::new(cand).join(DSH_CLI_SUBDIR).exists() {
                if let Some(home) = dirs::home_dir() {
                    let _ = std::fs::write(home.join(".dsh/web-dir"), cand);
                }
                return Some(std::path::PathBuf::from(cand));
            }
        }
    }
    None
}

/// DSH Web profile 数据目录（~/.dsh/profiles/web，去硬编码）
fn dsh_profile_dir() -> std::path::PathBuf {
    dirs::home_dir()
        .map(|h| h.join(".dsh/profiles/web"))
        .unwrap_or_else(|| std::path::PathBuf::from(".dsh/profiles/web"))
}

/// 由本工具启动的 DSH 服务器子进程 PID（0 = 不是本工具启动的）
static SPAWNED_PID: AtomicU32 = AtomicU32::new(0);
/// 最近一次探测到的端口（跨命令记忆，避免每次都全端口扫描）
static LAST_KNOWN_PORT: Mutex<Option<u16>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize)]
pub struct ServerStatus {
    /// 运行状态
    pub running: bool,
    /// 端口是否可连通
    pub port_open: bool,
    /// 实际端口（动态探测）
    pub port: u16,
    /// 匹配到的 DSH Web 进程 PID（0 = 未找到）
    pub pid: u32,
    /// 进程名（找到时）
    pub process_name: String,
    /// 服务器访问地址（运行中时；若捕获到带 token 的地址则为其完整形态）
    pub url: String,
    /// 带 token 的完整访问地址（浏览器打开这个才能真正进入；未捕获时为空串）
    pub auth_url: String,
    /// 状态说明文案
    pub message: String,
}

/// 从启动日志里解析带 token 的访问地址（`dsh web: http://...?token=...`）
fn read_auth_url_from_log() -> String {
    let path = start_log_path();
    let Ok(content) = std::fs::read_to_string(&path) else {
        return String::new();
    };
    // 取最后一条 dsh web: 行（重启后日志覆盖写，通常只有一条）
    for line in content.lines().rev() {
        if let Some(idx) = line.find("dsh web: http") {
            let url = line[idx + "dsh web: ".len()..].trim();
            // 只取到空白前（可能跟 "(LAN: ...)"）
            let url = url.split_whitespace().next().unwrap_or(url);
            if url.contains("token=") {
                return url.to_string();
            }
        }
    }
    String::new()
}

/// 检测端口是否可连通（500ms 超时，无监听端口秒拒、防火墙丢包时不至于卡 2.5s）
fn port_open(port: u16) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    let mut addrs = match ("127.0.0.1", port).to_socket_addrs() {
        Ok(a) => a,
        Err(_) => return false,
    };
    match addrs.next() {
        Some(addr) => TcpStream::connect_timeout(&addr, Duration::from_millis(500)).is_ok(),
        None => false,
    }
}

#[allow(dead_code)]
fn connect_timeout(port: u16) -> bool {
    use std::net::{TcpStream, ToSocketAddrs};
    let addr = ("127.0.0.1", port).to_socket_addrs();
    match addr {
        Ok(mut addrs) => {
            if let Some(a) = addrs.next() {
                TcpStream::connect_timeout(&a, Duration::from_millis(800)).is_ok()
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

/// 扫描系统进程，找到所有 DSH Web 服务器进程。
/// 兼容两种形态：
///   A. 编译版: bin.js + --profile web
///   B. 开发态: apps/cli/src/bin.ts + web 子命令（tsx 直跑）
/// 返回 (pid, 进程名, 命令行中解析出的端口 Option)
fn find_dsh_web_processes() -> Vec<(u32, String, Option<u16>)> {
    let mut found: Vec<(u32, String, Option<u16>)> = Vec::new();
    let mut sys = fast_process_system();
    sys.refresh_processes(ProcessesToUpdate::All, true);
    for p in sys.processes().values() {
        let name = p.name().to_string_lossy().to_lowercase();
        if !name.contains("node") {
            continue;
        }
        // Windows 下 sysinfo 的 cmd() 返回完整命令行（&[OsString]）
        let cmd = p.cmd();
        if cmd.is_empty() {
            continue;
        }
        let cmd_str = cmd
            .iter()
            .map(|s| s.to_string_lossy())
            .collect::<Vec<_>>()
            .join(" ")
            .to_lowercase();

        // 形态 A：编译版 bin.js --profile web
        let is_a = cmd_str.contains("--profile web") && cmd_str.contains("bin.js");
        // 形态 B：开发态 src/bin.ts web（注意排除误匹配：必须含 bin.ts 才认）
        let is_b = cmd_str.contains("bin.ts") && contains_web_subcommand(&cmd_str);
        // 排除 MCP server 等无关 node 进程：bin.ts/web 特征已足够窄
        if !is_a && !is_b {
            continue;
        }

        // 从命令行提取 --port <n>（兼容 --port=<n>）
        let port = extract_port(&cmd_str);
        found.push((p.pid().as_u32(), p.name().to_string_lossy().to_string(), port));
    }
    found
}

/// 判断命令行是否包含 `web` 子命令（独立 token，且不是 --profile web 之外的路径片段）
fn contains_web_subcommand(cmd: &str) -> bool {
    cmd.split_whitespace().any(|tok| tok == "web" || tok == "'web'" || tok == "\"web\"")
}

/// 从命令行提取端口：--port 3081 或 --port=3081
fn extract_port(cmd: &str) -> Option<u16> {
    let tokens: Vec<&str> = cmd.split_whitespace().collect();
    for (i, tok) in tokens.iter().enumerate() {
        if *tok == "--port" || *tok == "--port=" {
            if let Some(p) = tokens.get(i + 1) {
                if let Ok(n) = p.trim_matches(|c| c == '\'' || c == '"').parse::<u16>() {
                    return Some(n);
                }
            }
        } else if let Some(rest) = tok.strip_prefix("--port=") {
            if let Ok(n) = rest.parse::<u16>() {
                return Some(n);
            }
        }
    }
    None
}

/// 探测当前 DSH Web 服务器实际端口：
/// 1. 进程命令行里的 --port 参数（最准）
/// 2. 上次记忆的端口
/// 3. 常见端口列表逐个试连
fn detect_port() -> Option<u16> {
    // 1. 进程命令行
    for (_pid, _name, port) in find_dsh_web_processes() {
        if let Some(p) = port {
            if port_open(p) {
                return Some(p);
            }
        }
    }
    // 2. 上次记忆
    if let Ok(guard) = LAST_KNOWN_PORT.lock() {
        if let Some(p) = *guard {
            if port_open(p) {
                return Some(p);
            }
        }
    }
    // 3. 常见端口逐个试（用户常用 3081 优先，官方默认 3080 次之，8787 为历史兼容）
    for p in [DSH_WEB_PORT_DEFAULT, 3080, DSH_WEB_PORT_LEGACY, 3000, 8080, 8788] {
        if port_open(p) {
            // 有监听不一定是 DSH，进一步验证：该端口监听进程的命令行是否像 DSH
            if let Some(pid) = pid_by_port(p) {
                for (wpid, _n, _pt) in find_dsh_web_processes() {
                    if wpid == pid {
                        return Some(p);
                    }
                }
            }
        }
    }
    None
}

/// 查询服务器状态
/// async：含进程扫描与端口探测，必须离开主线程
#[tauri::command]
pub async fn server_status() -> ServerStatus {
    // 动态探测端口 + 进程
    let procs = find_dsh_web_processes();
    let (pid, name) = match procs.first() {
        Some((p, n, _)) => (*p, n.clone()),
        None => {
            // 本工具启动的进程可能 sysinfo 看不到命令行（权限），用记录的 PID 兜底
            let spid = SPAWNED_PID.load(Ordering::SeqCst);
            if spid > 0 {
                let mut sys = fast_process_system();
                sys.refresh_processes(
                    ProcessesToUpdate::Some(&[sysinfo::Pid::from_u32(spid)]),
                    true,
                );
                if let Some(p) = sys.process(sysinfo::Pid::from_u32(spid)) {
                    (spid, p.name().to_string_lossy().to_string())
                } else {
                    (0, String::new())
                }
            } else {
                (0, String::new())
            }
        }
    };

    // 端口：进程命令行 > 记忆 > 默认探测
    let port = procs
        .iter()
        .find_map(|(_, _, p)| *p)
        .or_else(detect_port)
        .unwrap_or(DSH_WEB_PORT_DEFAULT);

    let open = port_open(port);
    if open {
        if let Ok(mut g) = LAST_KNOWN_PORT.lock() {
            *g = Some(port);
        }
    }

    let running = open || pid > 0;
    let url = format!("http://127.0.0.1:{}", port);
    // 带 token 的完整地址（仅日志里有；不匹配当前端口时视为无效）
    let auth_url = read_auth_url_from_log();
    let auth_url = if auth_url.starts_with(&url) {
        auth_url
    } else {
        String::new()
    };

    let message = if running && open {
        if auth_url.is_empty() {
            format!("服务器运行中，端口 {} 可访问", port)
        } else {
            format!("服务器运行中，端口 {} 可访问（已获取访问令牌）", port)
        }
    } else if running && !open {
        format!("进程存在但端口 {} 尚未就绪（可能正在启动）", port)
    } else {
        format!("服务器未运行（端口 {} 无监听）", port)
    };

    ServerStatus {
        running,
        port_open: open,
        port,
        pid,
        process_name: name,
        url,
        auth_url,
        message,
    }
}

/// 启动输出日志路径（捕获 stdout，用于解析带 token 的访问地址）
fn start_log_path() -> std::path::PathBuf {
    std::env::temp_dir().join("dsh-plugin-butler-server.log")
}

/// 启动 DSH Web 服务器（端口参数可选，缺省 3081）
/// async：含进程扫描与端口探测，避免阻塞主线程
#[tauri::command]
pub async fn server_start(port: Option<u16>) -> Result<String, String> {
    let port = port.unwrap_or_else(|| {
        // 没指定端口时：若已有服务器在跑，沿用其端口；否则用默认
        detect_port().unwrap_or(DSH_WEB_PORT_DEFAULT)
    });

    // 幂等：该端口已在跑就不重复启动
    if port_open(port) {
        if let Ok(mut g) = LAST_KNOWN_PORT.lock() {
            *g = Some(port);
        }
        return Ok(format!("服务器已在运行中（端口 {}）", port));
    }

    if !dsh_web_root().as_ref().map(|p| p.join(DSH_CLI_SUBDIR).exists()).unwrap_or(false) {
        return Err(format!(
            "未找到 DSH Web 源码目录，请在设置中指定路径，或安装 DeepSeek Harness 本体\n\n\
             已尝试的环境变量: {:?}",
            std::env::var("DSH_WEB_DIR").ok()
        ));
    }
    let web_root = dsh_web_root().expect("checked above");
    let profile_dir = dsh_profile_dir();
    if !profile_dir.exists() {
        // profile 目录不存在时 DSH 会自动创建，这里只提示不阻断
        eprintln!("[server] profile 目录不存在，将由 DSH 自动创建: {}", profile_dir.display());
    }

    // node 路径：优先受管 Node 22（DSH 原生模块编译目标），PATH 兜底
    let node = find_node();

    // 启动输出重定向到日志文件（DSH 启动时会打印带 token 的访问地址，
    // 必须捕获，否则浏览器打开裸地址会被 401 拦截）
    let log_path = start_log_path();
    let log_file = std::fs::File::create(&log_path)
        .map_err(|e| format!("无法创建启动日志文件 {}：{}", log_path.display(), e))?;
    let log_err = log_file
        .try_clone()
        .map_err(|e| format!("无法复制日志句柄：{}", e))?;

    // 分离启动（DETACHED）：关闭本工具后服务器继续运行
    let port_str = port.to_string();
    #[cfg(target_os = "windows")]
    let spawn_result = {
        use std::os::windows::process::CommandExt;
        // CREATE_NEW_PROCESS_GROUP | DETACHED_PROCESS
        const DETACHED_FLAGS: u32 = 0x00000008 | 0x00000200;
        std::process::Command::new(&node)
            .args([
                "--expose-internals",
                "lib/bin.js",
                "--profile",
                "web",
                "--no-open",
                "--port",
                &port_str,
            ])
            .current_dir(web_root.join(DSH_CLI_SUBDIR))
            .creation_flags(DETACHED_FLAGS)
            .stdout(std::process::Stdio::from(log_file))
            .stderr(std::process::Stdio::from(log_err))
            .spawn()
    };
    #[cfg(not(target_os = "windows"))]
    let spawn_result = {
        std::process::Command::new(&node)
            .args([
                "--expose-internals",
                "lib/bin.js",
                "--profile",
                "web",
                "--no-open",
                "--port",
                &port_str,
            ])
            .current_dir(web_root.join(DSH_CLI_SUBDIR))
            .stdout(std::process::Stdio::from(log_file))
            .stderr(std::process::Stdio::from(log_err))
            .spawn()
    };

    match spawn_result {
        Ok(child) => {
            SPAWNED_PID.store(child.id(), Ordering::SeqCst);
            if let Ok(mut g) = LAST_KNOWN_PORT.lock() {
                *g = Some(port);
            }
            let pid = child.id();
            // 不 wait，分离进程
            std::mem::forget(child);

            // 启动后探测：最多等 15 秒确认端口就绪。
            // 此前只发命令不确认，进程若因 Node 版本不匹配等原因崩溃，
            // 面板仍显示「启动成功」，用户浏览器打开却 404/打不开。
            let mut ready = false;
            for _ in 0..30 {
                std::thread::sleep(Duration::from_millis(500));
                if port_open(port) {
                    ready = true;
                    break;
                }
                // 进程已退出且端口未开 → 提前失败
                let mut sys = fast_process_system();
                sys.refresh_processes(
                    ProcessesToUpdate::Some(&[sysinfo::Pid::from_u32(pid)]),
                    true,
                );
                if sys.process(sysinfo::Pid::from_u32(pid)).is_none() {
                    break;
                }
            }

            if ready {
                Ok(format!("服务器已启动（PID {}，端口 {}）", pid, port))
            } else {
                // 读取启动日志尾部的真实报错，帮助定位（如 Node 版本不匹配）
                let log_tail = std::fs::read_to_string(start_log_path())
                    .map(|c| {
                        let lines: Vec<&str> = c.lines().collect();
                        let start = lines.len().saturating_sub(8);
                        lines[start..].join("\n")
                    })
                    .unwrap_or_default();
                Err(format!(
                    "启动后端口 {} 未就绪（进程可能已退出）。\n启动日志尾部：\n{}",
                    port, log_tail
                ))
            }
        }
        Err(e) => Err(format!(
            "启动失败：{}\n尝试的 node 路径：{}",
            e, node
        )),
    }
}

/// 停止 DSH Web 服务器（杀所有 DSH Web 形态进程 + 端口兜底）
/// async：含进程扫描、taskkill 与重试等待，必须离开主线程
#[tauri::command]
pub async fn server_stop() -> Result<String, String> {
    let mut stopped: Vec<String> = Vec::new();
    let mut killed_pids: Vec<u32> = Vec::new();

    // 1. 杀所有识别到的 DSH Web 进程（兼容编译版/开发态，可能不止一个）
    for (pid, _name, _port) in find_dsh_web_processes() {
        if kill_process_tree(pid) {
            killed_pids.push(pid);
            stopped.push(format!("进程 {} 已结束", pid));
        }
    }

    // 2. 本工具启动记录的 PID 兜底
    let spid = SPAWNED_PID.swap(0, Ordering::SeqCst);
    if spid > 0 && !killed_pids.contains(&spid)
        && kill_process_tree(spid) {
            killed_pids.push(spid);
            stopped.push(format!("进程 {} 已结束", spid));
        }

    // 3. 终极兜底：已知端口仍开着，按端口找 PID（netstat）
    let known_ports: Vec<u16> = LAST_KNOWN_PORT
        .lock()
        .ok()
        .and_then(|g| *g)
        .into_iter()
        .chain([DSH_WEB_PORT_DEFAULT, 3080, DSH_WEB_PORT_LEGACY])
        .collect();

    for attempts in 0..3 {
        let mut still_open: Vec<u16> = Vec::new();
        for port in &known_ports {
            if port_open(*port) {
                still_open.push(*port);
                if let Some(pid) = pid_by_port(*port) {
                    if !killed_pids.contains(&pid) && kill_process_tree(pid) {
                        killed_pids.push(pid);
                        stopped.push(format!("端口 {} 占用进程 {} 已结束", port, pid));
                    }
                }
            }
        }
        if still_open.is_empty() || attempts == 2 {
            break;
        }
        std::thread::sleep(Duration::from_millis(400));
    }

    let any_left = known_ports.iter().any(|p| port_open(*p));
    if any_left {
        return Err("停止失败：服务器端口仍被占用，请手动检查进程".to_string());
    }

    if let Ok(mut g) = LAST_KNOWN_PORT.lock() {
        *g = None;
    }

    if stopped.is_empty() {
        Ok("服务器本就未在运行".to_string())
    } else {
        Ok(stopped.join("；"))
    }
}

/// 重启 = 停止 + 用原端口启动
#[tauri::command]
pub async fn server_restart(port: Option<u16>) -> Result<String, String> {
    // 先记住重启前端口
    let prior_port = port
        .or_else(|| {
            find_dsh_web_processes()
                .iter()
                .find_map(|(_, _, p)| *p)
        })
        .or_else(detect_port);

    let _ = server_stop().await?;
    // 等待端口释放（TIME_WAIT 等）
    let wait_ports: Vec<u16> = prior_port
        .into_iter()
        .chain([DSH_WEB_PORT_DEFAULT, 3080, DSH_WEB_PORT_LEGACY])
        .collect();
    let mut waited = 0;
    while waited < 20 {
        let any_open = wait_ports.iter().any(|p| port_open(*p));
        if !any_open {
            break;
        }
        std::thread::sleep(Duration::from_millis(300));
        waited += 1;
    }
    server_start(prior_port).await
}

// ---------- 内部工具 ----------

/// 查找可用的 node 可执行文件
///
/// ⚠️ 优先级很关键：DSH 的原生依赖（fs-ext 等）是用 Node 22 编译的
/// （NODE_MODULE_VERSION 127）。如果用系统 PATH 里的 Node 24（MODULE_VERSION 137）
/// 启动，会 ERR_DLOPEN_FAILED 直接崩溃，表现为「启动成功但打不开」。
/// 所以必须优先用受管 Node 22，PATH 兜底。
fn find_node() -> String {
    // 0. 环境变量显式指定（最高优先级）
    if let Ok(p) = std::env::var("DSH_NODE_PATH") {
        if !p.is_empty() && std::path::Path::new(&p).exists() {
            return p;
        }
    }
    // 1. 受管 Node（.workbuddy 目录，按版本号取最新 22.x）
    if let Some(home) = dirs::home_dir() {
        let base = home.join(".workbuddy/binaries/node/versions");
        if base.exists() {
            if let Ok(entries) = std::fs::read_dir(&base) {
                let mut versions: Vec<_> = entries
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.is_dir() && p.file_name().map(|n| n.to_string_lossy().starts_with("22")).unwrap_or(false))
                    .collect();
                versions.sort();
                if let Some(latest) = versions.into_iter().next_back() {
                    let exe = latest.join(if cfg!(windows) { "node.exe" } else { "node" });
                    if exe.exists() {
                        return exe.to_string_lossy().to_string();
                    }
                }
            }
        }
    }
    // 2. PATH 兜底
    if let Ok(path) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path) {
            let candidate = dir.join(if cfg!(windows) { "node.exe" } else { "node" });
            if candidate.exists() {
                return candidate.to_string_lossy().to_string();
            }
        }
    }
    // 3. 其他常见安装位置
    let fallbacks = [
        "C:/Program Files/nodejs/node.exe",
        "C:/Program Files (x86)/nodejs/node.exe",
        "/usr/local/bin/node",
        "/usr/bin/node",
    ];
    for f in fallbacks {
        if std::path::Path::new(f).exists() {
            return f.to_string();
        }
    }
    "node".to_string() // 最后交给系统 PATH 解析
}

/// 杀进程树（Windows: taskkill /F /T；Unix: kill -9）
fn kill_process_tree(pid: u32) -> bool {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("taskkill")
            .args(["/F", "/T", "/PID", &pid.to_string()])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("kill")
            .args(["-9", &pid.to_string()])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

/// 按端口找监听进程 PID（Windows: netstat -ano）
fn pid_by_port(port: u16) -> Option<u32> {
    #[cfg(target_os = "windows")]
    {
        let out = std::process::Command::new("netstat")
            .args(["-ano", "-p", "TCP"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&out.stdout);
        let marker = format!(":{}", port);
        for line in text.lines() {
            let lower = line.to_lowercase();
            if lower.contains(&marker) && lower.contains("listening") {
                let fields: Vec<&str> = line.split_whitespace().collect();
                if let Some(last) = fields.last() {
                    if let Ok(pid) = last.parse::<u32>() {
                        if pid > 0 {
                            return Some(pid);
                        }
                    }
                }
            }
        }
        None
    }
    #[cfg(not(target_os = "windows"))]
    {
        let out = std::process::Command::new("ss")
            .args(["-tlnp"])
            .output()
            .ok()?;
        let text = String::from_utf8_lossy(&out.stdout);
        let marker = format!(":{}", port);
        for line in text.lines() {
            if line.contains(&marker) {
                if let Some(start) = line.find("pid=") {
                    let rest = &line[start + 4..];
                    let pid_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        return Some(pid);
                    }
                }
            }
        }
        None
    }
}
