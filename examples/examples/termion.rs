#[cfg(not(windows))]
mod example {

    use std::io;

    use terminput::{Encoding, Event, KeyCode};
    use terminput_termion::to_terminput;
    use termion::input::TermRead;

    pub fn print_events() -> io::Result<()> {
        let stdin = io::stdin();
        let mut buf = [0; 16];

        for event in stdin.events() {
            let event: Result<Event, _> = to_terminput(event?);

            if let Ok(event) = event {
                println!("Event:   {:?}\r", event);
                let written = event.encode(&mut buf, Encoding::Xterm);
                if let Ok(written) = written {
                    println!("Encoded: {:?}\r", &buf[..written]);
                    if let Ok(Some(decoded)) = Event::parse_from(&buf[..written]) {
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
}

#[cfg(not(windows))]
fn main() -> std::io::Result<()> {
    use termion::input::MouseTerminal;
    use termion::raw::IntoRawMode;

    let stdout = std::io::stdout().into_raw_mode()?;
    let _stdout = MouseTerminal::from(stdout);
    example::print_events()?;

    Ok(())
}

#[cfg(windows)]
fn main() {
    panic!("termion is not supported on windows");
}
