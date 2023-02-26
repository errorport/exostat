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
    fn update(&mut self, cmd: &mut Command, lock_key: Arc<LockKey>) {
        self.numlock = lock_key.state(LockKeys::NumberLock).unwrap();
        self.capslock = lock_key.state(LockKeys::CapitalLock).unwrap();

        let _output = cmd.arg("-query").output().unwrap();
        self.layout = String::from_utf8(_output.stdout).unwrap()
            .split('\n').collect::<Vec<&str>>()[2].to_string()
            .replace("layout:", "").replace(" ", "");
    }

    #[inline]
    pub fn spawn_kbdstat(kdb_util: Arc<Mutex<Self>>) {
        let sleep_time = time::Duration::from_millis(config::KDB_READ_CYCLE_ms.into());
        let mut cmd_setxkbmap = Command::new("setxkbmap");
        thread::spawn(move || {
            let lock_key = Arc::new(LockKey::new());
            loop {
                kdb_util.lock().unwrap().update(&mut cmd_setxkbmap, lock_key.clone());
                thread::yield_now();
                thread::sleep(sleep_time);
            }
        });
    }
}

