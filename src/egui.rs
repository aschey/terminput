use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, UnsupportedEvent,
};

impl TryFrom<egui::Event> for Event {
    type Error = UnsupportedEvent;
    fn try_from(value: egui::Event) -> Result<Self, Self::Error> {
        match value {
            egui::Event::Paste(text) => Ok(Event::Paste(text)),
            egui::Event::Text(text) => Ok(Event::Paste(text)),
            egui::Event::Key {
                key,
                physical_key: _,
                pressed,
                repeat,
                modifiers,
            } => Ok(Event::Key(KeyEvent {
                code: key.try_into()?,
                modifiers: modifiers.try_into()?,
                kind: if repeat {
                    KeyEventKind::Repeat
                } else if pressed {
                    KeyEventKind::Press
                } else {
                    KeyEventKind::Release
                },
                state: KeyEventState::empty(),
            })),
            egui::Event::PointerMoved(pos) => Ok(Event::Mouse(MouseEvent {
                kind: MouseEventKind::Moved,
                column: pos.x as u16,
                row: pos.y as u16,
                modifiers: KeyModifiers::empty(),
            })),

            egui::Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            } => Ok(Event::Mouse(MouseEvent {
                kind: if pressed {
                    MouseEventKind::Down(button.try_into()?)
                } else {
                    MouseEventKind::Up(button.try_into()?)
                },
                column: pos.x as u16,
                row: pos.y as u16,
                modifiers: modifiers.try_into()?,
            })),
            egui::Event::WindowFocused(true) => Ok(Event::FocusGained),
            egui::Event::WindowFocused(false) => Ok(Event::FocusLost),
            egui::Event::Copy
            | egui::Event::Cut
            | egui::Event::MouseMoved(_)
            | egui::Event::Ime(_)
            | egui::Event::PointerGone
            | egui::Event::Zoom(_)
            | egui::Event::MouseWheel { .. }
            | egui::Event::Touch { .. }
            | egui::Event::Screenshot { .. } => Err(UnsupportedEvent(format!("{value:?}"))),
        }
    }
}

impl TryFrom<egui::Key> for KeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::Key) -> Result<Self, Self::Error> {
        match value {
            egui::Key::ArrowDown => Ok(KeyCode::Down),
            egui::Key::ArrowLeft => Ok(KeyCode::Left),
            egui::Key::ArrowRight => Ok(KeyCode::Right),
            egui::Key::ArrowUp => Ok(KeyCode::Up),
            egui::Key::Escape => Ok(KeyCode::Esc),
            egui::Key::Tab => Ok(KeyCode::Tab),
            egui::Key::Backspace => Ok(KeyCode::Backspace),
            egui::Key::Enter => Ok(KeyCode::Enter),
            egui::Key::Space => Ok(KeyCode::Char(' ')),
            egui::Key::Insert => Ok(KeyCode::Insert),
            egui::Key::Delete => Ok(KeyCode::Delete),
            egui::Key::Home => Ok(KeyCode::Home),
            egui::Key::End => Ok(KeyCode::End),
            egui::Key::PageUp => Ok(KeyCode::PageUp),
            egui::Key::PageDown => Ok(KeyCode::PageDown),
            egui::Key::Copy => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Cut => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Paste => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Colon => Ok(KeyCode::Char(':')),
            egui::Key::Comma => Ok(KeyCode::Char(',')),
            egui::Key::Backslash => Ok(KeyCode::Char('\\')),
            egui::Key::Slash => Ok(KeyCode::Char('/')),
            egui::Key::Pipe => Ok(KeyCode::Char('|')),
            egui::Key::Questionmark => Ok(KeyCode::Char('?')),
            egui::Key::OpenBracket => Ok(KeyCode::Char('[')),
            egui::Key::CloseBracket => Ok(KeyCode::Char(']')),
            egui::Key::Backtick => Ok(KeyCode::Char('`')),
            egui::Key::Minus => Ok(KeyCode::Char('-')),
            egui::Key::Period => Ok(KeyCode::Char('.')),
            egui::Key::Plus => Ok(KeyCode::Char('+')),
            egui::Key::Equals => Ok(KeyCode::Char('=')),
            egui::Key::Semicolon => Ok(KeyCode::Char(';')),
            egui::Key::Quote => Ok(KeyCode::Char('\'')),
            egui::Key::Num0 => Ok(KeyCode::Char('0')),
            egui::Key::Num1 => Ok(KeyCode::Char('1')),
            egui::Key::Num2 => Ok(KeyCode::Char('2')),
            egui::Key::Num3 => Ok(KeyCode::Char('3')),
            egui::Key::Num4 => Ok(KeyCode::Char('4')),
            egui::Key::Num5 => Ok(KeyCode::Char('5')),
            egui::Key::Num6 => Ok(KeyCode::Char('6')),
            egui::Key::Num7 => Ok(KeyCode::Char('7')),
            egui::Key::Num8 => Ok(KeyCode::Char('8')),
            egui::Key::Num9 => Ok(KeyCode::Char('9')),
            egui::Key::A => Ok(KeyCode::Char('a')),
            egui::Key::B => Ok(KeyCode::Char('b')),
            egui::Key::C => Ok(KeyCode::Char('c')),
            egui::Key::D => Ok(KeyCode::Char('d')),
            egui::Key::E => Ok(KeyCode::Char('e')),
            egui::Key::F => Ok(KeyCode::Char('f')),
            egui::Key::G => Ok(KeyCode::Char('g')),
            egui::Key::H => Ok(KeyCode::Char('h')),
            egui::Key::I => Ok(KeyCode::Char('i')),
            egui::Key::J => Ok(KeyCode::Char('j')),
            egui::Key::K => Ok(KeyCode::Char('k')),
            egui::Key::L => Ok(KeyCode::Char('l')),
            egui::Key::M => Ok(KeyCode::Char('m')),
            egui::Key::N => Ok(KeyCode::Char('n')),
            egui::Key::O => Ok(KeyCode::Char('o')),
            egui::Key::P => Ok(KeyCode::Char('p')),
            egui::Key::Q => Ok(KeyCode::Char('q')),
            egui::Key::R => Ok(KeyCode::Char('r')),
            egui::Key::S => Ok(KeyCode::Char('s')),
            egui::Key::T => Ok(KeyCode::Char('t')),
            egui::Key::U => Ok(KeyCode::Char('u')),
            egui::Key::V => Ok(KeyCode::Char('v')),
            egui::Key::W => Ok(KeyCode::Char('w')),
            egui::Key::X => Ok(KeyCode::Char('x')),
            egui::Key::Y => Ok(KeyCode::Char('y')),
            egui::Key::Z => Ok(KeyCode::Char('z')),
            egui::Key::F1 => Ok(KeyCode::F(1)),
            egui::Key::F2 => Ok(KeyCode::F(2)),
            egui::Key::F3 => Ok(KeyCode::F(3)),
            egui::Key::F4 => Ok(KeyCode::F(4)),
            egui::Key::F5 => Ok(KeyCode::F(5)),
            egui::Key::F6 => Ok(KeyCode::F(6)),
            egui::Key::F7 => Ok(KeyCode::F(7)),
            egui::Key::F8 => Ok(KeyCode::F(8)),
            egui::Key::F9 => Ok(KeyCode::F(9)),
            egui::Key::F10 => Ok(KeyCode::F(10)),
            egui::Key::F11 => Ok(KeyCode::F(11)),
            egui::Key::F12 => Ok(KeyCode::F(12)),
            egui::Key::F13 => Ok(KeyCode::F(13)),
            egui::Key::F14 => Ok(KeyCode::F(14)),
            egui::Key::F15 => Ok(KeyCode::F(15)),
            egui::Key::F16 => Ok(KeyCode::F(16)),
            egui::Key::F17 => Ok(KeyCode::F(17)),
            egui::Key::F18 => Ok(KeyCode::F(18)),
            egui::Key::F19 => Ok(KeyCode::F(19)),
            egui::Key::F20 => Ok(KeyCode::F(20)),
            egui::Key::F21 => Ok(KeyCode::F(21)),
            egui::Key::F22 => Ok(KeyCode::F(22)),
            egui::Key::F23 => Ok(KeyCode::F(23)),
            egui::Key::F24 => Ok(KeyCode::F(24)),
            egui::Key::F25 => Ok(KeyCode::F(25)),
            egui::Key::F26 => Ok(KeyCode::F(26)),
            egui::Key::F27 => Ok(KeyCode::F(27)),
            egui::Key::F28 => Ok(KeyCode::F(28)),
            egui::Key::F29 => Ok(KeyCode::F(29)),
            egui::Key::F30 => Ok(KeyCode::F(30)),
            egui::Key::F31 => Ok(KeyCode::F(31)),
            egui::Key::F32 => Ok(KeyCode::F(32)),
            egui::Key::F33 => Ok(KeyCode::F(33)),
            egui::Key::F34 => Ok(KeyCode::F(34)),
            egui::Key::F35 => Ok(KeyCode::F(35)),
        }
    }
}

impl TryFrom<egui::Modifiers> for KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::Modifiers) -> Result<Self, Self::Error> {
        let mut mapped = KeyModifiers::empty();
        if value.alt {
            mapped |= KeyModifiers::ALT;
        }
        if value.shift {
            mapped |= KeyModifiers::SHIFT;
        }
        if value.command | value.ctrl | value.mac_cmd {
            mapped |= KeyModifiers::CTRL;
        }
        Ok(mapped)
    }
}

impl TryFrom<egui::PointerButton> for MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::PointerButton) -> Result<Self, Self::Error> {
        Ok(match value {
            egui::PointerButton::Primary => MouseButton::Left,
            egui::PointerButton::Secondary => MouseButton::Right,
            egui::PointerButton::Middle => MouseButton::Middle,
            egui::PointerButton::Extra1 => MouseButton::Unknown,
            egui::PointerButton::Extra2 => MouseButton::Unknown,
        })
    }
}
