#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]

use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::peripherals::TIM3;
use embassy_stm32::usart::UartTx;
use embassy_stm32::{mode, rcc};
use rtic::app;
use rtic_monotonics::stm32::prelude::*;
use rtic_sync::arbiter::Arbiter;
use rtictest as _; // global logger + panicking-behavior + memory layout

mod version;

// Define the monotonic and set it to 1MHz tick rate
stm32_tim3_monotonic!(Mono, 1_000_000);

pub type Instant = <Mono as Monotonic>::Instant;
pub type Duration = <Mono as Monotonic>::Duration;

// use cortex_m_rt::{exception, ExceptionFrame};
// Hardfaults on panics!.
// https://embassy.dev/book/dev/faq.html#_my_binary_is_still_big_filled_with_stdfmt_stuff
// #[exception]
// unsafe fn HardFault(_frame: &ExceptionFrame) -> ! {
//     cortex_m::peripheral::SCB::sys_reset()
// }

#[app(device = embassy_stm32, peripherals = true, dispatchers = [TIM16, TIM17])]
mod app {
    use super::*;
    use core::mem::MaybeUninit;

    #[shared]
    struct Shared {
        some_shared: usize,
    }

    #[local]
    struct Local {
        led: Output<'static>,
    }

    #[init(local = [uart_dma_buf: [u8; 512] = [0; 512], uart_tx_arbiter: MaybeUninit<Arbiter<UartTx<'static, mode::Async>>> = MaybeUninit::uninit()])]
    fn init(cx: init::Context) -> (Shared, Local) {
        let mut config = embassy_stm32::Config::default();
        {
            use embassy_stm32::rcc::*;
            config.rcc.hsi = true;
            // config.rcc.hse = Some(Hse {
            //     freq: Hertz(16_000_000),
            //     mode: HseMode::Oscillator,
            // });
            config.rcc.pll = Some(Pll {
                source: PllSource::HSI,
                //source: PllSource::HSE,
                prediv: PllPreDiv::DIV1,
                mul: PllMul::MUL8,
                divr: Some(PllRDiv::DIV2),
                divp: None,
                divq: None,
            });
            config.rcc.sys = Sysclk::PLL1_R;
            config.enable_debug_during_sleep = true;
        }
        let p: embassy_stm32::Peripherals = embassy_stm32::init(config);

        // Setup Monotonic clock.
        Mono::start(rcc::frequency::<TIM3>().0);

        defmt::info!("start");

        // Spawn tasks.
        housekeeping::spawn().ok();
        communication::spawn().ok();
        (
            Shared { some_shared: 0 },
            Local {
                led: Output::new(cx.device.PB2, Level::High, Speed::Low),
            },
        )
    }

    #[task(local = [led])]
    async fn housekeeping(cx: housekeeping::Context) {
        loop {
            cx.local.led.set_high();
            Mono::delay(100.millis()).await;

            cx.local.led.set_low();
            Mono::delay(900.millis()).await;
        }
    }

    //#[task(priority = 0)]
    // Does not work with priority 2.
    #[task(priority = 2)]
    async fn communication(mut cx: communication::Context) {
        loop {
            defmt::info!("spin");
            Mono::delay(Duration::millis(1000)).await;
        }
    }
}
