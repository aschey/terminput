use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, UnsupportedEvent,
};

impl TryFrom<termion::event::Event> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::Event) -> Result<Self, Self::Error> {
        Ok(match value {
            termion::event::Event::Key(key_event) => Self::Key(key_event.try_into()?),
            termion::event::Event::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            termion::event::Event::Unsupported(val) => Err(UnsupportedEvent(format!("{val:?}")))?,
        })
    }
}

impl TryFrom<Event> for termion::event::Event {
    type Error = UnsupportedEvent;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::Key(key_event) => Self::Key(key_event.try_into()?),
            Event::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            Event::FocusGained | Event::FocusLost | Event::Paste(_) | Event::Resize { .. } => {
                Err(UnsupportedEvent(format!("{value:?}")))?
            }
        })
    }
}

impl TryFrom<termion::event::Key> for KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::Key) -> Result<Self, Self::Error> {
        Ok(match value {
            termion::event::Key::Backspace => Self {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Left => Self {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::ShiftLeft => Self {
                code: KeyCode::Left,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::AltLeft => Self {
                code: KeyCode::Left,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlLeft => Self {
                code: KeyCode::Left,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Right => Self {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::ShiftRight => Self {
                code: KeyCode::Right,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::AltRight => Self {
                code: KeyCode::Right,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlRight => Self {
                code: KeyCode::Right,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Up => Self {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::ShiftUp => Self {
                code: KeyCode::Up,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::AltUp => Self {
                code: KeyCode::Up,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlUp => Self {
                code: KeyCode::Up,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Down => Self {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::ShiftDown => Self {
                code: KeyCode::Down,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::AltDown => Self {
                code: KeyCode::Down,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlDown => Self {
                code: KeyCode::Down,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Home => Self {
                code: KeyCode::Home,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlHome => Self {
                code: KeyCode::Home,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::End => Self {
                code: KeyCode::End,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::CtrlEnd => Self {
                code: KeyCode::End,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::PageUp => Self {
                code: KeyCode::PageUp,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::PageDown => Self {
                code: KeyCode::PageDown,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::BackTab => Self {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::SHIFT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Delete => Self {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Insert => Self {
                code: KeyCode::Insert,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::F(f) => Self {
                code: KeyCode::F(f),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Char('\n') => Self {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Char('\t') => Self {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Char(c) => Self {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Alt('\n') => Self {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Alt('\t') => Self {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Alt(c) => Self {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::ALT,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Ctrl('\n') => Self {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Ctrl('\t') => Self {
                code: KeyCode::Tab,
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Ctrl(c) => Self {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CTRL,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            termion::event::Key::Esc => Self {
                code: KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::empty(),
            },
            _ => Err(UnsupportedEvent(format!("{value:?}")))?,
        })
    }
}

impl TryFrom<KeyEvent> for termion::event::Key {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        if value.kind != KeyEventKind::Press {
            return Err(UnsupportedEvent(format!("{value:?}")));
        }
        if value.modifiers.intersects(KeyModifiers::CTRL) {
            match value.code {
                KeyCode::Char(c) => return Ok(Self::Ctrl(c)),
                KeyCode::Left => return Ok(Self::CtrlLeft),
                KeyCode::Right => return Ok(Self::CtrlRight),
                KeyCode::Up => return Ok(Self::CtrlUp),
                KeyCode::Down => return Ok(Self::CtrlDown),
                _ => {}
            }
        }
        if value.modifiers.intersects(KeyModifiers::ALT) {
            match value.code {
                KeyCode::Char(c) => return Ok(Self::Alt(c)),
                KeyCode::Left => return Ok(Self::AltLeft),
                KeyCode::Right => return Ok(Self::AltRight),
                KeyCode::Up => return Ok(Self::AltUp),
                KeyCode::Down => return Ok(Self::AltDown),
                _ => {}
            }
        }
        if value.modifiers.intersects(KeyModifiers::SHIFT) {
            match value.code {
                KeyCode::Left => return Ok(Self::ShiftLeft),
                KeyCode::Right => return Ok(Self::ShiftRight),
                KeyCode::Up => return Ok(Self::ShiftUp),
                KeyCode::Down => return Ok(Self::ShiftDown),
                _ => {}
            }
        }
        Ok(match value.code {
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Enter => Self::Char('\n'),
            KeyCode::Left => Self::Left,
            KeyCode::Right => Self::Right,
            KeyCode::Up => Self::Up,
            KeyCode::Down => Self::Down,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::Tab if value.modifiers.intersects(KeyModifiers::SHIFT) => Self::BackTab,
            KeyCode::Tab => Self::Char('\t'),

            KeyCode::Delete => Self::Delete,
            KeyCode::Insert => Self::Insert,
            KeyCode::F(f) => Self::F(f),
            KeyCode::Char(c) => Self::Char(c),
            KeyCode::Esc => Self::Esc,
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
}

impl TryFrom<termion::event::MouseEvent> for MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termion::event::MouseEvent) -> Result<Self, Self::Error> {
        Ok(match value {
            termion::event::MouseEvent::Press(termion::event::MouseButton::Left, column, row) => {
                Self {
                    kind: MouseEventKind::Down(MouseButton::Left),
                    row: row - 1,
                    column: column - 1,
                    modifiers: KeyModifiers::NONE,
                }
            }
            termion::event::MouseEvent::Press(termion::event::MouseButton::Right, column, row) => {
                Self {
                    kind: MouseEventKind::Down(MouseButton::Right),
                    row: row - 1,
                    column: column - 1,
                    modifiers: KeyModifiers::NONE,
                }
            }
            termion::event::MouseEvent::Press(termion::event::MouseButton::Middle, column, row) => {
                Self {
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
            ) => Self {
                kind: MouseEventKind::ScrollDown,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelUp,
                row,
                column,
            ) => Self {
                kind: MouseEventKind::ScrollUp,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelLeft,
                row,
                column,
            ) => Self {
                kind: MouseEventKind::ScrollLeft,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Press(
                termion::event::MouseButton::WheelRight,
                row,
                column,
            ) => Self {
                kind: MouseEventKind::ScrollRight,
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Release(column, row) => Self {
                kind: MouseEventKind::Up(MouseButton::Unknown),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
            termion::event::MouseEvent::Hold(column, row) => Self {
                kind: MouseEventKind::Drag(MouseButton::Unknown),
                row: row - 1,
                column: column - 1,
                modifiers: KeyModifiers::NONE,
            },
        })
    }
}

impl TryFrom<MouseEvent> for termion::event::MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseEvent) -> Result<Self, Self::Error> {
        let column = value.column + 1;
        let row = value.row + 1;
        Ok(match value.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                Self::Press(termion::event::MouseButton::Left, column, row)
            }
            MouseEventKind::Down(MouseButton::Right) => {
                Self::Press(termion::event::MouseButton::Right, column, row)
            }
            MouseEventKind::Down(MouseButton::Middle) => {
                Self::Press(termion::event::MouseButton::Middle, column, row)
            }
            val @ MouseEventKind::Down(MouseButton::Unknown) => {
                Err(UnsupportedEvent(format!("{val:?}")))?
            }
            MouseEventKind::Up(_) => Self::Release(column, row),
            MouseEventKind::Drag(_) => Self::Hold(column, row),
            val @ MouseEventKind::Moved => Err(UnsupportedEvent(format!("{val:?}")))?,
            MouseEventKind::ScrollDown => {
                Self::Press(termion::event::MouseButton::WheelDown, column, row)
            }
            MouseEventKind::ScrollUp => {
                Self::Press(termion::event::MouseButton::WheelUp, column, row)
            }
            MouseEventKind::ScrollLeft => {
                Self::Press(termion::event::MouseButton::WheelLeft, column, row)
            }
            MouseEventKind::ScrollRight => {
                Self::Press(termion::event::MouseButton::WheelRight, column, row)
            }
        })
    }
}
