use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use kilocore_gateway::{GatewayClient, GatewayStatus};
use warpui::{AppContext, ModelContext, SingletonEntity};

const POLL_INTERVAL: Duration = Duration::from_secs(30);

/// Background polling model for the KiloCore gateway.
///
/// Registered as a singleton in the app context. Polls `http://localhost:8000/mcp`
/// every 30 seconds from a background thread and caches the result. The terminal
/// pane header reads from this singleton to render the gateway health indicator.
pub struct GatewayPoller {
    status: Arc<Mutex<GatewayStatus>>,
}

impl SingletonEntity for GatewayPoller {}

impl GatewayPoller {
    pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
        let status = Arc::new(Mutex::new(GatewayStatus::unreachable()));
        let shared = status.clone();
        thread::Builder::new()
            .name("kilocore-gateway-poller".to_string())
            .spawn(move || {
                let client = GatewayClient::from_env();
                loop {
                    let probe = client.probe();
                    if let Ok(mut guard) = shared.lock() {
                        *guard = probe;
                    }
                    thread::sleep(POLL_INTERVAL);
                }
            })
            .ok();
        Self { status }
    }

    pub fn current_status(&self) -> GatewayStatus {
        self.status
            .lock()
            .map(|g| g.clone())
            .unwrap_or_else(|_| GatewayStatus::unreachable())
    }

    pub fn current_label(&self) -> String {
        self.current_status().label()
    }
}

/// Read the cached gateway status from the app context.
pub fn cached_status(app: &AppContext) -> GatewayStatus {
    GatewayPoller::as_ref(app).current_status()
}
