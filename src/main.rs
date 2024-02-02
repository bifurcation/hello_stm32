#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use cortex_m_rt::entry;
use panic_halt as _;
use rand::RngCore;
use stm32f4xx_hal::{pac, prelude::*};

// Register a custom global allocator to enable Rust Crypto and mls-rs to work
#[global_allocator]
static ALLOCATOR: emballoc::Allocator<{ 112 * 1024 }> = emballoc::Allocator::new();

// Expose the hardware RNG to software
static mut GLOBAL_RNG: Option<Box<dyn RngCore>> = None;

#[entry]
fn main() -> ! {
    let device = pac::Peripherals::take().unwrap();

    // Set up the clocks
    let rcc = device.RCC.constrain();
    let clocks = rcc.cfgr.require_pll48clk().freeze();
    let _ = device.SYSCFG.constrain();
    let mut delay = device.TIM2.delay_ms(&clocks);

    // Set up the global RNG
    let rng = device.RNG.constrain(&clocks);
    unsafe { GLOBAL_RNG = Some(Box::new(rng)) };

    // Get handles to the GPIO interface
    let gpioc = device.GPIOC.split();
    let mut led = gpioc.pc0.into_push_pull_output();

    loop {
        led.toggle();

        let rng = unsafe { GLOBAL_RNG.as_mut().unwrap() };
        let ms = 100 + 2 * rng.next_u32();

        delay.delay_ms(ms);
    }
}
