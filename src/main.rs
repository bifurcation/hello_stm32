#![no_main]
#![no_std]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::PC0;
use embassy_time::{Duration, Ticker};
use panic_probe as _;

mod mls;

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PC0, Level::Low, Speed::Medium);

    spawner.spawn(blinky(led)).unwrap();
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static, PC0>) -> ! {
    let mut ticker = Ticker::every(Duration::from_millis(200));
    loop {
        led.toggle();
        ticker.next().await;
    }
}
