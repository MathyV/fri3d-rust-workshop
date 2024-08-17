# Exercise 2: Add a touch of ESP32

In this exercise, we'll do our first calls to the ESP32. Your badge is equipped with some cool addressable RGB LEDs,
let's try to make them light up!

## The exercise

In the previous exercise you made your program count from 0 to 4 in a continuous loop. Which is handy, because your
badge has 5 LEDs, with addresses 0, to 4. What a coincidence! Make the LEDs light up in BLUE one after the other, with a
500ms interval. Only one LED should light up at the same time.

## Useful information

The LEDs on the badge are of the WS2812C type. Luckily you don't need to write your own implementation of the protocol,
there's a crate we can use! `smart-leds` provides a general API for controlling LEDs while `ws2812-esp32-rmt-driver`
provides the actual protocol implementation. You'll need to add the following dependencies to your Cargo.toml:

```
ws2812-esp32-rmt-driver = { version = "0.9.0", features = ["smart-leds-trait"] }
smart-leds = "*"
smart-leds-trait = "*"
```

To find out how to use the library, have a look at
[the crate page for the driver](https://crates.io/crates/ws2812-esp32-rmt-driver). We'll provide this additional hint
a final time: the Rust community has a very strong focus on coding by example.

To configure the driver, you will need to know the pin the LEDs are connected to. Take a good look at the back of your
badge ;-)

The way access to the pins (and any other ESP32 hardware) works with `esp-idf-sys` is that you `take()` control of the
interface and are responsible for it from then on. This is completely according to the principles of
[ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) in Rust.

You'll also need to deal with [error handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html) for this
exercise.

## Solution

You can find the solution [here](./solution)

## Next exercise

When you're done, proceed to the [third exercise](../exercise_3).
