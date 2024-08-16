# Exercise 1: getting to know Rust

In the first exercise we will not yet interact with the ESP32, we will purely focus on Rust functionality. We could run these examples on our own PC but where's the fun in that?

The Rust documentation is excellent, you can find a lot of resources here:

[Learn Rust portal](https://www.rust-lang.org/learn)

As you are expected to have a programming background for this workshop, the most useful website is probably [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html).
Find the programming paradigm you're looking for, look at the syntax and related information and start programming!

## The exercise

Let's start simple: write a program that repeatedly counts from 0 to 5, sleeping for 500ms between each step.

## Expected output

```
Counter: 0
Counter: 1
Counter: 2
Counter: 3
Counter: 4
Counter: 0
Counter: 1
...
```

## Useful information

Here's some extra information if you need it:

* [Flow of Control in Rust](https://doc.rust-lang.org/rust-by-example/flow_control.html)
* [String formatting](https://doc.rust-lang.org/rust-by-example/std/str.html?highlight=string#strings). TIP: `log::info` also has a macro `log::info!`
* [Sleep](https://doc.rust-lang.org/std/thread/fn.sleep.html)
