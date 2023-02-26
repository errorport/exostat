use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform};

#[derive(Default, Debug)]
pub struct NetworkUtil {
    rx_bytes_summa: u32,
    tx_bytes_summa: u32,
    rx_bytes_previous: u32,
    tx_bytes_previous: u32,
    rx_bytes_diff: u32,
    tx_bytes_diff: u32,
}

impl NetworkUtil {
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
    fn calc_rxtx(&mut self, sys: Arc<Mutex<System>>) {
        // Lets assume that we will not crash on these. If we do, it is nothing we can do
        // under the scope of this software.
        let lock = sys.lock().unwrap();
        let netw = lock.networks().unwrap();

        self.rx_bytes_summa = 0;
        self.tx_bytes_summa = 0;
        for network_if in netw.values() {
            match lock.network_stats(&network_if.name) {
                Ok(netstat) => {
                    self.rx_bytes_summa += netstat.rx_bytes.as_u64() as u32;
                    self.tx_bytes_summa += netstat.tx_bytes.as_u64() as u32;
                }
                Err(_) => {},
            }
        }
        self.rx_bytes_diff = self.rx_bytes_summa - self.rx_bytes_previous;
        self.tx_bytes_diff = self.tx_bytes_summa - self.tx_bytes_previous;
        self.rx_bytes_previous = self.rx_bytes_summa;
        self.tx_bytes_previous = self.tx_bytes_summa;
    }

    #[inline]
    pub fn spawn_networkstat(network_util: Arc<Mutex<Self>>, sys: Arc<Mutex<System>>) {
        // Strict 1 sec is required because of measurement accurance!
        // Do not change / call from configuration!
        let sleep_time = time::Duration::from_millis(1000);
        thread::spawn(move || {
            loop {
                network_util.lock().unwrap().calc_rxtx(sys.clone());
                thread::sleep(sleep_time);
            }
        });
    }
}

