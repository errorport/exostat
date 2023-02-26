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
    fn update_cpu_info(&mut self, sys: Arc<Mutex<System>>) -> Result<(), std::io::Error> {
        let lock = sys.lock().unwrap();
        match lock.cpu_temp() {
            Ok(temp) => {
                self.temperature = temp;
            },
            Err(e) => { return Err(e); },
        }
        Ok(())
    }

    #[inline]
    pub fn spawn_cpustat(cpu_util: Arc<Mutex<Self>>, sys: Arc<Mutex<System>>) {
        thread::spawn(move || {
            loop {
                cpu_util.lock().unwrap().update_cpu_info(sys.clone());
                thread::sleep(time::Duration::from_millis(config::CPU_READ_CYCLE_ms.into()));
            }
        });
    }
}

