use std::process::Command;
use super::config;
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
pub fn get_keyboard_layout(
    //path_to_root: &'static str
    //, script_filename: &'static str
    ) -> String {
    let _output = Command::new("/bin/bash")
        .arg(format!("{}/{}", config::PATH_TO_ROOT, config::GETKBL_SCRIPT))
        .output()
        .expect("??");
    String::from_utf8(_output.stdout)
        .unwrap()
        .replace("\n", "")
}

// Clock stuff.
// Works with only exodwm or text-color patched dwm.
pub fn number_to_binary_str(num: u8) -> String {
    let mut binary_str: String = "".to_string();
    for bit in 0..8 {
        binary_str = match num >> bit & 0x01 {
            1 => format!("{}^c{}^●^d^", binary_str, config::ACTIVE_COLOR),
            _ => format!("{}●", binary_str),
        }
    }
    binary_str
}
