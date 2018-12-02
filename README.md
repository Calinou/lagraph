# lagraph

[![Crates.io](https://img.shields.io/crates/v/lagraph.svg)](https://crates.io/crates/lagraph)

**lagraph** is a command-line utility that can be used to draw a ping graph over time.

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

You need rustc 1.30 or later to build lagraph from source.

## Usage

Use `lagraph --help` for a full list of command-line options.

### Examples

Ping an host at the default interval (0.5 seconds):

```bash
lagraph <host>
```

Ping an host every 5 seconds, displaying a short timestamp on the left:

```bash
lagraph -i 5 -t short <host>
```

Ping an host with a maximum displayable ping value of 100 milliseconds
and remove colors from the output:

```bash
lagraph -M 100 -C none <host>
```

### Setting true-color output by default

To use true-color output by default, you need to set the environment variable
`COLORTERM` to `truecolor`. You can make this permanent by adding the following
line to your shell startup file (such as `~/.bashrc` or `~/.zshrc`):

```bash
export COLORTERM="truecolor"
```

On Windows, this can be done using the following commands in a Command Prompt:

```bat
:: Permanently set COLORTERM to "truecolor" for the current user
setx COLORTERM truecolor
:: Sets the variable in the current shell
set COLORTERM=truecolor
```

Note that not all terminals support true-color terminal output; see
[this gist](https://gist.github.com/XVilka/8346728) for more information.
Windows 10 supports true-color terminal output since the Creators Update
(version 1703).

## License

Copyright © 2018 Hugo Locurcio and contributors

Licensed (at your option) under the [MIT](/LICENSE.MIT.md)
or [Apache 2.0](LICENSE.Apache-2.0.txt) license.
