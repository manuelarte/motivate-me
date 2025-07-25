# Motivate Me 💪

[![rust](https://github.com/manuelarte/motivate-me/actions/workflows/rust.yml/badge.svg)](https://github.com/manuelarte/motivate-me/actions/workflows/rust.yml)
[![PR checks](https://github.com/manuelarte/motivate-me/actions/workflows/pr-checks.yml/badge.svg)](https://github.com/manuelarte/motivate-me/actions/workflows/pr-checks.yml)
![version](https://img.shields.io/github/v/release/manuelarte/motivate-me)

An app to motivate me continue coding by staring or forking my repositories.
Every time you ⭐ or 🍴 my repositories, I get a notification in my [Raspberry Pi][raspberry-pi].

## How It Works

There is a Raspberry Pi Model 3+ running [Axum][axum] web server.
A GitHub webhook is connected to this repository, listening to the `star` and `fork` events.

When one of the event is triggered, the webhook gets called and blinks a LED. Below you can find a video showing it working:

<a href="http://www.youtube.com/watch?feature=player_embedded&v=hQUKIi9MQ8Y
" target="_blank"><img src="http://img.youtube.com/vi/hQUKIi9MQ8Y/0.jpg"
alt="video of led blinking after staring the repo" width="240" height="180" border="10" /></a>

## Technologies

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![GitHub](https://img.shields.io/badge/GitHub-%23121011.svg?logo=github&logoColor=white)](https://github.com/)

[raspberry-pi]: https://www.raspberrypi.com/
[axum]: https://docs.rs/axum/latest/axum/
