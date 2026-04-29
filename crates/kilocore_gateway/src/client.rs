use std::{
    io::{Read, Write as IoWrite},
    net::TcpStream,
    time::{Duration, Instant},
};

use anyhow::{Context, Result};
use serde_json::Value;

use crate::status::{GatewayStatus, ServiceHealth, ToolInfo};

const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8000;
const PROBE_TIMEOUT: Duration = Duration::from_millis(1500);

/// Synchronous MCP gateway client — no async runtime required.
///
/// Designed for use in Warp's status bar polling path where a lightweight
/// stdlib-only probe is preferable to spinning up a Tokio runtime.
pub struct GatewayClient {
    host: String,
    port: u16,
}

impl Default for GatewayClient {
    fn default() -> Self {
        Self::new(DEFAULT_HOST, DEFAULT_PORT)
    }
}

impl GatewayClient {
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self { host: host.into(), port }
    }

    pub fn from_env() -> Self {
        let host = std::env::var("KILOCORE_GATEWAY_HOST")
            .unwrap_or_else(|_| DEFAULT_HOST.to_string());
        let port = std::env::var("KILOCORE_GATEWAY_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(DEFAULT_PORT);
        Self::new(host, port)
    }

    /// Full health snapshot: reachability + tool count + mission board.
    pub fn probe(&self) -> GatewayStatus {
        let t0 = Instant::now();

        // TCP reachability fast path
        if TcpStream::connect_timeout(
            &format!("{}:{}", self.host, self.port).parse().unwrap(),
            PROBE_TIMEOUT,
        )
        .is_err()
        {
            log::debug!("kilocore_gateway: TCP connect failed");
            return GatewayStatus::unreachable();
        }

        let tools = self.list_tools().unwrap_or_default();
        let mission_ok = self.probe_mission_board().unwrap_or(false);

        let services = vec![
            ServiceHealth {
                name: "nusyq-hub".to_string(),
                ok: true,
                latency_ms: Some(t0.elapsed().as_millis() as u64),
            },
            ServiceHealth {
                name: "mission-board".to_string(),
                ok: mission_ok,
                latency_ms: None,
            },
        ];

        GatewayStatus {
            reachable: true,
            tool_count: tools.len(),
            mission_board_ok: mission_ok,
            services,
        }
    }

    /// Fetch the MCP tools/list via JSON-RPC over raw HTTP/1.1.
    pub fn list_tools(&self) -> Result<Vec<ToolInfo>> {
        let body = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{}}"#;
        let resp = self.http_post("/mcp", body)?;
        let val: Value = serde_json::from_str(&resp)?;
        let tools = val["result"]["tools"]
            .as_array()
            .context("no tools array")?
            .iter()
            .map(|t| ToolInfo {
                name: t["name"].as_str().unwrap_or("").to_string(),
                description: t["description"].as_str().map(|s| s.to_string()),
            })
            .collect();
        Ok(tools)
    }

    fn probe_mission_board(&self) -> Result<bool> {
        let resp =
            self.http_get("/apps/kilocore-mission-board/health")?;
        Ok(resp.contains("200") || resp.to_lowercase().contains("ok"))
    }

    // ── raw HTTP/1.1 helpers ────────────────────────────────────────────────

    fn http_get(&self, path: &str) -> Result<String> {
        let req = format!(
            "GET {} HTTP/1.1\r\nHost: {}:{}\r\nConnection: close\r\n\r\n",
            path, self.host, self.port
        );
        self.raw_request(&req)
    }

    fn http_post(&self, path: &str, body: &str) -> Result<String> {
        let req = format!(
            "POST {} HTTP/1.1\r\nHost: {}:{}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            path, self.host, self.port, body.len(), body
        );
        self.raw_request(&req)
    }

    fn raw_request(&self, req: &str) -> Result<String> {
        let addr = format!("{}:{}", self.host, self.port);
        let mut stream =
            TcpStream::connect_timeout(&addr.parse()?, PROBE_TIMEOUT)
                .context("TCP connect")?;
        stream.set_read_timeout(Some(PROBE_TIMEOUT))?;
        stream.write_all(req.as_bytes()).context("write")?;
        let mut buf = String::new();
        stream.read_to_string(&mut buf).context("read")?;
        // Return everything after the HTTP header separator
        Ok(buf
            .split_once("\r\n\r\n")
            .map(|(_, body)| body)
            .unwrap_or(&buf)
            .to_string())
    }
}
