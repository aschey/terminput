use std::io;

use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, read,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue};
use terminput::{Encoding, Event, KeyCode, KittyFlags};

fn print_events(encoding: Encoding) -> io::Result<()> {
    let mut buf = [0; 16];
    loop {
        let event: Result<Event, _> = read()?.try_into();

        if let Ok(event) = event {
            println!("Event:   {:?}\r", event);
            let written = event.encode(&mut buf, encoding);
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

fn main() -> io::Result<()> {
    println!("Press escape to exit");

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );

    if supports_keyboard_enhancement {
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::all())
        )?;
    }

    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
    )?;

    print_events(if supports_keyboard_enhancement {
        Encoding::Kitty(KittyFlags::all())
    } else {
        Encoding::Xterm
    })?;

    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags)?;
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )?;

    disable_raw_mode()
}
