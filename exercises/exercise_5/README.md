# Exercise 5: Async some more

If you solved exercise 4 like in the solution, it's not that different as the solution to exercise 3: you wait until
the button is pressed and then take action. While the CPU is no longer occupied by the wait loop (although, internally,
it is), the flow of the program is still the same. Now let's explore async programming a bit more.

## The exercise

* When you press A, the behaviour stays the same
* When you press B, the LEDs flash one by one but in the reverse direction, last one first

## Useful information

In order to wait for multiple async tasks, you'll need to use the
[select!](https://rust-lang.github.io/async-book/06_multiple_futures/03_select.html) macro.

## Solution

You can find the solution [here](./solution)

## Next exercise

The next exercise is completely up to you. Use your imagination and create something cool!
