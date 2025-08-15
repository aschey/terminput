#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, ScrollDirection, UnsupportedEvent,
};
#[cfg(feature = "termion_4")]
use termion_4 as termion;

/// Converts the termion [`Event`](termion::event::Event) to a terminput [`Event`].
pub fn to_terminput(value: termion::event::Event) -> Result<Event, UnsupportedEvent> {
    Ok(match value {
        termion::event::Event::Key(key_event) => Event::Key(to_terminput_key(key_event)?),
        termion::event::Event::Mouse(mouse_event) => Event::Mouse(to_terminput_mouse(mouse_event)),
        termion::event::Event::Unsupported(val) => Err(UnsupportedEvent(format!("{val:?}")))?,
    })
}

/// Converts the terminput [`Event`] to a termion [`Event`](termion::event::Event).
pub fn to_termion(value: Event) -> Result<termion::event::Event, UnsupportedEvent> {
    Ok(match value {
        Event::Key(key_event) => termion::event::Event::Key(key_to_termion(key_event)?),
        Event::Mouse(mouse_event) => termion::event::Event::Mouse(to_termion_mouse(mouse_event)?),
        Event::FocusGained | Event::FocusLost | Event::Paste(_) | Event::Resize { .. } => {
            Err(UnsupportedEvent(format!("{value:?}")))?
        }
    })
}

/// Converts the termion [`Key`](termion::event::Key) to a terminput [`KeyEvent`].
pub fn to_terminput_key(value: termion::event::Key) -> Result<KeyEvent, UnsupportedEvent> {
    Ok(match value {
        termion::event::Key::Backspace => KeyEvent {
            code: KeyCode::Backspace,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Left => KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::ShiftLeft => KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::AltLeft => KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlLeft => KeyEvent {
            code: KeyCode::Left,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Right => KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::ShiftRight => KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::AltRight => KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlRight => KeyEvent {
            code: KeyCode::Right,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Up => KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::ShiftUp => KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::AltUp => KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlUp => KeyEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Down => KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::ShiftDown => KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::AltDown => KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlDown => KeyEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Home => KeyEvent {
            code: KeyCode::Home,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlHome => KeyEvent {
            code: KeyCode::Home,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::End => KeyEvent {
            code: KeyCode::End,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::CtrlEnd => KeyEvent {
            code: KeyCode::End,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::PageUp => KeyEvent {
            code: KeyCode::PageUp,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::PageDown => KeyEvent {
            code: KeyCode::PageDown,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::BackTab => KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::SHIFT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Delete => KeyEvent {
            code: KeyCode::Delete,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Insert => KeyEvent {
            code: KeyCode::Insert,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::F(f) => KeyEvent {
            code: KeyCode::F(f),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Char('\n') => KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Char('\t') => KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Char(c) => KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Alt('\n') => KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Alt('\t') => KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Alt(c) => KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::ALT,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Ctrl('\n') => KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Ctrl('\t') => KeyEvent {
            code: KeyCode::Tab,
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Ctrl(c) => KeyEvent {
            code: KeyCode::Char(c),
            modifiers: KeyModifiers::CTRL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        termion::event::Key::Esc => KeyEvent {
            code: KeyCode::Esc,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        },
        _ => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

/// Converts the terminput [`KeyEvent`] to a termion [`Key`](termion::event::Key).
pub fn key_to_termion(value: KeyEvent) -> Result<termion::event::Key, UnsupportedEvent> {
    if value.kind != KeyEventKind::Press {
        return Err(UnsupportedEvent(format!("{value:?}")));
    }
    if value.modifiers.intersects(KeyModifiers::CTRL) {
        match value.code {
            KeyCode::Char(c) => return Ok(termion::event::Key::Ctrl(c)),
            KeyCode::Left => return Ok(termion::event::Key::CtrlLeft),
            KeyCode::Right => return Ok(termion::event::Key::CtrlRight),
            KeyCode::Up => return Ok(termion::event::Key::CtrlUp),
            KeyCode::Down => return Ok(termion::event::Key::CtrlDown),
            _ => {}
        }
    }
    if value.modifiers.intersects(KeyModifiers::ALT) {
        match value.code {
            KeyCode::Char(c) => return Ok(termion::event::Key::Alt(c)),
            KeyCode::Left => return Ok(termion::event::Key::AltLeft),
            KeyCode::Right => return Ok(termion::event::Key::AltRight),
            KeyCode::Up => return Ok(termion::event::Key::AltUp),
            KeyCode::Down => return Ok(termion::event::Key::AltDown),
            _ => {}
        }
    }
    if value.modifiers.intersects(KeyModifiers::SHIFT) {
        match value.code {
            KeyCode::Left => return Ok(termion::event::Key::ShiftLeft),
            KeyCode::Right => return Ok(termion::event::Key::ShiftRight),
            KeyCode::Up => return Ok(termion::event::Key::ShiftUp),
            KeyCode::Down => return Ok(termion::event::Key::ShiftDown),
            _ => {}
        }
    }
    Ok(match value.code {
        KeyCode::Backspace => termion::event::Key::Backspace,
        KeyCode::Enter => termion::event::Key::Char('\n'),
        KeyCode::Left => termion::event::Key::Left,
        KeyCode::Right => termion::event::Key::Right,
        KeyCode::Up => termion::event::Key::Up,
        KeyCode::Down => termion::event::Key::Down,
        KeyCode::Home => termion::event::Key::Home,
        KeyCode::End => termion::event::Key::End,
        KeyCode::PageUp => termion::event::Key::PageUp,
        KeyCode::PageDown => termion::event::Key::PageDown,
        KeyCode::Tab if value.modifiers.intersects(KeyModifiers::SHIFT) => {
            termion::event::Key::BackTab
        }
        KeyCode::Tab => termion::event::Key::Char('\t'),

        KeyCode::Delete => termion::event::Key::Delete,
        KeyCode::Insert => termion::event::Key::Insert,
        KeyCode::F(f) => termion::event::Key::F(f),
        KeyCode::Char(c) => termion::event::Key::Char(c),
        KeyCode::Esc => termion::event::Key::Esc,
        KeyCode::CapsLock
        | KeyCode::NumLock
        | KeyCode::ScrollLock
        | KeyCode::PrintScreen
        | KeyCode::Pause
        | KeyCode::Menu
        | KeyCode::KeypadBegin
        | KeyCode::Media(_)
        | KeyCode::Modifier(_, _) => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

/// Converts the termion [`MouseEvent`](termion::event::MouseEvent) to a terminput [`MouseEvent`].
pub fn to_terminput_mouse(value: termion::event::MouseEvent) -> MouseEvent {
    match value {
        termion::event::MouseEvent::Press(termion::event::MouseButton::Left, column, row) => {
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Left),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::Right, column, row) => {
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::Middle, column, row) => {
            MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Right),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::WheelDown, row, column) => {
            MouseEvent {
                kind: MouseEventKind::Scroll(ScrollDirection::Down),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::WheelUp, row, column) => {
            MouseEvent {
                kind: MouseEventKind::Scroll(ScrollDirection::Up),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::WheelLeft, row, column) => {
            MouseEvent {
                kind: MouseEventKind::Scroll(ScrollDirection::Left),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Press(termion::event::MouseButton::WheelRight, row, column) => {
            MouseEvent {
                kind: MouseEventKind::Scroll(ScrollDirection::Right),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            }
        }
        termion::event::MouseEvent::Release(column, row) => MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Unknown),
            row: row - 1,
            column: column - 1,
            modifiers: KeyModifiers::NONE,
        },
        termion::event::MouseEvent::Hold(column, row) => MouseEvent {
            kind: MouseEventKind::Drag(MouseButton::Unknown),
            row: row - 1,
            column: column - 1,
            modifiers: KeyModifiers::NONE,
        },
    }
}

/// Converts the terminput [`MouseEvent`] to a termion [`MouseEvent`](termion::event::MouseEvent).
pub fn to_termion_mouse(value: MouseEvent) -> Result<termion::event::MouseEvent, UnsupportedEvent> {
    let column = value.column + 1;
    let row = value.row + 1;
    Ok(match value.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::Left, column, row)
        }
        MouseEventKind::Down(MouseButton::Right) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::Right, column, row)
        }
        MouseEventKind::Down(MouseButton::Middle) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::Middle, column, row)
        }
        val @ MouseEventKind::Down(MouseButton::Unknown) => {
            Err(UnsupportedEvent(format!("{val:?}")))?
        }
        MouseEventKind::Up(_) => termion::event::MouseEvent::Release(column, row),
        MouseEventKind::Drag(_) => termion::event::MouseEvent::Hold(column, row),
        val @ MouseEventKind::Moved => Err(UnsupportedEvent(format!("{val:?}")))?,
        MouseEventKind::Scroll(ScrollDirection::Down) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::WheelDown, column, row)
        }
        MouseEventKind::Scroll(ScrollDirection::Up) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::WheelUp, column, row)
        }
        MouseEventKind::Scroll(ScrollDirection::Left) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::WheelLeft, column, row)
        }
        MouseEventKind::Scroll(ScrollDirection::Right) => {
            termion::event::MouseEvent::Press(termion::event::MouseButton::WheelRight, column, row)
        }
    })
}
