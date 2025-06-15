#![no_std]
#![no_main]

use esp_hal::clock::CpuClock;
use esp_hal::delay::Delay;
use esp_hal::gpio::{Level, Output};
use esp_hal::main;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

#[main]
fn main() -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 72 * 1024);

    // GPIO2をOutputピンとして初期化（初期状態Low）- XIAO ESP32-S3のD1ピン
    let mut led = Output::new(peripherals.GPIO2, Level::Low);

    let _timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    // WiFi初期化も一旦コメントアウト
    /*
    let _init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();
    */

    let delay = Delay::new();

    loop {
        led.toggle();
        delay.delay_millis(500);
    }
}
