use std::process::Command;

use systemstat::{Platform, System};
use chrono::{Timelike, DateTime};
use super::config;
use super::utility;

// Displaying keyboard layout
// Icon: 
#[inline]
pub fn get_keyboard_text(cmd_layout: &mut Command, cmd_leds: &mut Command) -> String {
    format!(
        " {} {}"
        , utility::get_keyboard_layout(cmd_layout)
        , utility::get_keyboard_ledmask(cmd_leds)
    )
}

// Displaying CPU temp.
// Icon: 
#[inline]
pub fn get_cpu_text(temp: f32) -> String {
        format!(" {:02}°C", temp)
}

// Displaying network traffic statistics.
// Icons: " "
#[inline]
pub fn get_netw_rxtx_text(rx_bytes: &u32, tx_bytes: &u32) -> String {
        let mut upload_icon = "".to_string();
        let mut download_icon = "".to_string();
        if *rx_bytes > 0 {
            download_icon = format!("^c{}^{}^d^", config::ACTIVE_COLOR, download_icon);
        }
        if *tx_bytes > 0 {
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
#[inline]
pub fn get_battery_text(
    pwr: &u8
    , ac: &bool
    ) -> String {
    let mut _battery_icon = "".to_string();
    if *pwr > 80 {
        _battery_icon = "".to_string();
    } else if *pwr > 60 {
        _battery_icon = "".to_string();
    } else if *pwr > 40 {
        _battery_icon = "".to_string();
    } else if *pwr > 20 {
        _battery_icon = "".to_string();
    }
    if *ac {
        _battery_icon
            = format!("^c{}^{}^d^", config::ACTIVE_COLOR, _battery_icon);
    }
    format!("{} {:02}%", _battery_icon, *pwr)
}

// Displaying binary-watch format time.
#[inline]
pub fn get_binary_clock_text(now: &DateTime<chrono::Local>) -> String {
    utility::number_to_binary_str(
        now.time().hour() as u8
        , now.time().minute() as u8
        , now.time().second() as u8
    )
}

// Displaying time.
#[inline]
pub fn get_clock_text(now: &DateTime<chrono::Local>) -> String {
    now.format("^w^%Y-%m-%d %H:%M:%S").to_string()
}

#[inline]
pub fn get_heartbeat_text(hb: u8) -> String {
    if hb % 2 == 0 { return "+".to_string() }
    "-".to_string()
}

