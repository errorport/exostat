use systemstat::{Platform, System};
use chrono::{Timelike, DateTime};
use super::config;
use super::utility;

// Displaying keyboard layout
// Icon: 
pub fn get_keyboard_text() -> String {
    format!(
        " {} {}"
        , utility::get_keyboard_layout()
        , utility::get_keyboard_ledmask()
    )
}

// Displaying CPU temp.
// Icon: 
pub fn get_cpu_text(system: &System) -> String {
    match system.cpu_temp() {
        Ok(_temp) => format!(" {}°C", _temp),
        Err(_e) => "  ??".to_string(),
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
            "{} {:04}KiB/s {} {:04}KiB/s",
            upload_icon,
            (*tx_bytes / 1024) as u32,
            download_icon,
            (*rx_bytes / 1024) as u32
        )
}

// Displaying battery status.
// Icons:         
pub fn get_battery_text(
    pwr: &u8
    , ac: &bool
    ) -> String {
    let mut _battery_icon = "".to_string();
    if *pwr > 20 {
        _battery_icon = "".to_string();
    }
    if *pwr > 40 {
        _battery_icon = "".to_string();
    }
    if *pwr > 60 {
        _battery_icon = "".to_string();
    }
    if *pwr > 80 {
        _battery_icon = "".to_string();
    }
    if *ac {
        _battery_icon
            = format!("^c{}^{}^d^", config::ACTIVE_COLOR, _battery_icon);
    }
    format!("{} {:02}%", _battery_icon, *pwr)
}

// Displaying binary-watch format time.
pub fn get_binary_clock_text(now: &DateTime<chrono::Local>) -> String {
    utility::number_to_binary_str(
        now.time().hour() as u8
        , now.time().minute() as u8
        , now.time().second() as u8
    )
}

// Displaying time.
pub fn get_clock_text(now: &DateTime<chrono::Local>) -> String {
    now.format("^w^%Y-%m-%d %H:%M:%S").to_string()
}
