# lagraph

[![Crates.io](https://img.shields.io/crates/v/lagraph.svg)](https://crates.io/crates/lagraph)

**lagraph** is a command-line utility that can be used to draw a ping graph over time.

**Note that Windows is not supported yet.**

## Features

- Bars drawn using Unicode or ASCII character, supporting "half characters"
  for increased precision
- True-color output with configurable saturation
- Setting ping interval and/or count
- Optional short or long timestamp

## Use cases

- Monitoring your connection quality and stability over time.
  - This is especially useful when using Wi-Fi or mobile connections such as 4G.

## Installation

### Using `cargo`

If you already have [Rust](https://rust-lang.org/) installed, you can use
[Cargo](https://crates.io/) to build and install lagraph:

```bash
cargo install lagraph
```

## License

Copyright © 2018 Hugo Locurcio and contributors

Licensed (at your option) under the [MIT](/LICENSE.MIT.md)
or [Apache 2.0](LICENSE.Apache-2.0.txt) license.
