/// Quick CLI probe — exercises GatewayClient::probe() and prints a status bar label.
///
/// Usage:
///   cargo run --package kilocore_gateway --example status_probe
///   KILOCORE_GATEWAY_HOST=192.168.1.5 cargo run --package kilocore_gateway --example status_probe
fn main() {
    env_logger::init();

    let client = kilocore_gateway::GatewayClient::from_env();
    let status = client.probe();

    println!("{}", status.label());
    println!();
    println!("reachable:     {}", status.reachable);
    println!("tool_count:    {}", status.tool_count);
    println!("mission_board: {}", status.mission_board_ok);
    println!();
    for svc in &status.services {
        let latency = svc.latency_ms
            .map(|ms| format!(" ({}ms)", ms))
            .unwrap_or_default();
        println!(
            "  {:20} {}{}",
            svc.name,
            if svc.ok { "UP" } else { "DOWN" },
            latency
        );
    }

    std::process::exit(if status.reachable { 0 } else { 1 });
}
