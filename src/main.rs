extern crate chrono;
extern crate systemstat;

use systemstat::{System, Platform};
use chrono::Timelike;

fn number_to_binary_str(num: u8) -> String
{
    let mut binary_str: String = "".to_string();
    for bit in 0..8 
    {
        binary_str += match num>>bit&0x01
        {
            1 => "^c#00f0aa^●^d^",
            _ => "●",
        }
    }
    binary_str
}

fn main()
{
    let mut _status_text: String = "".to_string();
    let sys = System::new();
  
    // Displaying CPU temp.
    match sys.cpu_temp()
    {
        Ok(_temp) =>
            _status_text = format!(
                " {}°C"
                , _temp
            ),
        Err(e) =>
            println!("{}", e)
    }

    // Displaying network statistics.
    let network_interfaces = sys.networks().unwrap();
    let mut rx_bytes = 0u64;
    let mut tx_bytes = 0u64;
    for network_if in network_interfaces.values()
    {
        match sys.network_stats(&network_if.name)
        {
            Ok(netstat) =>
            {
              rx_bytes += netstat.rx_bytes as u64;
              tx_bytes += netstat.tx_bytes as u64;
              // TODO:
              // Reset network stats to get
              // per secundum value from the
              // lastmeasurement.
              // netstat.rx_bytes = 0;
              // netstat.tx_bytes = 0;
            }
            Err(e) => println!("{}", e)
        }
    }
    _status_text = format!(
                "{} |  {:04}kB -  {:04}kB"
                , _status_text
                , (rx_bytes/1024) as u16
                , (tx_bytes/1024) as u16
    );

    // Displaying battery status.
    let mut _battery_icon = "".to_string();
    let mut _battery_capacity = 0u8;
    match sys.battery_life()
    {
        Ok(battery) =>
        {
            let pwr = (battery.remaining_capacity*100.0) as u8;
            if pwr > 20 { _battery_icon = "".to_string(); }
            if pwr > 40 { _battery_icon = "".to_string(); }
            if pwr > 60 { _battery_icon = "".to_string(); }
            if pwr > 80 { _battery_icon = "".to_string(); }
            _battery_capacity = (battery.remaining_capacity*100.0) as u8;
        },
        Err(e) => println!("{}", e),
    }

    // Displaying AC status.
    match sys.on_ac_power()
    {
        Ok(_is_ac_plugged) =>
        {
            if _is_ac_plugged {
                _battery_icon = format!(
                    "^c#00f0aa^{}^d^"
                    , _battery_icon
                );
            }
        },
        Err(e) =>
            println!("{}", e)
    }
    _status_text = format!(
            "{} | {} {:02}%"
            , _status_text
            , _battery_icon
            , _battery_capacity
    );

    // Displaying local time.
    let now = chrono::Local::now();
    _status_text 
        = format!(" {} | {}" 
                  , _status_text
                  , now.format("%Y-%m-%d %H:%M:%S"));
    // Displaying binary-watch format time.
    _status_text
        = format!(" {} | {} {} {}"
                  , _status_text
                  , number_to_binary_str(now.time().second() as u8)
                  , number_to_binary_str(now.time().minute() as u8)
                  , number_to_binary_str(now.time().hour() as u8)
                  );

    println!("{}", _status_text);
}
