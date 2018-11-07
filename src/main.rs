// lagraph: Display a ping graph in a terminal
//
// Copyright © 2018 Hugo Locurcio and contributors
// Licensed (at your option) under the MIT or Apache 2 license,
// see `LICENSE.MIT.md` and `LICENSE.Apache-2.0.txt` for details.

extern crate ansi_term;
extern crate chrono;
extern crate clap;
extern crate terminal_size;
use ansi_term::Colour::{Blue, Green, Red, Yellow, RGB};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};
use chrono::prelude::*;
use clap::{load_yaml, value_t, value_t_or_exit, App};
use std::process::Command;
use std::{cmp, env, thread, time};
use terminal_size::{terminal_size, Width};

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Initialize settings (with defaults when applicable)

    // The host to ping (required)
    let host = value_t_or_exit!(matches, "host", String);

    // Wait time in seconds between each ping
    // NOTE: the actual value varies depends on ping and can't be lower than the ping time
    // Defaults to 0.5 seconds (2 pings per second)
    let wait = (value_t!(matches, "interval", f32).unwrap_or(0.5) * 1000.0) as u64;

    // Maximal ping in the graph (values above will be cut off)
    // Defaults to 300 (a sane baseline for most connections)
    let max_ping = value_t!(matches, "max_ping", u32).unwrap_or(300);

    // The characters to use for drawing the bars
    let bar_character;
    let cap_half_character;
    // The character that separates the ping value and the bar (can be changed by styles)
    let separator;

    // Number of pings to do (0 = no limit, which is the default)
    let mut count = value_t!(matches, "count", u32).unwrap_or(0);

    // Used to break out of the loop if needed
    let finite = if count != 0 { true } else { false };

    // Default style is "bar"
    let style = value_t!(matches, "style", String).unwrap_or(String::from("bar"));

    // All available bar styles (not all of them support half-caps)
    // If half-caps are not supported, then the half cap is just an empty string
    match style.as_ref() {
        "bar" => {
            bar_character = "▄";
            cap_half_character = "▖";
            separator = "▏";
        }
        "block" => {
            bar_character = "█";
            cap_half_character = "▌";
            separator = "▏";
        }
        "line" => {
            bar_character = "▁";
            cap_half_character = "";
            separator = "▏";
        }
        _ => {
            // Also handles "ascii" case
            bar_character = "=";
            cap_half_character = "-";
            separator = "|";
        }
    }

    // The color scheme to use:
    // - `truecolor` does not work on all platforms and terminals, but usually looks best
    //   and is the most accurate
    // - `256color` is more compatible but not as accurate
    // - `16color` is the most compatible along with `none` (disabled colors)
    // The default value varies depending on the environment variable COLORTERM
    let color_default = match env::var("COLORTERM") {
        Ok(value) => {
            if value == "truecolor" || value == "24bit" {
                "truecolor"
            } else {
                "16color"
            }
        }
        Err(_) => "16color",
    };

    let color = value_t!(matches, "color", String).unwrap_or(String::from(color_default));

    // Whether to print a header or not (default: print header)
    let print_header = !matches.is_present("no_header");

    if print_header {
        // If the ping count is 0, display "unlimited"
        let count_print = if count == 0 {
            String::from("unlimited")
        } else {
            count.to_string()
        };

        let header: &[ANSIString<'static>] = &[
            Style::new().bold().paint("Ping graph for host "),
            Green.bold().underline().paint(host.to_string()),
            Style::new().bold().paint(" - updated every "),
            Yellow.bold().paint((wait as f32 / 1000.0).to_string()),
            Style::new().bold().paint(" seconds - performing "),
            Blue.bold().paint(count_print),
            Style::new().bold().paint(" pings:"),
        ];

        println!("{}", ANSIStrings(header));
    }

    loop {
        // Get the terminal width every iteration so that the ping graph
        // adapts to terminal resizes
        let size = terminal_size();
        let width = if let Some((Width(w), _)) = size {
            // The left column's typical width is substracted to the total width
            // FIXME: Take the timestamp into account if present
            w as u32 - 12
        } else {
            80
        };

        // Call the system `ping` command
        // Windows uses "-n" to set the ping count, UNIX-like platforms use "-c"
        let output = Command::new("ping")
            .arg(if cfg!(windows) { "-n" } else { "-c" })
            .arg("1")
            .arg(host.to_string())
            .output()
            .unwrap();

        let status = output.status;
        // Default to 0 ping in case of errors (for the wait time to make sense)
        let mut ping = 0.0;

        if status.success() {
            // Ping successful
            let output_string = String::from_utf8_lossy(&output.stdout);

            // Trim the output to keep only a string containing the ping value
            // The output line is the 2nd one on Linux and macOS and the 3rd one on Windows
            // There is a space before "ms" on Linux and macOS but not on Windows
            // (in some languages such as English), so it must be trimmed
            let ping_string = output_string
                .split("\n")
                .nth(if cfg!(windows) { 2 } else { 1 })
                .unwrap()
                .split("=")
                .last()
                .unwrap()
                .split("ms")
                .nth(0)
                .unwrap()
                .trim();

            // Convert string to floating-point number
            ping = ping_string.parse().unwrap();

            // Ratio of full screen width
            let ratio = ping / max_ping as f32;

            // How many bars to draw
            // String.repeat() wants an `usize`
            let number_of_bars = (ratio * width as f32) as usize;

            // Ping as displayed as the bars (inaccurate)
            let bar_ping = (number_of_bars as f32 / width as f32 / ratio * ping).round();
            // Ping as displayed by the bars + 1 (also inaccurate)
            let bar_ping_next = ((number_of_bars + 1) as f32 / width as f32 / ratio * ping).round();
            // Average of bar_ping and bar_ping_next (to check if a cap should be added for precision)
            let bar_ping_half = (bar_ping + bar_ping_next) / 2.0;

            // Don't draw caps if user has turned them off or there is no need
            // (in case there is sub-millisecond precision thanks to large terminal width)
            // (or if the bar has overflowed)
            let draw_cap;
            if max_ping <= width || number_of_bars >= (width) as usize {
                draw_cap = false;
            } else {
                draw_cap = true;
            }

            let cap;
            // Draw a cap for sub-character precision
            if draw_cap && ping >= bar_ping_half && ping < bar_ping_next {
                // There should be a cap
                cap = cap_half_character;
            } else {
                // There shouldn't be a cap ("empty" cap)
                cap = "";
            }

            // Ping bar/number color
            let ping_color;

            match color.as_ref() {
                "truecolor" => {
                    // The higher this value, the less colored the bars are
                    let desaturation = 255 - value_t!(matches, "saturation", u8).unwrap_or(160);

                    // No red at 0% bar width, fully red at 100% bar width
                    let red = cmp::min(
                        (255.0 + (512.0 - 2.0 * desaturation as f32) * (ratio - 0.5)) as i16,
                        255 as i16,
                    ) as u8;

                    // Fully green from 0% to 50% bar width, no green at 100% bar width
                    let green = cmp::min(
                        255,
                        cmp::max(
                            (255.0 - (512.0 - desaturation as f32 * 2.0) * (ratio - 0.5)) as i16,
                            desaturation as i16,
                        ),
                    ) as u8;

                    // Constant value
                    let blue = desaturation;

                    ping_color = RGB(red, green, blue);
                }
                _ => {
                    if ratio >= 0.0 && ratio <= 0.33 {
                        ping_color = Green;
                    } else if ratio > 0.33 && ratio <= 0.67 {
                        ping_color = Yellow;
                    } else {
                        ping_color = Red;
                    }
                }
            }

            // String containing the whole bar (including the cap)
            let bar = [
                bar_character.repeat(cmp::min(number_of_bars, width as usize)),
                cap.to_string(),
            ]
                .join("");

            // Timestamp

            let now = Local::now();
            let timestamp_setting =
                value_t!(matches, "timestamp", String).unwrap_or(String::from("none"));
            let timestamp = match timestamp_setting.as_ref() {
                "short" => {
                    // Only hours, minutes and seconds
                    now.format("%H:%M:%S").to_string()
                }
                "full" => {
                    // Full date in a SQL-like format
                    now.format("%Y-%m-%d %H:%M:%S").to_string()
                }
                _ => {
                    // This also matches the default "none" setting
                    // No timestamp
                    String::from("")
                }
            };

            // Print the ping graph (and cap if needed to avoid line breaks)
            println!(
                "{} {}  {} {}",
                // The optional timestamp
                timestamp,
                // The ping value with a " ms" suffix (right-aligned)
                ping_color.paint(format!(
                    "{:>7}",
                    [(ping as u32).to_string(), "ms".to_string()].join(" ")
                )),
                // The separator character (depends on style)
                separator,
                // The bar
                ping_color.paint(bar)
            );
        } else {
            // Ping failed
            let err = output.stderr;
            println!("{}", Red.bold().paint(String::from_utf8_lossy(&err)));
        }

        // Decrement the ping counter if a ping count was specified
        if finite {
            count -= 1;
        }

        // Exit if the limit of pings has been reached
        if finite && count == 0 {
            break;
        }

        // Sleep after receiving the ping result or an error
        // Try to sleep around the "wait" time in milliseconds
        // by compensating the value based on the ping time
        let sleep = time::Duration::from_millis(wait - cmp::min(ping as u64, wait as u64));
        thread::sleep(sleep);
    }
}
