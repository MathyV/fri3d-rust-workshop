# Exercise 4: To async or not to async

Your solution for exercise 3 probably included some polling loop to check whether the button was pressed or not, which
generally you'd like to avoid. Hopefully you put at least a sleep inside the loop to allow other tasks to use the CPU,
or you might have seen the WatchDog kick in.

In this exercise we'll explore async programming in Rust, so we can give up control of the CPU while we are waiting for
certain events to happen.

## The exercise

Rewrite the application so that waiting on the buttons happens asynchronously.

## Useful information

* [Async programming](https://rust-lang.github.io/async-book/01_getting_started/02_why_async.html) gives some high-level
  information on how async programming works in Rust. It's not necessary to completely understand the inner workings to
  finish this exercise, but it can provide some extra insights.
* [Embassy](https://github.com/embassy-rs/embassy) support is natively available in `esp-idf-hal`. If you paid attention
  while solving the previous exercise, this one might be very easy to solve for you.

## Solution

You can find the solution [here](./solution)

## Next exercise

When you're done, proceed to the [fifth exercise](../exercise_5).
