use std::io;

use terminput::{Event, KeyCode, UnsupportedEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn print_events() {
    let stdin = io::stdin();
    for event in stdin.events() {
        let event: Result<Event, UnsupportedEvent> = event.unwrap().try_into();
        println!("Event: {:?}\r", event);
        if let Ok(event) = event {
            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
}

fn main() {
    // setup terminal
    let stdout = io::stdout().into_raw_mode().unwrap();
    let _stdout = MouseTerminal::from(stdout);
    print_events();
}
