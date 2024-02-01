#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::{delay::Delay, pac, prelude::*};

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();
    let _ = device.SYSCFG.constrain();

    let gpioc = device.GPIOC.split();

    let mut led = gpioc.pc0.into_push_pull_output();
    let mut delay = Delay::new(core.SYST, &clocks);

    loop {
        led.toggle();
        delay.delay_ms(200u8);
    }
}
