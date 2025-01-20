use std::io;

use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, read,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue};
use terminput::{Event, KeyCode, UnsupportedEvent};

fn print_events() -> io::Result<()> {
    loop {
        let event: Result<terminput::Event, UnsupportedEvent> = read()?.try_into();

        println!("Event: {:?}\r", event);
        if let Ok(event) = event {
            if event == Event::Key(KeyCode::Esc.into()) {
                break;
            }
        }
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

    queue!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
    )?;

    print_events()?;

    if supports_keyboard_enhancement {
        execute!(stdout, PopKeyboardEnhancementFlags)?;
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        DisableFocusChange,
        DisableMouseCapture
    )?;

    disable_raw_mode()
}
