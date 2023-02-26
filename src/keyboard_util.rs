use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use crate::config;

#[derive(Default, Debug)]
pub struct KbdUtil {
    pub layout: String,
    pub ledmask: String,
}

impl KbdUtil {
    pub fn get_keyboard_layout(&self) -> String {
        self.layout.clone()
    }

    pub fn get_ledmask(&self) -> u8 {
        self.ledmask.parse::<u8>().unwrap()
    }

    // Running keyboard layout getter script.
    #[inline]
    fn update_keyboard_layout(&mut self, cmd: &mut Command) {
        let _output = cmd.arg("-query").output().expect("??");
        self.layout = String::from_utf8(
            _output.stdout[_output.stdout.len()-3..].to_vec()
        ).unwrap().replace("\n", "").to_uppercase()
    }

    // Getting the ledmask
    // xset q | grep 'LED mask' | awk '{print $10}'
    fn update_ledmask(&mut self, cmd: &mut Command) {
        let mut _output = cmd.arg("q").output().expect("??");
        let mut output = String::from_utf8(_output.stdout).unwrap();
        if let Some(position) = output.rfind("LED") {
            // These magic numbers dependent on xset's output.
            output = output.split_at(position + 11).1.to_string();
            self.ledmask = output.split_at(8).0.to_string();
        }
    }

    #[inline]
    pub fn spawn_kbdstat(kdb_util: Arc<Mutex<Self>>) {
        let sleep_time = time::Duration::from_millis(config::KDB_READ_CYCLE_ms.into());
        let mut cmd_setxkbmap = Command::new("setxkbmap");
        let mut cmd_xset = Command::new("xset");
        thread::spawn(move || {
            loop {
                kdb_util.lock().unwrap().update_keyboard_layout(&mut cmd_setxkbmap);
                kdb_util.lock().unwrap().update_ledmask(&mut cmd_xset);
                thread::sleep(sleep_time);
            }
        });
    }
}

