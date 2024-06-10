use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, UnsupportedEvent,
};

impl TryFrom<termion::event::Event> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::Event) -> Result<Self, Self::Error> {
        Ok(match value {
            termion::event::Event::Key(key_event) => Event::Key(key_event.try_into()?),
            termion::event::Event::Mouse(mouse_event) => Event::Mouse(mouse_event.try_into()?),
            termion::event::Event::Unsupported(_) => Err(UnsupportedEvent)?,
        })
    }
}

impl TryFrom<termion::event::Key> for KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::Key) -> Result<Self, Self::Error> {
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
                modifiers: KeyModifiers::CONTROL,
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
                modifiers: KeyModifiers::CONTROL,
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
                modifiers: KeyModifiers::CONTROL,
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
                modifiers: KeyModifiers::CONTROL,
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
                modifiers: KeyModifiers::CONTROL,
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
                modifiers: KeyModifiers::CONTROL,
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
                code: KeyCode::BackTab,
                modifiers: KeyModifiers::NONE,
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
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Ctrl('\t') => KeyEvent {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Ctrl(c) => KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Null => KeyEvent {
                code: KeyCode::Null,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Esc => KeyEvent {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            _ => Err(UnsupportedEvent)?,
        })
    }
}

impl TryFrom<termion::event::MouseEvent> for MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::MouseEvent) -> Result<Self, Self::Error> {
        Ok(match value {
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
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelDown,
                row,
                column,
            ) => MouseEvent {
                kind: MouseEventKind::ScrollDown,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelUp,
                row,
                column,
            ) => MouseEvent {
                kind: MouseEventKind::ScrollUp,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelLeft,
                row,
                column,
            ) => MouseEvent {
                kind: MouseEventKind::ScrollLeft,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelRight,
                row,
                column,
            ) => MouseEvent {
                kind: MouseEventKind::ScrollRight,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
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
        })
    }
}
