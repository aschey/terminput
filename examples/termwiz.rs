use std::time::Duration;

use terminput::{Encoding, Event, KeyCode, UnsupportedEvent, parse_event};
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
    let mut buf = [0; 16];
    loop {
        if let Ok(Some(event)) = terminal
            .terminal()
            .poll_input(Some(Duration::from_millis(10)))
        {
            let event: Result<Event, UnsupportedEvent> = event.try_into();

            if let Ok(event) = event {
                println!("Event:   {:?}\r", event);
                // Note: termwiz enables xterm's modifyOtherKeys setting which isn't supported by
                // the encoder
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
    }
}
