# terminput

A library to provide an abstraction over various backends that provide input
sources such as key and mouse events. This was mainly created as a common
interface to the terminal backends that
[Ratatui](https://crates.io/crates/ratatui) supports.

Many TUI libraries want to support input from multiple backends, but mapping
each backend's input types into a common interface can be tedious. This library
aims to provide a uniform interface to these types.

Additionally, we supply methods for parsing and encoding ANSI escape sequences
for events.

## Usage

```rust,no_run
use crossterm::event::read;
use std::io;
use terminput::Event;

fn main() -> io::Result<()> {
    let crossterm_event = read()?;
    let event: Result<Event, _> = crossterm_event.try_into();
    println!("{event:?}");

    if let Ok(event) = event {
        // Conversions work both ways
        let event2: Result<crossterm::event::Event, _> = event.try_into();
        println!("{event2:?}");
    }

    Ok(())
}
```

## Backends

The following backends are currently supported. All backends are disabled by
default and each can be enabled with a feature flag of the same name.

- [`crossterm`](https://crates.io/crates/crossterm)
- [`termion`](https://crates.io/crates/termion)
- [`termwiz`](https://crates.io/crates/termwiz)
- [`egui`](https://crates.io/crates/egui)

The `Event` struct provided in this library is an attempt to provide a superset
of all supported backend functionality that TUI apps may be interested in.

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

Use the `parse_event` function to parse an ANSI-encoded sequence of bytes into
an event instance. This can be helpful for usage with SSH or other situations
where you need to read raw input from something other than a normal TTY device.

The input parser used here was extracted from
[crossterm's implementation](https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs).

```rust,no_run
use terminput::parse_event;

fn read_input(input: &[u8]) {
    let event = parse_event(input);

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

`Input` structs can also be encoded into ANSI escape sequences. This can be
useful if you're controlling a child pty and need to send it some encoded input.

```rust
use terminput::{Encoding, Event, KeyCode, KeyEvent, KittyFlags};

fn main() {
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
}
```
