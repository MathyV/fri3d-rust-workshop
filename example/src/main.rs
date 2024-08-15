use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::sys::esp_random;
use smart_leds::hsv::{hsv2rgb, Hsv};
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

    log::info!("Hello, world!");

    let peripherals = Peripherals::take()
        .expect("Could not get hardware lock");

    let mut ws2812 = Ws2812Esp32Rmt::new(
        peripherals.rmt.channel0,
        peripherals.pins.gpio12,
    ).unwrap();

    println!("Start NeoPixel rainbow!");

    let mut hue = unsafe { esp_random() } as u8;
    loop {
        let pixels = std::iter::repeat(hsv2rgb(Hsv {
            hue,
            sat: 255,
            val: 50,
        }))
            .take(25);
        ws2812.write(pixels).unwrap();

        sleep(Duration::from_millis(100));

        hue = hue.wrapping_add(10);
    }
}
