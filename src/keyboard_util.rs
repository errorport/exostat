use std::process::Command;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use lock_keys::*;

use crate::config;

#[derive(Debug)]
pub struct KbdUtil {
    pub layout: String,
    pub numlock: LockKeyState,
    pub capslock: LockKeyState,
}

impl KbdUtil {
    pub fn new() -> Self {
        KbdUtil {
            layout: "".to_string(),
            numlock: LockKeyState::Disabled,
            capslock: LockKeyState::Disabled,
        }
    }

    pub fn get_keyboard_layout(&self) -> String {
        self.layout.clone()
    }

    pub fn get_ledmask(&self) -> (bool, bool) {
        (self.numlock == LockKeyState::Enabled, self.capslock == LockKeyState::Enabled)
    }

    // Running keyboard layout getter script.
    #[inline]
    fn update(&mut self, cmd: &mut Command) {
        let lock_key = LockKey::new();
        self.numlock = lock_key.state(LockKeys::NumberLock).unwrap();
        self.capslock = lock_key.state(LockKeys::CapitalLock).unwrap();

        let _output = cmd.arg("-query").output().expect("??");
        self.layout = String::from_utf8(
            _output.stdout[_output.stdout.len()-3..].to_vec()
        ).unwrap().replace("\n", "").to_uppercase()

    }

    #[inline]
    pub fn spawn_kbdstat(kdb_util: Arc<Mutex<Self>>) {
        let sleep_time = time::Duration::from_millis(config::KDB_READ_CYCLE_ms.into());
        let mut cmd_setxkbmap = Command::new("setxkbmap");
        thread::spawn(move || {
            loop {
                kdb_util.lock().unwrap().update(&mut cmd_setxkbmap);
                thread::sleep(sleep_time);
            }
        });
    }
}

