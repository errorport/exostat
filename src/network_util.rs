use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

use crate::config;

pub struct NetworkUtil {
    sys: System,
    rx_bytes_summa: u32,
    tx_bytes_summa: u32,
    rx_bytes_previous: u32,
    tx_bytes_previous: u32,
    rx_bytes_diff: u32,
    tx_bytes_diff: u32,
}

impl NetworkUtil {

    pub fn new() -> Self {
        NetworkUtil {
            sys: System::new(),
            rx_bytes_summa: 0,
            tx_bytes_summa: 0,
            rx_bytes_previous: 0,
            tx_bytes_previous: 0,
            rx_bytes_diff:  0,
            tx_bytes_diff:  0,
        }
    }

    #[inline]
    pub fn get_rxtx(&self) -> (u32, u32) {
        (self.rx_bytes_diff, self.tx_bytes_diff)
    }

    /*
     *  This package can provide a sum total of bytes being sent and received on a given
     *  network interface. Therefore we have to accumulate the difference between readouts
     *  and reset them in every second.
     */
    #[inline]
    fn calc_rxtx(&mut self) {
        // Lets assume that we will not crash on these. If we do, it is nothing we can do
        // under the scope of this software.
        let netw = self.sys.networks().unwrap();

        self.rx_bytes_summa = 0;
        self.tx_bytes_summa = 0;
        for network_if in netw.values() {
            let netstat = self.sys.network_stats(&network_if.name).unwrap();
            self.rx_bytes_summa += netstat.rx_bytes.as_u64() as u32;
            self.tx_bytes_summa += netstat.tx_bytes.as_u64() as u32;
        }
        self.rx_bytes_summa /= config::NETW_READ_CYCLE_s;
        self.tx_bytes_summa /= config::NETW_READ_CYCLE_s;
        self.rx_bytes_diff = self.rx_bytes_summa - self.rx_bytes_previous;
        self.tx_bytes_diff = self.tx_bytes_summa - self.tx_bytes_previous;
        self.rx_bytes_previous = self.rx_bytes_summa;
        self.tx_bytes_previous = self.tx_bytes_summa;
    }

    #[inline]
    pub fn spawn_networkstat(network_util: Arc<Mutex<Self>>) {
        // Strict 1 sec is required because of measurement accurance!
        // Do not change / call from configuration!
        let sleep_time = time::Duration::from_secs(config::NETW_READ_CYCLE_s.into());
        thread::spawn(move || {
            loop {
                network_util.lock().unwrap().calc_rxtx();
                thread::sleep(sleep_time);
            }
        });
    }
}

