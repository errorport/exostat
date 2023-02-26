use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

use crate::config;

#[derive(Default, Debug)]
pub struct CPUUtil {
    pub temperature: f32,
    // CPU load is not part of this project.
}

impl CPUUtil {
    #[inline]
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    #[inline]
    fn update_cpu_info(&mut self, sys: Arc<Mutex<System>>) {
        let lock = sys.lock().unwrap();
        self.temperature = lock.cpu_temp().unwrap();
    }

    #[inline]
    pub fn spawn_cpustat(cpu_util: Arc<Mutex<Self>>, sys: Arc<Mutex<System>>) {
        let sleep_time = time::Duration::from_millis(config::CPU_READ_CYCLE_ms.into());
        thread::spawn(move || {
            loop {
                cpu_util.lock().unwrap().update_cpu_info(sys.clone());
                thread::sleep(sleep_time);
            }
        });
    }
}

