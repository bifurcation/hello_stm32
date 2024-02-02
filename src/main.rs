#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rand::RngCore;
use stm32f4xx_hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::peripheral::Peripherals::take().unwrap();

    // Set up the clocks
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.use_hse(12.MHz()).sysclk(168.MHz()).freeze();
    let _ = device.SYSCFG.constrain();
    let mut delay = core.SYST.delay(&clocks);

    // Set up the global RNG
    let mut rng = device.RNG.constrain(&clocks);

    // Get handles to the GPIO interface
    let gpioc = device.GPIOC.split();
    let mut led = gpioc.pc0.into_push_pull_output();

    loop {
        led.toggle();

        let ms = 100 + 2 * rng.next_u32();

        delay.delay_ms(ms);
    }
}
