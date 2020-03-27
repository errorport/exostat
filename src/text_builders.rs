use systemstat::{Platform, System};
use super::config;
use super::utility;

// Displaying keyboard layout
// Icon: 
pub fn get_keyboard_text() -> String {
    format!(" {}"
    , utility::get_keyboard_layout())
}

// Displaying CPU temp.
// Icon:  
pub fn get_cpu_text(system: &System) -> String {
    match system.cpu_temp() {
        Ok(_temp) => format!(" {}°C", _temp),
        Err(e) => "  ??".to_string(),
    }
}
