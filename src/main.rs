#[allow(non_snake_case)]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

extern crate chrono;
extern crate systemstat;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::process::Command;
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
    let sys  = Arc::new(Mutex::new(System::new()));
    let mut heartbeat = 0u8;
    // Networking resources
    let network_util = Arc::new(Mutex::new(NetworkUtil::default()));
    NetworkUtil::spawn_networkstat(Arc::clone(&network_util), Arc::clone(&sys));
    let mut rx_bytes: u32;
    let mut tx_bytes: u32;
    // Battery info resources
    let battery_util = Arc::new(Mutex::new(BatteryUtil::default()));
    BatteryUtil::spawn_batterystat(Arc::clone(&battery_util), Arc::clone(&sys));
    let mut battery_capacity: u8;
    let mut battery_ac: bool;
    // CPU info resources
    let cpu_util = Arc::new(Mutex::new(CPUUtil::default()));
    CPUUtil::spawn_cpustat(Arc::clone(&cpu_util), Arc::clone(&sys));
    let mut cpu_temperature: f32;
    // Keyboard resources
    let kbd_util = Arc::new(Mutex::new(KbdUtil::new()));
    KbdUtil::spawn_kbdstat(Arc::clone(&kbd_util));
    let mut keyboard_layout: String;
    let mut keyboard_ledmask: (bool, bool);
    // XSETROOT
    let mut cmd_xsetroot = Command::new("xsetroot");

    let mut now = chrono::Local::now();
    let mut _status_text = "".to_string();

    loop {
        (rx_bytes, tx_bytes) = Arc::clone(&network_util).lock().unwrap().get_rxtx();
        battery_capacity = Arc::clone(&battery_util).lock().unwrap().get_battery_pwr();
        battery_ac = Arc::clone(&battery_util).lock().unwrap().get_battery_ac();
        cpu_temperature = Arc::clone(&cpu_util).lock().unwrap().get_temperature();
        keyboard_layout = Arc::clone(&kbd_util).lock().unwrap().get_keyboard_layout();
        keyboard_ledmask = Arc::clone(&kbd_util).lock().unwrap().get_ledmask();

        _status_text.clear();
        now = chrono::Local::now();

        _status_text = format!(
            "{} "
            , text_builders::get_heartbeat_text(heartbeat)
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
        utility::setxroot(&mut cmd_xsetroot, &_status_text);
        thread::sleep(sleep_time);
        heartbeat += 1;
    }
}
