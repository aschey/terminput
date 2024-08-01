//! Demonstrates how to block read events.
//!
//! cargo run --example event-read

use std::io;
use std::time::Duration;

use crossterm::cursor::position;
use crossterm::event::{
    poll, read, DisableBracketedPaste, DisableFocusChange, DisableMouseCapture,
    EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags,
    PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue};
use terminput::{Event, KeyCode, UnsupportedEvent};

const HELP: &str = r#"Blocking read()
 - Keyboard, mouse, focus and terminal resize events enabled
 - Hit "c" to print current cursor position
 - Use Esc to quit
"#;

fn print_events() -> io::Result<()> {
    loop {
        // Blocking read
        let event: Result<terminput::Event, UnsupportedEvent> = read()?.try_into();

        println!("Event: {:?}\r", event);
        if let Ok(event) = event {
            if let Event::Key(key_event) = event {
                if key_event.code == KeyCode::Char('c') {
                    println!("Cursor position: {:?}\r", position());
                }

                if key_event.code == KeyCode::Esc {
                    break;
                }
            }

            if let Event::Resize(x, y) = event {
                let (original_size, new_size) = flush_resize_events((x, y));
                println!("Resize from: {:?}, to: {:?}\r", original_size, new_size);
            }
        }
    }

    Ok(())
}

// Resize events can occur in batches.
// With a simple loop they can be flushed.
// This function will keep the first and last resize event.
fn flush_resize_events(first_resize: (u16, u16)) -> ((u16, u16), (u16, u16)) {
    let mut last_resize = first_resize;
    while let Ok(true) = poll(Duration::from_millis(50)) {
        if let Ok(Ok(Event::Resize(x, y))) = read().map(|r| r.try_into()) {
            last_resize = (x, y);
        }
    }

    (first_resize, last_resize)
}

fn main() -> io::Result<()> {
    println!("{}", HELP);

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    let supports_keyboard_enhancement = matches!(
        crossterm::terminal::supports_keyboard_enhancement(),
        Ok(true)
    );

    if supports_keyboard_enhancement {
        queue!(
            stdout,
            PushKeyboardEnhancementFlags(
                KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                    | KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS
                    | KeyboardEnhancementFlags::REPORT_EVENT_TYPES
            )
        )?;
    }

    execute!(
        stdout,
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture,
    )?;

    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    if supports_keyboard_enhancement {
        queue!(stdout, PopKeyboardEnhancementFlags)?;
    }

    execute!(
        stdout,
        DisableBracketedPaste,
        PopKeyboardEnhancementFlags,
        DisableFocusChange,
        DisableMouseCapture
    )?;

    disable_raw_mode()
}
