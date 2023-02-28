#[allow(non_snake_case)]

extern crate chrono;
extern crate systemstat;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::process::Command;
use std::num::Wrapping;

use systemstat::{Platform, System};

pub mod utility;
pub mod config;
mod text_builders;
mod network_util;
mod battery_util;
mod cpu_util;
mod keyboard_util;

use crate::network_util::NetworkUtil;
use crate::battery_util::BatteryUtil;
use crate::cpu_util::CPUUtil;
use crate::keyboard_util::KbdUtil;

fn main() {
    let sleep_time = time::Duration::from_millis(config::CYCLE_LENGTH_ms as u64);

    // Initializing resources
    let mut heartbeat = Wrapping(0u8);
    // Networking resources
    let network_util = Arc::new(Mutex::new(NetworkUtil::new()));
    NetworkUtil::spawn_networkstat(Arc::clone(&network_util));
    let mut rx_bytes = 0u32;
    let mut tx_bytes = 0u32;
    // Battery info resources
    let battery_util = Arc::new(Mutex::new(BatteryUtil::new()));
    BatteryUtil::spawn_batterystat(Arc::clone(&battery_util));
    let mut battery_capacity = 0f32;
    let mut battery_ac = false;
    // CPU info resources
    let cpu_util = Arc::new(Mutex::new(CPUUtil::new()));
    CPUUtil::spawn_cpustat(Arc::clone(&cpu_util));
    let mut cpu_temperature = 0f32;
    // Keyboard resources
    let kbd_util = Arc::new(Mutex::new(KbdUtil::new()));
    KbdUtil::spawn_kbdstat(Arc::clone(&kbd_util));
    let mut keyboard_layout: String = "".to_string();
    let mut keyboard_ledmask = (false, false);

    let mut now = chrono::Local::now();
    let mut _status_text = "".to_string();

    loop {
        if let Ok(lock) = Arc::clone(&network_util).try_lock() {
            (rx_bytes, tx_bytes) = lock.get_rxtx();
        }
        if let Ok(lock) = Arc::clone(&battery_util).try_lock() {
            battery_capacity = lock.get_battery_pwr();
            battery_ac = lock.get_battery_ac();
        }
        if let Ok(lock) = Arc::clone(&cpu_util).try_lock() {
            cpu_temperature = lock.get_temperature();
        }
        if let Ok(lock) = Arc::clone(&kbd_util).try_lock() {
            keyboard_layout = lock.get_keyboard_layout();
            keyboard_ledmask = lock.get_ledmask();
        }

        _status_text.clear();
        now = chrono::Local::now();

        _status_text = format!(
            "{} "
            , text_builders::get_heartbeat_text(heartbeat.0)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_keyboard_text(keyboard_layout.clone(), keyboard_ledmask)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_cpu_text(cpu_temperature)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_netw_rxtx_text(&rx_bytes, &tx_bytes)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_battery_text(&battery_capacity, &battery_ac)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_binary_clock_text(&now)
        );

        // Displaying local time.
        _status_text = format!(
            " {} {}"
            , _status_text
            , text_builders::get_clock_text(&now)
        );
        //println!("{}", _status_text);
        utility::setxroot(_status_text.clone());
        thread::sleep(sleep_time);
        heartbeat += 1;
    }
}
