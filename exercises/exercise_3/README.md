# Exercise 3: Let's get some input

Aside from LEDs, your badge also has buttons, let's try to use one!

## The exercise

Instead of continuously looping the LEDs, wait for the A button to be pressed, light the LEDs up one after the other
once and then start waiting for the button again.

## Useful information

To be able to read the button, you'll need a
[PinDriver](https://docs.esp-rs.org/esp-idf-hal/esp_idf_hal/gpio/struct.PinDriver.html) configured as an input.

## Solution

You can find the solution [here](./solution)

## Next exercise

When you're done, proceed to the [fourth exercise](../exercise_4).
