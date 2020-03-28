use systemstat::{Platform, System};
use super::config;
use super::utility;

// Displaying keyboard layout
// Icon: 
pub fn get_keyboard_text() -> String {
    format!(" {}"
    , utility::get_keyboard_layout())
}

// Displaying CPU temp.
// Icon:  
pub fn get_cpu_text(system: &System) -> String {
    match system.cpu_temp() {
        Ok(_temp) => format!(" {}°C", _temp),
        Err(e) => "  ??".to_string(),
    }
}

// Displaying network traffic statistics.
// Icons: " "
pub fn get_netw_rxtx_text(
    rx_bytes_diff:   &i64
    , tx_bytes_diff: &i64
    , rx_bytes:      &u32
    , tx_bytes:      &u32
    ) -> String {
        let mut upload_icon = "".to_string();
        let mut download_icon = "".to_string();
        if *rx_bytes_diff > 0 {
            download_icon = format!("^c{}^{}^d^", config::ACTIVE_COLOR, download_icon);
        }
        if *tx_bytes_diff > 0 {
            upload_icon = format!("^c{}^{}^d^", config::ACTIVE_COLOR, upload_icon);
        }
        format!(
            "{} {:04}kB/s - {} {:04}kB/s",
            upload_icon,
            (*tx_bytes / 1024) as u32,
            download_icon,
            (*rx_bytes / 1024) as u32
        )
}
