#![no_main]
#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use cortex_m_rt::entry;
use getrandom::register_custom_getrandom;
use panic_halt as _;
use rand::RngCore;
use stm32f4xx_hal::{pac, prelude::*};

// mod mls;

// Register a custom global allocator to enable Rust Crypto and mls-rs to work
#[global_allocator]
static ALLOCATOR: emballoc::Allocator<{ 112 * 1024 }> = emballoc::Allocator::new();

// Expose the hardware RNG to software
static mut GLOBAL_RNG: Option<Box<dyn RngCore>> = None;

fn getrandom(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let rng = unsafe { GLOBAL_RNG.as_mut().ok_or(getrandom::Error::UNEXPECTED)? };
    rng.fill_bytes(buf);
    Ok(())
}

register_custom_getrandom!(getrandom);

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

        let rng = unsafe {
            GLOBAL_RNG
                .as_mut()
                .ok_or(getrandom::Error::UNEXPECTED)
                .unwrap()
        };
        let ms = 100 + 2 * rng.next_u32();

        delay.delay_ms(ms);
    }
}

/*


#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    let led = Output::new(p.PC0, Level::Low, Speed::Medium);

    let rng = Rng::new(p.RNG, Irqs);
    unsafe { GLOBAL_RNG = Some(Box::new(rng)) };

    spawner.spawn(blinky(led)).unwrap();
}

#[embassy_executor::task]
async fn blinky(mut led: Output<'static, PC0>) {
    const BLINK_INTERVAL_MS: u64 = 200;
    const INITIAL_BLINKS: usize = 10;

    let mut ticker = Ticker::every(Duration::from_millis(BLINK_INTERVAL_MS));

    // Blink a few times to show we're alive
    for _i in 0..INITIAL_BLINKS {
        led.toggle();
        ticker.next().await;
    }

    // Do MLS stuff until failure
    loop {
        led.toggle();
        ticker.next().await;

        let (a, b) = mls::test_group();
        if a != b {
            break;
        }

        led.toggle();
        ticker.next().await;
    }
}
*/
