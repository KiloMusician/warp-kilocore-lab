//! Tools for loading a [`ChannelConfig`] from the external config generator binary.
//!
//! For non-bundled builds, the generator is invoked at runtime. For bundled builds, the config
//! is embedded at compile time via the build script.
use std::env;

use warp_core::channel::ChannelConfig;
use warp_core::{
    channel::{AutoupdateConfig, CrashReportingConfig, McpStaticConfig, OzConfig, TelemetryConfig, WarpServerConfig},
    AppId,
};

/// The name of the config generator binary, expected to be on PATH.
const CONFIG_BIN_NAME: &str = "warp-channel-config";

#[macro_export]
#[cfg(windows)]
macro_rules! path_concat {
    ($path:expr, $file:expr) => {
        concat!($path, "\\", $file)
    };
}
#[macro_export]
#[cfg(not(windows))]
macro_rules! path_concat {
    ($path:expr, $file:expr) => {
        concat!($path, "/", $file)
    };
}

#[macro_export]
macro_rules! load_config {
    ($channel:expr) => {{
        #[cfg(feature = "release_bundle")]
        {
            channel_config::load_config_from_embedded(include_str!($crate::path_concat!(
                env!("OUT_DIR"),
                concat!($channel, "_config.json")
            )))
        }

        #[cfg(not(feature = "release_bundle"))]
        {
            channel_config::load_config_from_generator($channel)
        }
    }};
}
pub use load_config;

/// Invokes the config generator binary at runtime and deserializes its JSON output into a
/// [`ChannelConfig`].
#[cfg_attr(feature = "release_bundle", expect(dead_code))]
pub fn load_config_from_generator(channel: &str) -> ChannelConfig {
    if use_kilocore_local_fallback() {
        eprintln!(
            "KILOCORE_WARP_DEV is set; using local fallback ChannelConfig for '{channel}' instead of '{CONFIG_BIN_NAME}'."
        );
        return load_kilocore_local_fallback(channel);
    }

    let target_family = if cfg!(target_family = "wasm") {
        "wasm"
    } else {
        "native"
    };

    let target_os = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        "linux"
    };

    let output = command::blocking::Command::new(CONFIG_BIN_NAME)
        .arg("--channel")
        .arg(channel)
        .arg("--target-family")
        .arg(target_family)
        .arg("--target-os")
        .arg(target_os)
        .output()
        .unwrap_or_else(|err| {
            if err.kind() == std::io::ErrorKind::NotFound {
                panic!(
                    "\n\n'{CONFIG_BIN_NAME}' was not found on PATH.\n\n\
                     To build internal channels, run:\n\
                     \n\
                     \x20 ./script/install_channel_config\n\n"
                )
            }
            panic!("Failed to execute '{CONFIG_BIN_NAME}': {err}")
        });

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        panic!("Config generator failed for channel '{channel}':\n{stderr}");
    }

    serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!("Failed to parse config generator output for channel '{channel}': {err}\nOutput:\n{stdout}")
    })
}

/// Deserializes a [`ChannelConfig`] from a JSON string embedded at compile time.
///
/// This is used to load the channel configuration in release bundles, where configuration
/// is embedded at compile time instead of being generated at runtime.
#[cfg_attr(not(feature = "release_bundle"), expect(dead_code))]
pub fn load_config_from_embedded(json: &str) -> ChannelConfig {
    serde_json::from_str(json)
        .unwrap_or_else(|err| panic!("Failed to parse embedded channel config: {err}"))
}

fn use_kilocore_local_fallback() -> bool {
    env::var_os("KILOCORE_WARP_DEV").is_some()
}

fn load_kilocore_local_fallback(channel: &str) -> ChannelConfig {
    let (app_name, logfile_name) = match channel {
        "stable" => ("Warp", "warp.log"),
        "preview" => ("WarpPreview", "warp_preview.log"),
        "dev" => ("WarpDev", "warp_dev.log"),
        "local" => ("WarpLocal", "warp_local.log"),
        other => panic!("Unsupported channel for KiloCore local fallback: {other}"),
    };

    ChannelConfig {
        app_id: AppId::new("dev", "warp", app_name),
        logfile_name: logfile_name.into(),
        server_config: WarpServerConfig::production(),
        oz_config: OzConfig::production(),
        telemetry_config: None::<TelemetryConfig>,
        autoupdate_config: None::<AutoupdateConfig>,
        crash_reporting_config: None::<CrashReportingConfig>,
        mcp_static_config: None::<McpStaticConfig>,
    }
}
