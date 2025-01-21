use std::io;

use terminput::{Encoding, Event, KeyCode, parse_event};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn print_events() -> io::Result<()> {
    let stdin = io::stdin();
    let mut buf = [0; 16];

    for event in stdin.events() {
        let event: Result<Event, _> = event?.try_into();

        if let Ok(event) = event {
            println!("Event:   {:?}\r", event);
            let written = event.encode(&mut buf, Encoding::Xterm);
            if let Ok(written) = written {
                println!("Encoded: {:?}\r", &buf[..written]);
                if let Ok(Some(decoded)) = parse_event(&buf[..written]) {
                    println!("Decoded: {:?}\r", decoded);
                }
            }

            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let _stdout = MouseTerminal::from(stdout);
    print_events()?;

    Ok(())
}
