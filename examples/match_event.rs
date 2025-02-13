use std::io;

use crossterm::event::{
    DisableBracketedPaste, DisableFocusChange, DisableMouseCapture, EnableBracketedPaste,
    EnableFocusChange, EnableMouseCapture, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
    PushKeyboardEnhancementFlags, read,
};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::{execute, queue};
use terminput::KeyCode::*;
use terminput::{ALT, CAPS_LOCK, CTRL, Event, KeyModifiers, MouseButton, Repeats, key, modifiers};

fn print_events() -> io::Result<()> {
    const CTRL_ALT: KeyModifiers = modifiers!(CTRL, ALT);

    loop {
        let event: Result<Event, _> = read()?.try_into();

        let Ok(event) = event else {
            continue;
        };

        if let Some(key_event) = event.as_key_press(Repeats::Include) {
            match key_event {
                key!(CTRL | ALT, Char('c')) => {
                    println!("'ctrl+c' or 'alt+c' pressed\r");
                }
                key!(CTRL_ALT, Char('c')) => {
                    println!("'ctrl+alt+c' pressed\r");
                }
                // Note: this branch requires kitty keyboard protocol support
                key!(CAPS_LOCK, KeyModifiers::NONE, Char('c')) => {
                    println!("'c' pressed with caps lock\r");
                }
                key!(Char('c')) => {
                    println!("'c' pressed\r");
                }
                key!(Esc) => {
                    break;
                }
                e => {
                    println!("{e:?}\r");
                }
            }
        } else if let Some((mouse_event, MouseButton::Left)) = event.as_mouse_down() {
            println!("left mouse down: {mouse_event:?}\r");
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    println!("Press esc to exit");

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

    print_events()?;

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
