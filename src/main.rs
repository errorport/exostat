#[allow(non_snake_case)]

extern crate chrono;
extern crate systemstat;

use std::{thread, time};
use std::sync::{Arc, Mutex};
use std::process::{Command, Stdio};
use systemstat::{Platform, System};

pub mod utility;
pub mod config;
mod text_builders;
mod network_util;
mod battery_util;
mod cpu_util;

use crate::network_util::NetworkUtil;
use crate::battery_util::BatteryUtil;
use crate::cpu_util::CPUUtil;

fn main() {
    let sleep_time = time::Duration::from_millis(config::CYCLE_LENGTH_ms as u64);

    // Initializing resources
    let sys  = Arc::new(Mutex::new(System::new()));
    let mut heartbeat = 0u8;
    // Networking resources
    let network_util = Arc::new(Mutex::new(NetworkUtil::default()));
    NetworkUtil::spawn_networkstat(Arc::clone(&network_util), Arc::clone(&sys));
    let mut rx_bytes = 0u32;
    let mut tx_bytes = 0u32;
    // Battery info resources
    let battery_util = Arc::new(Mutex::new(BatteryUtil::default()));
    BatteryUtil::spawn_batterystat(Arc::clone(&battery_util), Arc::clone(&sys));
    let mut battery_capacity = 0u8;
    let mut battery_ac = false;
    // CPU info resources
    let cpu_util = Arc::new(Mutex::new(CPUUtil::default()));
    CPUUtil::spawn_cpustat(Arc::clone(&cpu_util), Arc::clone(&sys));
    let mut cpu_temperature = 0f32;
    // Subprocesses
    let mut cmd_xsetroot = Command::new("xsetroot");
    let mut cmd_setxkbmap = Command::new("setxkbmap");
    let mut cmd_xset = Command::new("xset");

    let mut now = chrono::Local::now();

    loop {
        (rx_bytes, tx_bytes) = Arc::clone(&network_util).lock().unwrap().get_rxtx();
        battery_capacity = Arc::clone(&battery_util).lock().unwrap().get_battery_pwr();
        battery_ac = Arc::clone(&battery_util).lock().unwrap().get_battery_ac();
        cpu_temperature = Arc::clone(&cpu_util).lock().unwrap().get_temperature();

        let mut _status_text: String = "".to_string();
        now = chrono::Local::now();

        _status_text = format!(
            "{} "
            , text_builders::get_heartbeat_text(heartbeat)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_keyboard_text(&mut cmd_setxkbmap, &mut cmd_xset)
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
        utility::setxroot(&mut cmd_xsetroot, _status_text);
        thread::sleep(sleep_time);
        heartbeat += 1;
    }
}
