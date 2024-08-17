use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::gpio::*;
use esp_idf_hal::task::*;
use smart_leds::{RGB8};
use smart_leds_trait::SmartLedsWrite;
use std::thread::sleep;
use std::time::Duration;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;
use futures::{
    future::FutureExt,
    pin_mut,
    select,
};

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

    let mut button_a = PinDriver::input(peripherals.pins.gpio39).unwrap();
    button_a.set_pull(Pull::Up).unwrap();
    let mut button_b = PinDriver::input(peripherals.pins.gpio40).unwrap();
    button_b.set_pull(Pull::Up).unwrap();

    let mut data: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];
    let empty: [RGB8; NUM_LEDS] = [RGB8::default(); NUM_LEDS];

    block_on(
        async {
            loop
            {
                ws2812.write(empty).unwrap();

                let wait_a = button_a.wait_for_low().fuse();
                let wait_b = button_b.wait_for_low().fuse();

                pin_mut!(wait_a, wait_b);

                let counter: Vec<usize>;

                select! {
                    _ = wait_a => counter = (0..NUM_LEDS).collect(),
                    _ = wait_b => counter = (0..NUM_LEDS).rev().collect(),
                };

                for i in counter
                {
                    log::info!("Counter: {}", i);
                    data[i].b = 0x33;
                    ws2812.write(data).unwrap();
                    sleep(Duration::from_millis(500));
                    data = empty;
                }
            }
        }
    );
}
