use std::process::{Command, Stdio};
use super::config;
use systemstat::{Platform, System, Network, BTreeMap};

// Updating X rootserver's window name.
pub fn setxroot(_status_text: String) {
    let _output = Command::new("xsetroot")
        .arg("-name")
        .arg(_status_text)
        .output()
        .expect("Failed to set X root window name!");
    // println!("setxroot status: {:?}", output.status);
}

// Running keyboard layout getter script.
pub fn get_keyboard_layout() -> String {
    let _output = Command::new("setxkbmap")
        .arg("-query")
        .output()
        .expect("??");
    String::from_utf8(
        _output.stdout[_output.stdout.len()-3..].to_vec()
    ).unwrap().replace("\n", "").to_uppercase()
}

// Getting numlock and capslock indicators.
// xset q | grep 'LED mask' | awk '{print $10}'
pub fn get_keyboard_ledmask() -> String {

    let mut ledmask: u8 = 0;

    let mut _xset_output_child = Command::new("xset")
        .arg("q")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    if let Some(_xset_output) = _xset_output_child.stdout.take() {
        let mut _grep_output_child = Command::new("grep")
            .arg("LED")
            .stdin(_xset_output)
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        _xset_output_child.wait().unwrap();

        if let Some(_grep_output) = _grep_output_child.stdout.take() {
            let mut _awk_output_child = Command::new("awk")
                .arg("{print $10}")
                .stdin(_grep_output)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            let mut _out = String::from_utf8(
                _awk_output_child.wait_with_output().unwrap().stdout
            ).unwrap();

            _grep_output_child.wait().unwrap();

            _out.pop();
            ledmask = _out.parse::<u64>().unwrap() as u8;

        }
    }

    format!(
        "{}{}^f{}^"
        , place_dot(
            ledmask & 0x01 // CAPS lock
            , 3
            , config::KEYLOCK_DOT_HORIZONTAL_SPACING
            , config::KEYLOCK_DOT_SIZE
        )
        , place_dot(
            ledmask >> 1 & 0x01 // NUM lock
            , 12
            , config::KEYLOCK_DOT_HORIZONTAL_SPACING
            , config::KEYLOCK_DOT_SIZE
        )
        , config::KEYLOCK_DOT_OFFSET
    )
}

fn place_dot(
    num: u8
    , vertical_offset: u8
    , horizontal_spacing: u8
    , dotsize: u8
) -> String {
    let mut dot_str = format!(
        "^r0,{},{},{}^"
        , vertical_offset
        , dotsize
        , dotsize
    );
    let small_dot_str = format!(
        "^r{},{},{},{}^"
        , horizontal_spacing
        , vertical_offset + horizontal_spacing
        , dotsize / 2
        , dotsize / 2
    );
    dot_str = match num {
        1 => format!(
            "^c{}^{}^d^"
            , config::ACTIVE_COLOR
            , dot_str
        ),
        _ => small_dot_str,
    };

    dot_str
}

// Clock stuff.
// Works with only exodwm or text-color patched dwm.
pub fn number_to_binary_str(hour: u8, min: u8, sec: u8) -> String {
    let mut binary_str: String = "".to_string();
    for bit in 0..8 {
        binary_str = format!(
            "{}{}{}{}"
            , binary_str
            , place_dot(
                hour >> bit & 0x01
                , 3
                , config::BINARY_DOT_HORIZONTAL_SPACING
                , config::BINARY_DOT_SIZE
            )
            , place_dot(
                min  >> bit & 0x01
                , 9
                , config::BINARY_DOT_HORIZONTAL_SPACING
                , config::BINARY_DOT_SIZE
            )
            , place_dot(
                sec  >> bit & 0x01
                , 15
                , config::BINARY_DOT_HORIZONTAL_SPACING
                , config::BINARY_DOT_SIZE
            )
        );
        binary_str = format!(
            "{}^f{}^"
            , binary_str
            , config::BINARY_DOT_OFFSET
        );
    }

    binary_str
}

// Calculating network statistics.
pub fn calculate_network_rxtx<'a>(
    sys:                 &System
    , netw:              &BTreeMap<String, Network>
    , rx_bytes_previous: &'a mut u32
    , tx_bytes_previous: &'a mut u32
    , rx_bytes_counter:  &'a mut u32
    , tx_bytes_counter:  &'a mut u32
    , rx_bytes:          &'a mut u32
    , tx_bytes:          &'a mut u32
    , rx_bytes_diff:     &'a mut i64
    , tx_bytes_diff:     &'a mut i64
    , cycle_counter:     &u8
    )
{
    let mut rx_bytes_summa = 0u32;
    let mut tx_bytes_summa = 0u32;

    for network_if in netw.values() {
        match sys.network_stats(&network_if.name) {
            Ok(netstat) => {
                rx_bytes_summa += netstat.rx_bytes as u32;
                tx_bytes_summa += netstat.tx_bytes as u32;
            }
            Err(e) => println!("{}", e),
        }
    }
    *rx_bytes_diff = rx_bytes_summa as i64 - *rx_bytes_previous as i64;
    if *rx_bytes_diff < 0 {
        *rx_bytes_diff = 0;
    }
    *tx_bytes_diff = tx_bytes_summa as i64 - *tx_bytes_previous as i64;
    if *tx_bytes_diff < 0 {
        *tx_bytes_diff = 0;
    }
    *rx_bytes_counter += *rx_bytes_diff as u32;
    *tx_bytes_counter += *tx_bytes_diff as u32;
    *rx_bytes_previous = rx_bytes_summa;
    *tx_bytes_previous = tx_bytes_summa;
    if (*cycle_counter as u16) % (1000 / (config::CYCLE_LENGTH as u16)) == 0 {
        *rx_bytes = *rx_bytes_counter;
        *tx_bytes = *tx_bytes_counter;
        *rx_bytes_counter = 0;
        *tx_bytes_counter = 0;
    }
}

pub fn get_battery_pwr(sys: &System) -> u8 {
    match sys.battery_life() {
        Ok(battery) => {
            (battery.remaining_capacity * 100.0) as u8
        }
        Err(_e) => {0u8}
    }
}

pub fn get_battery_ac(sys: &System) -> bool {
    match sys.on_ac_power() {
        Ok(is_ac_plugged) => {
            is_ac_plugged
        }
        Err(_e) => {false}
    }
}

