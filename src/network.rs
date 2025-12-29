use std::fs;

pub fn get_default_network_interface() -> Option<String> {
    let paths = fs::read_dir("/sys/class/net").ok()?;

    for entry in paths.flatten() {
        let iface = entry.file_name().into_string().ok()?;

        // 1. Skip loopback
        if iface == "lo" {
            continue;
        }

        let path = entry.path();

        // 2. Check if the interface is 'up'
        let operstate = fs::read_to_string(path.join("operstate")).unwrap_or_default();
        if !operstate.contains("up") {
            continue;
        }

        // 3. Check for carrier (physical connection detected)
        let carrier = fs::read_to_string(path.join("carrier")).unwrap_or_default();
        if carrier.trim() == "1" {
            return Some(iface);
        }
    }
    None
}

pub fn get_received_bytes(network_interface: &str) -> Option<u64> {
    let rx_bytes_path = format!("/sys/class/net/{}/statistics/rx_bytes", network_interface);
    if let Ok(received_bytes_str) = fs::read_to_string(rx_bytes_path) {
        return u64::from_str_radix(received_bytes_str.trim_end(), 10).ok();
    }
    None
}

pub fn get_sent_bytes(network_interface: &str) -> Option<u64> {
    let tx_bytes_path = format!("/sys/class/net/{}/statistics/tx_bytes", network_interface);
    if let Ok(sent_bytes_str) = fs::read_to_string(tx_bytes_path) {
        return u64::from_str_radix(sent_bytes_str.trim_end(), 10).ok();
    }
    None
}
