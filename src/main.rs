#[allow(non_snake_case)]

extern crate chrono;
extern crate systemstat;

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
    let mut rx_bytes_diff = 0i64;
    let mut tx_bytes_diff = 0i64;
    let mut battery_capacity = 0u8;
    let mut battery_ac = false;

    // Initializing system resources
    let sys  = System::new();
    let netw = sys.networks().unwrap();
    let mut now = chrono::Local::now();

    loop {
        let mut _status_text: String = "".to_string();
        now = chrono::Local::now();

        utility::calculate_network_rxtx(
            &sys
            , &netw
            , &mut rx_bytes_previous
            , &mut tx_bytes_previous
            , &mut rx_bytes_counter
            , &mut tx_bytes_counter
            , &mut rx_bytes
            , &mut tx_bytes
            , &mut rx_bytes_diff
            , &mut tx_bytes_diff
            , &cycle_counter
        );

        if (cycle_counter % config::BATTERY_READ_CYCLE) == 0 {
            battery_capacity = utility::get_battery_pwr(&sys);
            battery_ac = utility::get_battery_ac(&sys);
        }

        _status_text = format!(
            "{} |"
            , text_builders::get_keyboard_text()
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_cpu_text(&sys)
        );

        _status_text = format!(
            "{} {} |"
            , _status_text
            , text_builders::get_netw_rxtx_text(
                &rx_bytes_diff
                , &tx_bytes_diff
                , &rx_bytes
                , &tx_bytes
            )
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
        utility::setxroot(_status_text);
        thread::sleep(sleep_time);
        cycle_counter += 1;
        // Avoid to use unsafe block because of overflowing.
        if cycle_counter >= 254 {
            cycle_counter = 0;
        }
    }
}
