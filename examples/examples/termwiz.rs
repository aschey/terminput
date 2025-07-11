use std::time::Duration;

use terminput::{Encoding, Event, KeyCode};
use terminput_termwiz::to_terminput;
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
            let event: Result<Event, _> = to_terminput(event);

            if let Ok(event) = event {
                println!("Event:   {event:?}\r");
                // Note: termwiz enables xterm's modifyOtherKeys setting which isn't supported by
                // the encoder
                let written = event.encode(&mut buf, Encoding::Xterm);
                if let Ok(written) = written {
                    println!("Encoded: {:?}\r", &buf[..written]);
                    if let Ok(Some(decoded)) = Event::parse_from(&buf[..written]) {
                        println!("Decoded: {decoded:?}\r");
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
