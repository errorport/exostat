use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

use crate::config;

#[derive(Default, Debug)]
pub struct BatteryUtil {
    pub capacity: u8,
    pub ac: bool,
}

impl BatteryUtil {
    #[inline]
    pub fn get_battery_pwr(&self) -> u8 {
        self.capacity
    }

    #[inline]
    pub fn get_battery_ac(&self) -> bool {
        self.ac
    }

    #[inline]
    fn update_battery_info(&mut self, sys: Arc<Mutex<System>>) {
        let lock = sys.lock().unwrap();
        self.capacity = (lock.battery_life().unwrap().remaining_capacity * 100.0) as u8;
        self.ac = lock.on_ac_power().unwrap();
    }

    #[inline]
    pub fn spawn_batterystat(battery_util: Arc<Mutex<Self>>, sys: Arc<Mutex<System>>) {
        let sleep_time = time::Duration::from_millis(config::BATTERY_READ_CYCLE_ms.into());
        thread::spawn(move || {
            loop {
                battery_util.lock().unwrap().update_battery_info(sys.clone());
                thread::sleep(sleep_time);
            }
        });
    }
}

