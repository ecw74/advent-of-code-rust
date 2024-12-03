# Welcome

Hi! I'm ¯_(ツ)_/¯ Razupaltuff and I'm a big Fan of [Advent of Code](https://adventofcode.com/) and I started a small
project solving AoC Puzzles on an [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3).

## About Razupaltuff's Advent of Code Project

Welcome to an exciting journey through the [Advent of Code](https://adventofcode.com/) puzzles, uniquely implemented by
Razupaltuff! In this project, we delve into the world of coding challenges, but with a twist – the solutions are crafted
in Rust and run on an IoT platform, the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3).

## What Makes This Project Unique?

My setup is unconventional – instead of running on a typical PC, I've chosen
the [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3), a versatile microcontroller, as my target platform.
This adds an extra layer of challenge and creativity to the project.
The [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3) is not just the backbone of our computational tasks
but also hosts a web server, making our project accessible and interactive.

## Interactive Puzzle Tester Pages

For each [Advent of Code](https://adventofcode.com/) puzzle, a dedicated web page is served, similar to the AoC puzzle
pages. Here, enthusiasts can test their puzzle inputs and solutions in real-time. It's a hands-on way to engage with the
puzzles and see immediate results. These pages are more than just testing grounds; they are learning and exploration
hubs for coding enthusiasts at any skill level.

## See the Code

Transparency and learning are core to my philosophy. On each puzzle tester page, I display the Rust code used to solve
the corresponding puzzle. This provides a great opportunity for learners and experienced coders alike to understand the
intricacies of solving complex problems in Rust, and to see how code can be optimized for a microcontroller environment.

## See the Hardware

The [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3) is a dual-core XTensa LX7 microcontroller,
operating up to 240 MHz, and features Wi-Fi and Bluetooth 5 connectivity. It offers 512 KB of internal SRAM,
45 programmable GPIOs, and is built using 40 nm technology for efficient power usage and compact design.
This chip is ideal for various IoT applications due to its robust and integrated platform.

## See the Infrastructure

To provide access to the puzzle testers running on [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3), I
use the following infrastructure:

1. Cloudflare Tunnel: A secure tunnel is established using Cloudflare, allowing access from the internet to a Traefik
   load balancer. This ensures that the connection is reliable and protected from unauthorized access.

2. Traefik Load Balancer: The Traefik load balancer distributes incoming requests to the
   appropriate [ESP32-S3](https://www.espressif.com/en/products/socs/esp32-s3) devices. This setup ensures efficient
   handling of multiple requests and keeps the system responsive for users accessing the puzzle testers.

## Join the Adventure

Whether you're a seasoned coder, a Rust enthusiast, or just someone curious about coding puzzles, the project offers a
unique platform to test, learn, and grow. I invite you to explore the puzzles, try your hand at solving them, and even
peek into the world of microcontroller programming. Dive into the world of coding challenges with a twist, and
experience the [Advent of Code](https://adventofcode.com/)!

```rust
fn main() {
    let input = vec![
        "Uryyb, Nqirag bs Pbqr Ragnuvfg!",
        "Ernql gb hajenc gur chmmryf?",
        "Rnpu qnl oevatf n arj punyyratr, yvxr n qvtvgny nqirag pnyynaqre.",
        "Teno lbhe pbqvat ung naq yrg gur srfgvir pbqvat sha ortva!",
        "Znl lbhe pbqr or zreel naq oht-serr.",
        "Unccy pbqvat!",
        "¯_(ツ)_/¯",
    ];

    for line in input {
        println!("{}", decode(line));
    }
}

fn decode(input: &str) -> String { todo!() }
```