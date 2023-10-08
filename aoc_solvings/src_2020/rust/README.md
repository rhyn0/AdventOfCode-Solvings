# Advent of Code 2020

My solvings for AoC 2020 written in Rust. I really wanted to try out Rust and see what the hype around Rust was. While not a perfectly laid out directory, these do solve when running

```bash
cargo run --bin DAY INPUT
```

Where **DAY** is the of the form *day-X* for the code and the **INPUT** is a valid input text file for that day.

## Obtain Data

I really liked the `aocd` package in Python, and still use it as a CLI tool here. Check out the [project](https://github.com/wimglenn/advent-of-code-data).

It allows me to get my personalized input with the following call

```bash
aocd DAY YEAR > out.txt
```

By default session tokens are configured into `~/.config/aocd/token`. The package gives instructions on how to use the Developer Console in your browser to get your session token for Advent of Code.
