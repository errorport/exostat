use std::process::Command;

use crate::config;

// Updating X rootserver's window name.
#[inline]
pub fn setxroot(cmd: &mut Command, _status_text: &String) {
    let _output = cmd.arg("-name").arg(_status_text).output()
        .expect("Failed to set X root window name!");
    // println!("setxroot status: {:?}", output.status);
}

// Running keyboard layout getter script.
#[inline]
pub fn get_keyboard_layout(cmd: &mut Command) -> String {
    let _output = cmd.arg("-query").output()
        .expect("??");
    String::from_utf8(
        _output.stdout[_output.stdout.len()-3..].to_vec()
    ).unwrap().replace("\n", "").to_uppercase()
}

// Getting numlock and capslock indicators.
// xset q | grep 'LED mask' | awk '{print $10}'
#[inline]
pub fn get_keyboard_ledmask(cmd: &mut Command) -> String {
    let mut _output = cmd.arg("q").output().expect("??");

    let mut output = String::from_utf8(_output.stdout).unwrap();
    if let Some(position) = output.rfind("LED") {
        // THese magic numbers dependent on xset's output.
        output = output.split_at(position + 11).1.to_string();
        output = output.split_at(8).0.to_string();
    }

    let ledmask = output.parse::<u8>().unwrap();

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

#[inline]
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
#[inline]
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

