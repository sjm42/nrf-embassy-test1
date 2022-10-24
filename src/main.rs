// main.rs

#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
// #![allow(unused_mut)]
// #![allow(non_snake_case)]
// #![deny(warnings)]

use {embassy_nrf as _, panic_halt as _};

use embassy_executor::Spawner;
use embassy_nrf::gpio::{Level, Output, OutputDrive};
use embassy_time::{Duration, Timer};

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let p = embassy_nrf::init(Default::default());

    let mut led = Output::new(p.P0_06, Level::High, OutputDrive::Standard);

    let led2 = Some([
        p0.p0_08.into_push_pull_output(Level::High).degrade(),
        p1.p1_09.into_push_pull_output(Level::High).degrade(),
        p0.p0_12.into_push_pull_output(Level::High).degrade(),
    ]);

    loop {
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}

// EOF
