use std::io;

use terminput::{Event, KeyCode, UnsupportedEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

fn print_events() -> io::Result<()> {
    let stdin = io::stdin();
    for event in stdin.events() {
        let event: Result<Event, UnsupportedEvent> = event?.try_into();
        println!("Event: {:?}\r", event);
        if let Ok(Event::Key(key_event)) = event {
            if key_event.code == KeyCode::Esc {
                break;
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let _stdout = MouseTerminal::from(stdout);
    print_events()?;

    Ok(())
}
