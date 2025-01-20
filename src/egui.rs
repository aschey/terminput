use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, UnsupportedEvent,
};

impl TryFrom<egui::Event> for Event {
    type Error = UnsupportedEvent;
    fn try_from(value: egui::Event) -> Result<Self, Self::Error> {
        match value {
            egui::Event::Paste(text) => Ok(Self::Paste(text)),
            egui::Event::Text(text) => Ok(Self::Paste(text)),
            egui::Event::Key {
                key,
                physical_key: _,
                pressed,
                repeat,
                modifiers,
            } => Ok(Self::Key(KeyEvent {
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
            egui::Event::PointerMoved(pos) => Ok(Self::Mouse(MouseEvent {
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
            } => Ok(Self::Mouse(MouseEvent {
                kind: if pressed {
                    MouseEventKind::Down(button.try_into()?)
                } else {
                    MouseEventKind::Up(button.try_into()?)
                },
                column: pos.x as u16,
                row: pos.y as u16,
                modifiers: modifiers.try_into()?,
            })),
            egui::Event::WindowFocused(true) => Ok(Self::FocusGained),
            egui::Event::WindowFocused(false) => Ok(Self::FocusLost),
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
            egui::Key::ArrowDown => Ok(Self::Down),
            egui::Key::ArrowLeft => Ok(Self::Left),
            egui::Key::ArrowRight => Ok(Self::Right),
            egui::Key::ArrowUp => Ok(Self::Up),
            egui::Key::Escape => Ok(Self::Esc),
            egui::Key::Tab => Ok(Self::Tab),
            egui::Key::Backspace => Ok(Self::Backspace),
            egui::Key::Enter => Ok(Self::Enter),
            egui::Key::Space => Ok(Self::Char(' ')),
            egui::Key::Insert => Ok(Self::Insert),
            egui::Key::Delete => Ok(Self::Delete),
            egui::Key::Home => Ok(Self::Home),
            egui::Key::End => Ok(Self::End),
            egui::Key::PageUp => Ok(Self::PageUp),
            egui::Key::PageDown => Ok(Self::PageDown),
            egui::Key::Copy => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Cut => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Paste => Err(UnsupportedEvent(format!("{value:?}"))),
            egui::Key::Colon => Ok(Self::Char(':')),
            egui::Key::Comma => Ok(Self::Char(',')),
            egui::Key::Backslash => Ok(Self::Char('\\')),
            egui::Key::Slash => Ok(Self::Char('/')),
            egui::Key::Pipe => Ok(Self::Char('|')),
            egui::Key::Questionmark => Ok(Self::Char('?')),
            egui::Key::OpenBracket => Ok(Self::Char('[')),
            egui::Key::CloseBracket => Ok(Self::Char(']')),
            egui::Key::Backtick => Ok(Self::Char('`')),
            egui::Key::Minus => Ok(Self::Char('-')),
            egui::Key::Period => Ok(Self::Char('.')),
            egui::Key::Plus => Ok(Self::Char('+')),
            egui::Key::Equals => Ok(Self::Char('=')),
            egui::Key::Semicolon => Ok(Self::Char(';')),
            egui::Key::Quote => Ok(Self::Char('\'')),
            egui::Key::Num0 => Ok(Self::Char('0')),
            egui::Key::Num1 => Ok(Self::Char('1')),
            egui::Key::Num2 => Ok(Self::Char('2')),
            egui::Key::Num3 => Ok(Self::Char('3')),
            egui::Key::Num4 => Ok(Self::Char('4')),
            egui::Key::Num5 => Ok(Self::Char('5')),
            egui::Key::Num6 => Ok(Self::Char('6')),
            egui::Key::Num7 => Ok(Self::Char('7')),
            egui::Key::Num8 => Ok(Self::Char('8')),
            egui::Key::Num9 => Ok(Self::Char('9')),
            egui::Key::A => Ok(Self::Char('a')),
            egui::Key::B => Ok(Self::Char('b')),
            egui::Key::C => Ok(Self::Char('c')),
            egui::Key::D => Ok(Self::Char('d')),
            egui::Key::E => Ok(Self::Char('e')),
            egui::Key::F => Ok(Self::Char('f')),
            egui::Key::G => Ok(Self::Char('g')),
            egui::Key::H => Ok(Self::Char('h')),
            egui::Key::I => Ok(Self::Char('i')),
            egui::Key::J => Ok(Self::Char('j')),
            egui::Key::K => Ok(Self::Char('k')),
            egui::Key::L => Ok(Self::Char('l')),
            egui::Key::M => Ok(Self::Char('m')),
            egui::Key::N => Ok(Self::Char('n')),
            egui::Key::O => Ok(Self::Char('o')),
            egui::Key::P => Ok(Self::Char('p')),
            egui::Key::Q => Ok(Self::Char('q')),
            egui::Key::R => Ok(Self::Char('r')),
            egui::Key::S => Ok(Self::Char('s')),
            egui::Key::T => Ok(Self::Char('t')),
            egui::Key::U => Ok(Self::Char('u')),
            egui::Key::V => Ok(Self::Char('v')),
            egui::Key::W => Ok(Self::Char('w')),
            egui::Key::X => Ok(Self::Char('x')),
            egui::Key::Y => Ok(Self::Char('y')),
            egui::Key::Z => Ok(Self::Char('z')),
            egui::Key::F1 => Ok(Self::F(1)),
            egui::Key::F2 => Ok(Self::F(2)),
            egui::Key::F3 => Ok(Self::F(3)),
            egui::Key::F4 => Ok(Self::F(4)),
            egui::Key::F5 => Ok(Self::F(5)),
            egui::Key::F6 => Ok(Self::F(6)),
            egui::Key::F7 => Ok(Self::F(7)),
            egui::Key::F8 => Ok(Self::F(8)),
            egui::Key::F9 => Ok(Self::F(9)),
            egui::Key::F10 => Ok(Self::F(10)),
            egui::Key::F11 => Ok(Self::F(11)),
            egui::Key::F12 => Ok(Self::F(12)),
            egui::Key::F13 => Ok(Self::F(13)),
            egui::Key::F14 => Ok(Self::F(14)),
            egui::Key::F15 => Ok(Self::F(15)),
            egui::Key::F16 => Ok(Self::F(16)),
            egui::Key::F17 => Ok(Self::F(17)),
            egui::Key::F18 => Ok(Self::F(18)),
            egui::Key::F19 => Ok(Self::F(19)),
            egui::Key::F20 => Ok(Self::F(20)),
            egui::Key::F21 => Ok(Self::F(21)),
            egui::Key::F22 => Ok(Self::F(22)),
            egui::Key::F23 => Ok(Self::F(23)),
            egui::Key::F24 => Ok(Self::F(24)),
            egui::Key::F25 => Ok(Self::F(25)),
            egui::Key::F26 => Ok(Self::F(26)),
            egui::Key::F27 => Ok(Self::F(27)),
            egui::Key::F28 => Ok(Self::F(28)),
            egui::Key::F29 => Ok(Self::F(29)),
            egui::Key::F30 => Ok(Self::F(30)),
            egui::Key::F31 => Ok(Self::F(31)),
            egui::Key::F32 => Ok(Self::F(32)),
            egui::Key::F33 => Ok(Self::F(33)),
            egui::Key::F34 => Ok(Self::F(34)),
            egui::Key::F35 => Ok(Self::F(35)),
        }
    }
}

impl TryFrom<egui::Modifiers> for KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::Modifiers) -> Result<Self, Self::Error> {
        let mut mapped = Self::empty();
        if value.alt {
            mapped |= Self::ALT;
        }
        if value.shift {
            mapped |= Self::SHIFT;
        }
        if value.command | value.ctrl | value.mac_cmd {
            mapped |= Self::CTRL;
        }
        Ok(mapped)
    }
}

impl TryFrom<egui::PointerButton> for MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::PointerButton) -> Result<Self, Self::Error> {
        Ok(match value {
            egui::PointerButton::Primary => Self::Left,
            egui::PointerButton::Secondary => Self::Right,
            egui::PointerButton::Middle => Self::Middle,
            egui::PointerButton::Extra1 => Self::Unknown,
            egui::PointerButton::Extra2 => Self::Unknown,
        })
    }
}
