use std::io::{self, Write};
use std::time::Duration;

use termina::escape::csi::{self, Csi, KittyKeyboardFlags};
use termina::{PlatformTerminal, Terminal};
use terminput::{Encoding, Event, KeyCode, KittyFlags};
use terminput_termina::to_terminput;

macro_rules! decset {
    ($mode:ident) => {
        csi::Csi::Mode(csi::Mode::SetDecPrivateMode(csi::DecPrivateMode::Code(
            csi::DecPrivateModeCode::$mode,
        )))
    };
}
macro_rules! decreset {
    ($mode:ident) => {
        csi::Csi::Mode(csi::Mode::ResetDecPrivateMode(csi::DecPrivateMode::Code(
            csi::DecPrivateModeCode::$mode,
        )))
    };
}

fn print_events(terminal: &PlatformTerminal, encoding: Encoding) -> io::Result<()> {
    let mut buf = [0; 16];
    loop {
        let event: Result<Event, _> = to_terminput(terminal.read(|e| !e.is_escape())?);

        if let Ok(event) = event {
            println!("Event:   {event:?}\r");
            let written = event.encode(&mut buf, encoding);
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

    Ok(())
}

fn main() -> io::Result<()> {
    println!("Press escape to exit");

    let mut terminal = PlatformTerminal::new()?;
    terminal.enter_raw_mode()?;

    write!(
        terminal,
        "{}",
        // Kitty keyboard
        Csi::Keyboard(csi::Keyboard::QueryFlags),
    )?;
    terminal.flush()?;
    let supports_kitty_keyboard =
        if terminal.poll(termina::Event::is_escape, Some(Duration::from_millis(100)))? {
            matches!(
                terminal.read(termina::Event::is_escape)?,
                termina::Event::Csi(Csi::Keyboard(csi::Keyboard::ReportFlags(_)))
            )
        } else {
            false
        };

    write!(
        terminal,
        "{}{}{}{}{}{}{}{}",
        csi::Csi::Keyboard(csi::Keyboard::PushFlags(KittyKeyboardFlags::all())),
        decset!(FocusTracking),
        decset!(BracketedPaste),
        decset!(MouseTracking),
        decset!(ButtonEventMouse),
        decset!(AnyEventMouse),
        decset!(RXVTMouse),
        decset!(SGRMouse),
    )?;
    terminal.flush()?;

    print_events(
        &terminal,
        if supports_kitty_keyboard {
            Encoding::Kitty(KittyFlags::all())
        } else {
            Encoding::Xterm
        },
    )?;

    write!(
        terminal,
        "{}{}{}{}{}{}{}{}",
        csi::Csi::Keyboard(csi::Keyboard::PopFlags(1)),
        decreset!(FocusTracking),
        decreset!(BracketedPaste),
        decreset!(MouseTracking),
        decreset!(ButtonEventMouse),
        decreset!(AnyEventMouse),
        decreset!(RXVTMouse),
        decreset!(SGRMouse),
    )?;

    Ok(())
}
