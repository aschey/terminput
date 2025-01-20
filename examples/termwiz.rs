use std::time::Duration;

use terminput::{Event, KeyCode, UnsupportedEvent};
use termwiz::caps::Capabilities;
use termwiz::terminal::buffered::BufferedTerminal;
use termwiz::terminal::{SystemTerminal, Terminal};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let caps = Capabilities::new_from_env()?;

    let terminal = SystemTerminal::new(caps)?;

    let mut terminal = BufferedTerminal::new(terminal)?;
    terminal.terminal().set_raw_mode()?;

    print_events(terminal);

    Ok(())
}

fn print_events(mut terminal: BufferedTerminal<SystemTerminal>) {
    loop {
        if let Ok(Some(event)) = terminal
            .terminal()
            .poll_input(Some(Duration::from_millis(10)))
        {
            let event: Result<Event, UnsupportedEvent> = event.try_into();
            println!("Event: {:?}\r", event);
            if let Ok(Event::Key(key_event)) = event {
                if key_event.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }
}
