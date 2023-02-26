use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

use crate::config;

pub struct CPUUtil {
    sys: System,
    temperature: f32,
    // CPU load is not part of this project.
}

impl CPUUtil {

    pub fn new() -> Self {
        CPUUtil {
            sys: System::new(),
            temperature: 0.0,
        }
    }

    #[inline]
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }

    #[inline]
    fn update_cpu_info(&mut self) {
        self.temperature = self.sys.cpu_temp().unwrap();
    }

    #[inline]
    pub fn spawn_cpustat(cpu_util: Arc<Mutex<Self>>) {
        let sleep_time = time::Duration::from_millis(config::CPU_READ_CYCLE_ms.into());
        thread::spawn(move || {
            loop {
                cpu_util.lock().unwrap().update_cpu_info();
                thread::yield_now();
                thread::sleep(sleep_time);
            }
        });
    }
}

