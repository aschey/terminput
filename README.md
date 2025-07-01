# terminput

[![crates.io](https://img.shields.io/crates/v/terminput.svg?logo=rust)](https://crates.io/crates/terminput)
[![docs.rs](https://img.shields.io/docsrs/terminput?logo=rust)](https://docs.rs/terminput)
[![Dependency Status](https://deps.rs/repo/github/aschey/terminput/status.svg?style=flat-square)](https://deps.rs/repo/github/aschey/terminput)
![license](https://img.shields.io/badge/License-MIT%20or%20Apache%202-green.svg)
[![CI](https://github.com/aschey/terminput/actions/workflows/ci.yml/badge.svg)](https://github.com/aschey/terminput/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/aschey/terminput/graph/badge.svg?token=Q0tOXGhWPY)](https://codecov.io/gh/aschey/terminput)
![GitHub repo size](https://img.shields.io/github/repo-size/aschey/terminput)
![Lines of Code](https://aschey.tech/tokei/github/aschey/terminput)

A library to abstract over various backends that provide input events, such as
key presses and mouse clicks. This was mainly created as a common interface to
the terminal backends that [Ratatui](https://crates.io/crates/ratatui) supports.

Many TUI libraries want to support input from multiple backends, but mapping
each backend's input structure into a common interface can be tedious. This
library aims to provide a uniform interface to these types.

Additionally, we supply methods for parsing and encoding ANSI escape sequences
for events.

## Feature Flags

- `std` - Adds a dependency on the standard library, required for the encoder
  and parser modules (enabled by default).
- `serde` - Adds serde serialization for event types (not enabled by default).

## Usage

Use the conversion methods provided by the integration crates to convert to and
from from `terminput`'s event types.

```rust,no_run
use crossterm::event::read;
use std::io;
use terminput::Event;
use terminput_crossterm::{to_crossterm, to_terminput};

fn main() -> io::Result<()> {
    let crossterm_event = read()?;
    let event: Result<Event, _> = to_terminput(crossterm_event);
    println!("{event:?}");

    if let Ok(event) = event {
        // Conversions work both ways
        let event2: Result<crossterm::event::Event, _> = to_crossterm(event);
        println!("{event2:?}");
    }

    Ok(())
}
```

## Event Matching

Some helpers for matching on events are included. See the
[`match_event` example](https://github.com/aschey/terminput/blob/main/examples/examples/match_event.rs).

## Backends

The following backends are currently supported via separate integration crates:

- [`crossterm`](https://crates.io/crates/crossterm) -
  [`terminput-crossterm`](https://crates.io/crates/terminput-crossterm)
- [`termion`](https://crates.io/crates/termion) -
  [`terminput-termion`](https://crates.io/crates/terminput-termion)
- [`termwiz`](https://crates.io/crates/termwiz) -
  [`terminput-termwiz`](https://crates.io/crates/terminput-termwiz)
- [`egui`](https://crates.io/crates/egui) -
  [`terminput-egui`](https://crates.io/crates/terminput-egui)

The [`Event`](https://docs.rs/terminput/latest/terminput/enum.Event.html) struct
provided by this library is an attempt to create a superset of all supported
backend functionality that TUI apps may be interested in.

The following table shows the matrix of supported features:

|                        | crossterm | termion | termwiz | egui |
| ---------------------- | --------- | ------- | ------- | ---- |
| **key press**          | ✓         | ✓       | ✓       | ✓    |
| **key release/repeat** | ✓         |         |         | ✓    |
| **mouse down**         | ✓         | ✓       | ✓       | ✓    |
| **mouse up**           | ✓         | ✓       |         | ✓    |
| **mouse move**         | ✓         |         | ✓       | ✓    |
| **mouse drag**         | ✓         | ✓       |         |      |
| **focus**              | ✓         |         |         | ✓    |
| **paste**              | ✓         |         | ✓       | ✓    |
| **resize**             | ✓         |         | ✓       |      |

## Parsing

Use the
[`Event::parse_from`](https://docs.rs/terminput/latest/terminput/enum.Event.html#method.parse_from)
method to parse an ANSI-encoded sequence of bytes into an event struct. This can
be helpful for usage with
[SSH](https://docs.rs/russh/latest/russh/server/trait.Handler.html#method.data)
or other situations where you need to read raw input from something other than a
normal TTY device.

The input parser used here was extracted from
[crossterm's implementation](https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs).

```rust,no_run
use terminput::Event;

fn read_input(input: &[u8]) {
    let event = Event::parse_from(input);

    match event {
        Ok(Some(event)) => {
            println!("Successfully parsed input: {event:?}");
        }
        Ok(None) => {
            println!("More input required");
        }
        Err(e) => {
            println!("Unable to parse input: {e:?}");
        }
    }
}
```

## Encoding

`Input` structs can also be encoded into ANSI escape sequences using
[`Event::encode`](https://docs.rs/terminput/latest/terminput/enum.Event.html#method.encode).
This can be useful if you're
[controlling a child pty](https://docs.rs/portable-pty/0.8.1/portable_pty/) and
need to send it some encoded input.

```rust
use terminput::{Encoding, Event, KeyCode, KeyEvent, KittyFlags};

let event = Event::Key(KeyEvent::new(KeyCode::Char('a')));
let mut buf = [0; 16];

// Legacy encoding
let written = event.encode(&mut buf, Encoding::Xterm);
if let Ok(written) = written {
    println!("Encoded: {:?}", &buf[..written]);
}

// Kitty encoding
let mut buf = [0; 16];
let written = event.encode(&mut buf, Encoding::Kitty(KittyFlags::all()));
if let Ok(written) = written {
    println!("Encoded: {:?}", &buf[..written]);
}
```

## Supported Rust Versions

The MSRV is currently `1.85.0`. Since Cargo's V3 resolver supports MSRV-aware
dependencies, we do not treat an MSRV bump as a breaking change.
