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
    match sys.battery_life()
    {
        Ok(battery) => 
            _status_text=format!(
            "{} BAT: {:02}%"
            , _status_text
            , (battery.remaining_capacity*100.0) as u8
        ),
        Err(e) => println!("{}", e),
    }
    let now = chrono::Local::now();
    _status_text 
        = format!(" {} | {}" 
                  , _status_text
                  , now.format("%Y-%m-%d %H:%M:%S"));

    _status_text
        = format!(" {} | {} {} {}"
                  , _status_text
                  , number_to_binary_str(now.time().second() as u8)
                  , number_to_binary_str(now.time().minute() as u8)
                  , number_to_binary_str(now.time().hour() as u8)
                  );

    println!("{}", _status_text);
}
