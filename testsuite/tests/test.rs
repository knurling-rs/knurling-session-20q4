#![no_std]
#![no_main]

use cortex_m_rt::entry;
use knurling_session_20q4 as _; // memory layout + panic handler

#[entry]
fn main() -> ! {
    assert!(false, "TODO: Write actual tests");

    knurling_session_20q4::exit();
}
