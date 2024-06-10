use std::time::Duration;

use terminput::{Event, KeyCode, UnsupportedEvent};
use termwiz::{
    caps::Capabilities,
    terminal::{buffered::BufferedTerminal, SystemTerminal, Terminal},
};

fn main() {
    let caps = Capabilities::new_from_env().unwrap();

    let terminal = SystemTerminal::new(caps).unwrap();

    let mut terminal = BufferedTerminal::new(terminal).unwrap();
    terminal.terminal().set_raw_mode().unwrap();
    print_events(terminal);
}

fn print_events(mut terminal: BufferedTerminal<SystemTerminal>) {
    loop {
        if let Ok(Some(event)) = terminal.terminal().poll_input(Some(Duration::ZERO)) {
            let event: Result<Event, UnsupportedEvent> = event.try_into();
            println!("Event: {:?}\r", event);
            if let Ok(event) = event {
                if let Event::Key(key_event) = event {
                    if key_event.code == KeyCode::Esc {
                        break;
                    }
                }
            }
        } else {
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}
