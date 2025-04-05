#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MouseButton, MouseEvent,
    MouseEventKind, ScrollDirection, UnsupportedEvent,
};

/// Converts the egui [event](egui::Event) to a terminput [event](Event).
pub fn to_terminput(value: egui::Event) -> Result<Event, UnsupportedEvent> {
    match value {
        egui::Event::Paste(text) => Ok(Event::Paste(text)),
        egui::Event::Text(_) => Err(UnsupportedEvent(format!("{value:?}"))),
        egui::Event::Key {
            key,
            physical_key: _,
            pressed,
            repeat,
            modifiers,
        } => Ok(Event::Key(KeyEvent {
            code: key_code_to_terminput(key)?,
            modifiers: key_modifiers_to_terminput(modifiers),
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
                MouseEventKind::Down(mouse_button_to_terminput(button))
            } else {
                MouseEventKind::Up(mouse_button_to_terminput(button))
            },
            column: pos.x as u16,
            row: pos.y as u16,
            modifiers: key_modifiers_to_terminput(modifiers),
        })),
        egui::Event::MouseWheel {
            unit: _,
            delta,
            modifiers,
        } => Ok(Event::Mouse(MouseEvent {
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
            modifiers: key_modifiers_to_terminput(modifiers),
        })),
        egui::Event::WindowFocused(true) => Ok(Event::FocusGained),
        egui::Event::WindowFocused(false) => Ok(Event::FocusLost),
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

/// Converts the terminput [event](Event) to an egui [event](egui::Event).
pub fn to_egui(value: Event) -> Result<egui::Event, UnsupportedEvent> {
    Ok(match value {
        Event::Key(key_event) => egui::Event::Key {
            key: key_code_to_egui(key_event.code)?,
            physical_key: None,
            pressed: key_event.kind != KeyEventKind::Release,
            repeat: key_event.kind == KeyEventKind::Repeat,
            modifiers: key_modifiers_to_egui(key_event.modifiers),
        },
        Event::Mouse(mouse_event) => match mouse_event.kind {
            MouseEventKind::Down(mouse_button) | MouseEventKind::Drag(mouse_button) => {
                egui::Event::PointerButton {
                    pos: egui::Pos2 {
                        x: mouse_event.column as f32,
                        y: mouse_event.row as f32,
                    },
                    button: mouse_button_to_egui(mouse_button),
                    pressed: true,
                    modifiers: key_modifiers_to_egui(mouse_event.modifiers),
                }
            }
            MouseEventKind::Up(mouse_button) => egui::Event::PointerButton {
                pos: egui::Pos2 {
                    x: mouse_event.column as f32,
                    y: mouse_event.row as f32,
                },
                button: mouse_button_to_egui(mouse_button),
                pressed: false,
                modifiers: key_modifiers_to_egui(mouse_event.modifiers),
            },
            MouseEventKind::Moved => egui::Event::PointerMoved(egui::Pos2 {
                x: mouse_event.column as f32,
                y: mouse_event.row as f32,
            }),
            MouseEventKind::Scroll(scroll_direction) => egui::Event::MouseWheel {
                unit: egui::MouseWheelUnit::Line,
                delta: egui::Vec2 {
                    x: scroll_direction.delta().x as f32,
                    y: scroll_direction.delta().y as f32,
                },
                modifiers: key_modifiers_to_egui(mouse_event.modifiers),
            },
        },
        Event::Paste(text) => egui::Event::Paste(text),
        Event::FocusGained => egui::Event::WindowFocused(true),
        Event::FocusLost => egui::Event::WindowFocused(false),
        Event::Resize { .. } => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

fn key_code_to_terminput(value: egui::Key) -> Result<KeyCode, UnsupportedEvent> {
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
        egui::Key::Colon => Ok(KeyCode::Char(':')),
        egui::Key::Comma => Ok(KeyCode::Char(',')),
        egui::Key::Backslash => Ok(KeyCode::Char('\\')),
        egui::Key::Slash => Ok(KeyCode::Char('/')),
        egui::Key::Pipe => Ok(KeyCode::Char('|')),
        egui::Key::Questionmark => Ok(KeyCode::Char('?')),
        egui::Key::Exclamationmark => Ok(KeyCode::Char('!')),
        egui::Key::OpenBracket => Ok(KeyCode::Char('[')),
        egui::Key::CloseBracket => Ok(KeyCode::Char(']')),
        egui::Key::OpenCurlyBracket => Ok(KeyCode::Char('{')),
        egui::Key::CloseCurlyBracket => Ok(KeyCode::Char('}')),
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
        egui::Key::Copy | egui::Key::Cut | egui::Key::Paste => {
            Err(UnsupportedEvent(format!("{value:?}")))
        }
    }
}

fn key_code_to_egui(value: KeyCode) -> Result<egui::Key, UnsupportedEvent> {
    Ok(match value {
        KeyCode::Backspace => egui::Key::Backspace,
        KeyCode::Enter => egui::Key::Enter,
        KeyCode::Left => egui::Key::ArrowLeft,
        KeyCode::Right => egui::Key::ArrowRight,
        KeyCode::Up => egui::Key::ArrowUp,
        KeyCode::Down => egui::Key::ArrowDown,
        KeyCode::Home => egui::Key::Home,
        KeyCode::End => egui::Key::End,
        KeyCode::PageUp => egui::Key::PageUp,
        KeyCode::PageDown => egui::Key::PageDown,
        KeyCode::Tab => egui::Key::Tab,
        KeyCode::Delete => egui::Key::Delete,
        KeyCode::Insert => egui::Key::Insert,
        KeyCode::F(1) => egui::Key::F1,
        KeyCode::F(2) => egui::Key::F2,
        KeyCode::F(3) => egui::Key::F3,
        KeyCode::F(4) => egui::Key::F4,
        KeyCode::F(5) => egui::Key::F5,
        KeyCode::F(6) => egui::Key::F6,
        KeyCode::F(7) => egui::Key::F7,
        KeyCode::F(8) => egui::Key::F8,
        KeyCode::F(9) => egui::Key::F9,
        KeyCode::F(10) => egui::Key::F10,
        KeyCode::F(11) => egui::Key::F11,
        KeyCode::F(12) => egui::Key::F12,
        KeyCode::F(13) => egui::Key::F13,
        KeyCode::F(14) => egui::Key::F14,
        KeyCode::F(15) => egui::Key::F15,
        KeyCode::F(16) => egui::Key::F16,
        KeyCode::F(17) => egui::Key::F17,
        KeyCode::F(18) => egui::Key::F18,
        KeyCode::F(19) => egui::Key::F19,
        KeyCode::F(20) => egui::Key::F20,
        KeyCode::F(21) => egui::Key::F21,
        KeyCode::F(22) => egui::Key::F22,
        KeyCode::F(23) => egui::Key::F23,
        KeyCode::F(24) => egui::Key::F24,
        KeyCode::F(25) => egui::Key::F25,
        KeyCode::F(26) => egui::Key::F26,
        KeyCode::F(27) => egui::Key::F27,
        KeyCode::F(28) => egui::Key::F28,
        KeyCode::F(29) => egui::Key::F29,
        KeyCode::F(30) => egui::Key::F30,
        KeyCode::F(31) => egui::Key::F31,
        KeyCode::F(32) => egui::Key::F32,
        KeyCode::F(33) => egui::Key::F33,
        KeyCode::F(34) => egui::Key::F34,
        KeyCode::F(35) => egui::Key::F35,
        KeyCode::F(_) => Err(UnsupportedEvent(format!("{value:?}")))?,
        KeyCode::Char('a' | 'A') => egui::Key::A,
        KeyCode::Char('b' | 'B') => egui::Key::B,
        KeyCode::Char('c' | 'C') => egui::Key::C,
        KeyCode::Char('d' | 'D') => egui::Key::D,
        KeyCode::Char('e' | 'E') => egui::Key::E,
        KeyCode::Char('f' | 'F') => egui::Key::F,
        KeyCode::Char('g' | 'G') => egui::Key::G,
        KeyCode::Char('h' | 'H') => egui::Key::H,
        KeyCode::Char('i' | 'I') => egui::Key::I,
        KeyCode::Char('j' | 'J') => egui::Key::J,
        KeyCode::Char('k' | 'K') => egui::Key::K,
        KeyCode::Char('l' | 'L') => egui::Key::L,
        KeyCode::Char('m' | 'M') => egui::Key::M,
        KeyCode::Char('n' | 'N') => egui::Key::N,
        KeyCode::Char('o' | 'O') => egui::Key::O,
        KeyCode::Char('p' | 'P') => egui::Key::P,
        KeyCode::Char('q' | 'Q') => egui::Key::Q,
        KeyCode::Char('r' | 'R') => egui::Key::R,
        KeyCode::Char('s' | 'S') => egui::Key::S,
        KeyCode::Char('t' | 'T') => egui::Key::T,
        KeyCode::Char('u' | 'U') => egui::Key::U,
        KeyCode::Char('v' | 'V') => egui::Key::V,
        KeyCode::Char('w' | 'W') => egui::Key::W,
        KeyCode::Char('x' | 'X') => egui::Key::X,
        KeyCode::Char('y' | 'Y') => egui::Key::Y,
        KeyCode::Char('z' | 'Z') => egui::Key::Z,
        KeyCode::Char('0') => egui::Key::Num0,
        KeyCode::Char('1') => egui::Key::Num1,
        KeyCode::Char('2') => egui::Key::Num2,
        KeyCode::Char('3') => egui::Key::Num3,
        KeyCode::Char('4') => egui::Key::Num4,
        KeyCode::Char('5') => egui::Key::Num5,
        KeyCode::Char('6') => egui::Key::Num6,
        KeyCode::Char('7') => egui::Key::Num7,
        KeyCode::Char('8') => egui::Key::Num8,
        KeyCode::Char('9') => egui::Key::Num9,
        KeyCode::Char(' ') => egui::Key::Space,
        KeyCode::Char(':') => egui::Key::Colon,
        KeyCode::Char(',') => egui::Key::Comma,
        KeyCode::Char('\\') => egui::Key::Backslash,
        KeyCode::Char('/') => egui::Key::Slash,
        KeyCode::Char('|') => egui::Key::Pipe,
        KeyCode::Char('?') => egui::Key::Questionmark,
        KeyCode::Char('[') => egui::Key::OpenBracket,
        KeyCode::Char(']') => egui::Key::CloseBracket,
        KeyCode::Char('`') => egui::Key::Backtick,
        KeyCode::Char('-') => egui::Key::Minus,
        KeyCode::Char('.') => egui::Key::Period,
        KeyCode::Char('+') => egui::Key::Plus,
        KeyCode::Char('=') => egui::Key::Equals,
        KeyCode::Char(';') => egui::Key::Semicolon,
        KeyCode::Char('\'') => egui::Key::Quote,
        KeyCode::Char(_) => Err(UnsupportedEvent(format!("{value:?}")))?,
        KeyCode::Esc => egui::Key::Escape,
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

fn key_modifiers_to_terminput(value: egui::Modifiers) -> KeyModifiers {
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
    mapped
}

fn key_modifiers_to_egui(value: KeyModifiers) -> egui::Modifiers {
    egui::Modifiers {
        alt: value.intersects(KeyModifiers::ALT),
        ctrl: value.intersects(KeyModifiers::CTRL),
        shift: value.intersects(KeyModifiers::SHIFT),
        mac_cmd: false,
        command: false,
    }
}

fn mouse_button_to_terminput(value: egui::PointerButton) -> MouseButton {
    match value {
        egui::PointerButton::Primary => MouseButton::Left,
        egui::PointerButton::Secondary => MouseButton::Right,
        egui::PointerButton::Middle => MouseButton::Middle,
        egui::PointerButton::Extra1 | egui::PointerButton::Extra2 => MouseButton::Unknown,
    }
}

fn mouse_button_to_egui(value: MouseButton) -> egui::PointerButton {
    match value {
        MouseButton::Left | MouseButton::Unknown => egui::PointerButton::Primary,
        MouseButton::Right => egui::PointerButton::Secondary,
        MouseButton::Middle => egui::PointerButton::Middle,
    }
}
