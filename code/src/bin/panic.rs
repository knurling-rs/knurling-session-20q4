#![no_main]
#![no_std]

use knurling_session_20q4 as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::info!("main");

    panic!()
}
