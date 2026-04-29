use serde::{Deserialize, Serialize};

/// Overall gateway health snapshot used by the status bar widget.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GatewayStatus {
    pub reachable: bool,
    pub tool_count: usize,
    pub mission_board_ok: bool,
    pub services: Vec<ServiceHealth>,
}

impl GatewayStatus {
    pub fn unreachable() -> Self {
        Self {
            reachable: false,
            tool_count: 0,
            mission_board_ok: false,
            services: vec![],
        }
    }

    /// One-line summary suitable for a status bar label.
    pub fn label(&self) -> String {
        if !self.reachable {
            return "⬡ KiloCore: DOWN".to_string();
        }
        let ok_count = self.services.iter().filter(|s| s.ok).count();
        format!(
            "⬡ KC {}/{} svc  {} tools",
            ok_count,
            self.services.len(),
            self.tool_count
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceHealth {
    pub name: String,
    pub ok: bool,
    pub latency_ms: Option<u64>,
}

/// Lightweight tool descriptor returned from tools/list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,
    pub description: Option<String>,
}
