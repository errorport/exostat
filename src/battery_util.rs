use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

use crate::config;

pub struct BatteryUtil {
    sys: System,
    capacity: f32,
    ac: bool,
}

impl BatteryUtil {

    pub fn new() -> Self {
        BatteryUtil {
            sys: System::new(),
            capacity: 0.0,
            ac: false,
        }
    }

    #[inline]
    pub fn get_battery_pwr(&self) -> f32 {
        self.capacity
    }

    #[inline]
    pub fn get_battery_ac(&self) -> bool {
        self.ac
    }

    #[inline]
    fn update_battery_info(&mut self) {
        self.capacity = self.sys.battery_life().unwrap().remaining_capacity * 100.0;
        self.ac = self.sys.on_ac_power().unwrap();
    }

    #[inline]
    pub fn spawn_batterystat(battery_util: Arc<Mutex<Self>>) {
        let sleep_time = time::Duration::from_millis(config::BATTERY_READ_CYCLE_ms.into());
        thread::spawn(move || {
            loop {
                battery_util.lock().unwrap().update_battery_info();
                thread::yield_now();
                thread::sleep(sleep_time);
            }
        });
    }
}

