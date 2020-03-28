#[allow(non_snake_case)]

extern crate chrono;
extern crate systemstat;

use chrono::Timelike;
use std::{thread, time};
use systemstat::{Platform, System};

pub mod utility;
pub mod config;
mod text_builders;

fn main() { 
    let sleep_time = time::Duration::from_millis(config::CYCLE_LENGTH as u64);
    let mut rx_bytes_previous = 0u32;
    let mut tx_bytes_previous = 0u32;
    let mut cycle_counter = 0u8;
    let mut rx_bytes = 0u32;
    let mut tx_bytes = 0u32;
    let mut rx_bytes_counter = 0u32;
    let mut tx_bytes_counter = 0u32;
    let sys = System::new();

    loop {
        let mut _status_text: String = "".to_string();
        let mut rx_bytes_summa = 0u32;
        let mut tx_bytes_summa = 0u32;
        let mut rx_bytes_diff = 0i64;
        let mut tx_bytes_diff = 0i64;
        let mut upload_icon = "".to_string();
        let mut download_icon = "".to_string();

        utility::calculate_network_rxtx(
            &sys
            , &mut rx_bytes_previous
            , &mut tx_bytes_previous
            , &mut rx_bytes_counter
            , &mut tx_bytes_counter
            , &mut rx_bytes
            , &mut tx_bytes
            , &cycle_counter
        );

        _status_text = format!(
            "{} | "
            , text_builders::get_keyboard_text()
        );

        _status_text = format!(
            "{} {} | "
            , _status_text
            , text_builders::get_cpu_text(&sys)
        );


        // Displaying network statistics.
        let network_interfaces = sys.networks().unwrap();
        for network_if in network_interfaces.values() {
            match sys.network_stats(&network_if.name) {
                Ok(netstat) => {
                    rx_bytes_summa += netstat.rx_bytes as u32;
                    tx_bytes_summa += netstat.tx_bytes as u32;
                }
                Err(e) => println!("{}", e),
            }
        }
        rx_bytes_diff = rx_bytes_summa as i64 - rx_bytes_previous as i64;
        if rx_bytes_diff < 0 {
            rx_bytes_diff = 0;
        }
        tx_bytes_diff = tx_bytes_summa as i64 - tx_bytes_previous as i64;
        if tx_bytes_diff < 0 {
            tx_bytes_diff = 0;
        }
        rx_bytes_counter += rx_bytes_diff as u32;
        tx_bytes_counter += tx_bytes_diff as u32;
        rx_bytes_previous = rx_bytes_summa;
        tx_bytes_previous = tx_bytes_summa;
        if (cycle_counter as u16) % (1000 / (config::CYCLE_LENGTH as u16)) == 0 {
            rx_bytes = rx_bytes_counter;
            tx_bytes = tx_bytes_counter;
            rx_bytes_counter = 0;
            tx_bytes_counter = 0;
        }
        if rx_bytes_diff > 0 {
            download_icon = format!("^c{}^{}^d^", config::ACTIVE_COLOR, download_icon);
        }
        if tx_bytes_diff > 0 {
            upload_icon = format!("^c{}^{}^d^", config::ACTIVE_COLOR, upload_icon);
        }
        _status_text = format!(
            "{} | {} {:04}kB/s - {} {:04}kB/s",
            _status_text,
            upload_icon,
            (tx_bytes / 1024) as u32,
            download_icon,
            (rx_bytes / 1024) as u32
        );

        // Displaying battery status.
        let mut _battery_icon = "".to_string();
        let mut _battery_capacity = 0u8;
        match sys.battery_life() {
            Ok(battery) => {
                let pwr = (battery.remaining_capacity * 100.0) as u8;
                if pwr > 20 {
                    _battery_icon = "".to_string();
                }
                if pwr > 40 {
                    _battery_icon = "".to_string();
                }
                if pwr > 60 {
                    _battery_icon = "".to_string();
                }
                if pwr > 80 {
                    _battery_icon = "".to_string();
                }
                _battery_capacity = (battery.remaining_capacity * 100.0) as u8;
            }
            Err(e) => println!("{}", e),
        }

        // Displaying AC status.
        match sys.on_ac_power() {
            Ok(_is_ac_plugged) => {
                if _is_ac_plugged {
                    _battery_icon
                        = format!("^c{}^{}^d^", config::ACTIVE_COLOR, _battery_icon);
                }
            }
            Err(e) => println!("{}", e),
        }
        _status_text = format!(
            "{} | {} {:02}%",
            _status_text, _battery_icon, _battery_capacity
        );

        // Displaying binary-watch format time.
        let now = chrono::Local::now();
        _status_text = format!(
            " {} | {} {} {}",
            _status_text,
            utility::number_to_binary_str(now.time().second() as u8),
            utility::number_to_binary_str(now.time().minute() as u8),
            utility::number_to_binary_str(now.time().hour() as u8)
        );

        // Displaying local time.
        _status_text = format!(" {} | {}", _status_text, now.format("%Y-%m-%d %H:%M:%S"));

        //println!("{}", _status_text);
        utility::setxroot(_status_text);
        thread::sleep(sleep_time);
        cycle_counter += 1;
        // Avoid to use unsafe block because of overflowing.
        if cycle_counter >= 254 {
            cycle_counter = 0;
        }
    }
}
