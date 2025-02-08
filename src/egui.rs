use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, ScrollDirection, UnsupportedEvent,
};

impl TryFrom<egui::Event> for Event {
    type Error = UnsupportedEvent;
    fn try_from(value: egui::Event) -> Result<Self, Self::Error> {
        match value {
            egui::Event::Paste(text) => Ok(Self::Paste(text)),
            egui::Event::Text(_) => Err(UnsupportedEvent(format!("{value:?}"))),
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
            egui::Event::MouseWheel {
                unit: _,
                delta,
                modifiers,
            } => Ok(Self::Mouse(MouseEvent {
                kind: MouseEventKind::Scroll(if delta.y < 0.0 {
                    ScrollDirection::Down
                } else if delta.y > 0.0 {
                    ScrollDirection::Up
                } else if delta.x < 0.0 {
                    ScrollDirection::Left
                } else {
                    ScrollDirection::Right
                }),
                column: 0,
                row: 0,
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
            | egui::Event::Touch { .. }
            | egui::Event::Screenshot { .. } => Err(UnsupportedEvent(format!("{value:?}"))),
        }
    }
}

impl TryFrom<Event> for egui::Event {
    type Error = UnsupportedEvent;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::Key(key_event) => Self::Key {
                key: key_event.code.try_into()?,
                physical_key: None,
                pressed: key_event.kind != KeyEventKind::Release,
                repeat: key_event.kind == KeyEventKind::Repeat,
                modifiers: key_event.modifiers.try_into()?,
            },
            Event::Mouse(mouse_event) => match mouse_event.kind {
                MouseEventKind::Down(mouse_button) | MouseEventKind::Drag(mouse_button) => {
                    Self::PointerButton {
                        pos: egui::Pos2 {
                            x: mouse_event.column as f32,
                            y: mouse_event.row as f32,
                        },
                        button: mouse_button.try_into()?,
                        pressed: true,
                        modifiers: mouse_event.modifiers.try_into()?,
                    }
                }
                MouseEventKind::Up(mouse_button) => Self::PointerButton {
                    pos: egui::Pos2 {
                        x: mouse_event.column as f32,
                        y: mouse_event.row as f32,
                    },
                    button: mouse_button.try_into()?,
                    pressed: false,
                    modifiers: mouse_event.modifiers.try_into()?,
                },
                MouseEventKind::Moved => Self::PointerMoved(egui::Pos2 {
                    x: mouse_event.column as f32,
                    y: mouse_event.row as f32,
                }),
                MouseEventKind::Scroll(scroll_direction) => Self::MouseWheel {
                    unit: egui::MouseWheelUnit::Line,
                    delta: egui::Vec2 {
                        x: scroll_direction.delta().x as f32,
                        y: scroll_direction.delta().y as f32,
                    },
                    modifiers: mouse_event.modifiers.try_into()?,
                },
            },
            Event::Paste(text) => Self::Paste(text),
            Event::FocusGained => Self::WindowFocused(true),
            Event::FocusLost => Self::WindowFocused(false),
            Event::Resize { .. } => Err(UnsupportedEvent(format!("{value:?}")))?,
        })
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
            egui::Key::Colon => Ok(Self::Char(':')),
            egui::Key::Comma => Ok(Self::Char(',')),
            egui::Key::Backslash => Ok(Self::Char('\\')),
            egui::Key::Slash => Ok(Self::Char('/')),
            egui::Key::Pipe => Ok(Self::Char('|')),
            egui::Key::Questionmark => Ok(Self::Char('?')),
            egui::Key::Exclamationmark => Ok(Self::Char('!')),
            egui::Key::OpenBracket => Ok(Self::Char('[')),
            egui::Key::CloseBracket => Ok(Self::Char(']')),
            egui::Key::OpenCurlyBracket => Ok(Self::Char('{')),
            egui::Key::CloseCurlyBracket => Ok(Self::Char('}')),
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
            egui::Key::Copy | egui::Key::Cut | egui::Key::Paste => {
                Err(UnsupportedEvent(format!("{value:?}")))
            }
        }
    }
}

impl TryFrom<KeyCode> for egui::Key {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyCode::Backspace => Self::Backspace,
            KeyCode::Enter => Self::Enter,
            KeyCode::Left => Self::ArrowLeft,
            KeyCode::Right => Self::ArrowRight,
            KeyCode::Up => Self::ArrowUp,
            KeyCode::Down => Self::ArrowDown,
            KeyCode::Home => Self::Home,
            KeyCode::End => Self::End,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::Tab => Self::Tab,
            KeyCode::Delete => Self::Delete,
            KeyCode::Insert => Self::Insert,
            KeyCode::F(1) => Self::F1,
            KeyCode::F(2) => Self::F2,
            KeyCode::F(3) => Self::F3,
            KeyCode::F(4) => Self::F4,
            KeyCode::F(5) => Self::F5,
            KeyCode::F(6) => Self::F6,
            KeyCode::F(7) => Self::F7,
            KeyCode::F(8) => Self::F8,
            KeyCode::F(9) => Self::F9,
            KeyCode::F(10) => Self::F10,
            KeyCode::F(11) => Self::F11,
            KeyCode::F(12) => Self::F12,
            KeyCode::F(13) => Self::F13,
            KeyCode::F(14) => Self::F14,
            KeyCode::F(15) => Self::F15,
            KeyCode::F(16) => Self::F16,
            KeyCode::F(17) => Self::F17,
            KeyCode::F(18) => Self::F18,
            KeyCode::F(19) => Self::F19,
            KeyCode::F(20) => Self::F20,
            KeyCode::F(21) => Self::F21,
            KeyCode::F(22) => Self::F22,
            KeyCode::F(23) => Self::F23,
            KeyCode::F(24) => Self::F24,
            KeyCode::F(25) => Self::F25,
            KeyCode::F(26) => Self::F26,
            KeyCode::F(27) => Self::F27,
            KeyCode::F(28) => Self::F28,
            KeyCode::F(29) => Self::F29,
            KeyCode::F(30) => Self::F30,
            KeyCode::F(31) => Self::F31,
            KeyCode::F(32) => Self::F32,
            KeyCode::F(33) => Self::F33,
            KeyCode::F(34) => Self::F34,
            KeyCode::F(35) => Self::F35,
            KeyCode::F(_) => Err(UnsupportedEvent(format!("{value:?}")))?,
            KeyCode::Char('a' | 'A') => Self::A,
            KeyCode::Char('b' | 'B') => Self::B,
            KeyCode::Char('c' | 'C') => Self::C,
            KeyCode::Char('d' | 'D') => Self::D,
            KeyCode::Char('e' | 'E') => Self::E,
            KeyCode::Char('f' | 'F') => Self::F,
            KeyCode::Char('g' | 'G') => Self::G,
            KeyCode::Char('h' | 'H') => Self::H,
            KeyCode::Char('i' | 'I') => Self::I,
            KeyCode::Char('j' | 'J') => Self::J,
            KeyCode::Char('k' | 'K') => Self::K,
            KeyCode::Char('l' | 'L') => Self::L,
            KeyCode::Char('m' | 'M') => Self::M,
            KeyCode::Char('n' | 'N') => Self::N,
            KeyCode::Char('o' | 'O') => Self::O,
            KeyCode::Char('p' | 'P') => Self::P,
            KeyCode::Char('q' | 'Q') => Self::Q,
            KeyCode::Char('r' | 'R') => Self::R,
            KeyCode::Char('s' | 'S') => Self::S,
            KeyCode::Char('t' | 'T') => Self::T,
            KeyCode::Char('u' | 'U') => Self::U,
            KeyCode::Char('v' | 'V') => Self::V,
            KeyCode::Char('w' | 'W') => Self::W,
            KeyCode::Char('x' | 'X') => Self::X,
            KeyCode::Char('y' | 'Y') => Self::Y,
            KeyCode::Char('z' | 'Z') => Self::Z,
            KeyCode::Char('0') => Self::Num0,
            KeyCode::Char('1') => Self::Num1,
            KeyCode::Char('2') => Self::Num2,
            KeyCode::Char('3') => Self::Num3,
            KeyCode::Char('4') => Self::Num4,
            KeyCode::Char('5') => Self::Num5,
            KeyCode::Char('6') => Self::Num6,
            KeyCode::Char('7') => Self::Num7,
            KeyCode::Char('8') => Self::Num8,
            KeyCode::Char('9') => Self::Num9,
            KeyCode::Char(' ') => Self::Space,
            KeyCode::Char(':') => Self::Colon,
            KeyCode::Char(',') => Self::Comma,
            KeyCode::Char('\\') => Self::Backslash,
            KeyCode::Char('/') => Self::Slash,
            KeyCode::Char('|') => Self::Pipe,
            KeyCode::Char('?') => Self::Questionmark,
            KeyCode::Char('[') => Self::OpenBracket,
            KeyCode::Char(']') => Self::CloseBracket,
            KeyCode::Char('`') => Self::Backtick,
            KeyCode::Char('-') => Self::Minus,
            KeyCode::Char('.') => Self::Period,
            KeyCode::Char('+') => Self::Plus,
            KeyCode::Char('=') => Self::Equals,
            KeyCode::Char(';') => Self::Semicolon,
            KeyCode::Char('\'') => Self::Quote,
            KeyCode::Char(_) => Err(UnsupportedEvent(format!("{value:?}")))?,
            KeyCode::Esc => Self::Escape,
            KeyCode::CapsLock
            | KeyCode::ScrollLock
            | KeyCode::NumLock
            | KeyCode::PrintScreen
            | KeyCode::Pause
            | KeyCode::Menu
            | KeyCode::KeypadBegin
            | KeyCode::Media(_)
            | KeyCode::Modifier(_, _) => Err(UnsupportedEvent(format!("{value:?}")))?,
        })
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

impl TryFrom<KeyModifiers> for egui::Modifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyModifiers) -> Result<Self, Self::Error> {
        Ok(Self {
            alt: value.intersects(KeyModifiers::ALT),
            ctrl: value.intersects(KeyModifiers::CTRL),
            shift: value.intersects(KeyModifiers::SHIFT),
            mac_cmd: false,
            command: false,
        })
    }
}

impl TryFrom<egui::PointerButton> for MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: egui::PointerButton) -> Result<Self, Self::Error> {
        Ok(match value {
            egui::PointerButton::Primary => Self::Left,
            egui::PointerButton::Secondary => Self::Right,
            egui::PointerButton::Middle => Self::Middle,
            egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => Self::Unknown,
        })
    }
}

impl TryFrom<MouseButton> for egui::PointerButton {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseButton) -> Result<Self, Self::Error> {
        Ok(match value {
            MouseButton::Left | MouseButton::Unknown => Self::Primary,
            MouseButton::Right => Self::Secondary,
            MouseButton::Middle => Self::Middle,
        })
    }
}
