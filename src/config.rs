use std::env;

pub fn active_color() -> String {
    env::args().collect::<Vec<String>>()[1].clone()
}

#[allow(non_upper_case_globals)]
pub const  CYCLE_LENGTH_ms:                 u32             = 200;
#[allow(non_upper_case_globals)]
pub const  BATTERY_READ_CYCLE_ms:           u32             = 30000;
#[allow(non_upper_case_globals)]
pub const  CPU_READ_CYCLE_ms:               u32             = 10000;
#[allow(non_upper_case_globals)]
pub const  KDB_READ_CYCLE_ms:               u32             = 250;
#[allow(non_upper_case_globals)]
pub const  NETW_READ_CYCLE_s:               u32             = 1;
pub const  BINARY_DOT_SIZE:                 u8              = 4;
pub const  BINARY_DOT_HORIZONTAL_SPACING:   u8              = 1;
pub const  BINARY_DOT_OFFSET:               u8              = 6;
pub const  KEYLOCK_DOT_SIZE:                u8              = 7;
pub const  KEYLOCK_DOT_OFFSET:              u8              = 9;
pub const  KEYLOCK_DOT_HORIZONTAL_SPACING:  u8              = 2;
