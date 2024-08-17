use esp_idf_hal::prelude::Peripherals;
use smart_leds::{RGB8};
use smart_leds_trait::SmartLedsWrite;
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    const NUM_LEDS: usize = 5;

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()
        .expect("Could not get hardware lock");

    let mut ws2812 = Ws2812Esp32Rmt::new(
        peripherals.rmt.channel0,
        peripherals.pins.gpio12,
    ).unwrap();

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    loop
    {
        for i in 0..NUM_LEDS
        {
            log::info!("Counter: {}", i);
            data[i].b = 0x33;
            ws2812.write(data).unwrap();
            sleep(Duration::from_millis(500));
            data = empty;
        }
    }
}
