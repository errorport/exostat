use std::sync::{Arc, Mutex};
use std::{thread, time};

use systemstat::{System, Platform, BTreeMap};

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
    fn calc_rxtx(&mut self, sys: Arc<Mutex<System>>) -> Result<(), std::io::Error> {
        let mut netw: BTreeMap<_, _>;
        let lock = sys.lock().unwrap();
        match lock.networks() {
            Ok(networks) => { netw = networks; },
            Err(e) => { return Err(e); }
        }

        self.rx_bytes_summa = 0;
        self.tx_bytes_summa = 0;
        for network_if in netw.values() {
            match lock.network_stats(&network_if.name) {
                Ok(netstat) => {
                    self.rx_bytes_summa += netstat.rx_bytes.as_u64() as u32;
                    self.tx_bytes_summa += netstat.tx_bytes.as_u64() as u32;
                }
                Err(e) => println!("{}", e),
            }
        }
        if self.rx_bytes_summa > self.rx_bytes_previous {
            self.rx_bytes_diff = self.rx_bytes_summa - self.rx_bytes_previous;
        }
        if self.tx_bytes_summa > self.tx_bytes_previous {
            self.tx_bytes_diff = self.tx_bytes_summa - self.tx_bytes_previous;
        }
        self.rx_bytes_previous = self.rx_bytes_summa;
        self.tx_bytes_previous = self.tx_bytes_summa;
        Ok(())
    }

    #[inline]
    pub fn spawn_networkstat(network_util: Arc<Mutex<Self>>, sys: Arc<Mutex<System>>) {
        thread::spawn(move || {
            loop {
                network_util.lock().unwrap().calc_rxtx(sys.clone());
                // Strict 1 sec is required because of measurement accurance!
                // Do not change / call from configuration!
                thread::sleep(time::Duration::from_millis(1000));
            }
        });
    }
}

