use std::process::Command;

use crate::config;

// Updating X rootserver's window name.
#[inline]
pub fn setxroot(cmd: &mut Command, _status_text: &String) {
    let _output = cmd.arg("-name").arg(_status_text).output().expect("Failed to set X root window name!");
}

#[inline]
pub fn get_keyboard_ledmask_str(ledmask: u8) -> String {
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

