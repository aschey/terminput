#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, ScrollDirection,
    UnsupportedEvent,
};

/// Converts the termwiz [`InputEvent`](termwiz::input::InputEvent) to a terminput [`Event`].
pub fn to_terminput(value: termwiz::input::InputEvent) -> Result<Event, UnsupportedEvent> {
    Ok(match value {
        termwiz::input::InputEvent::Key(key_event) => Event::Key(to_terminput_key(key_event)?),
        termwiz::input::InputEvent::Mouse(mouse_event) => {
            Event::Mouse(to_terminput_mouse(mouse_event)?)
        }
        termwiz::input::InputEvent::Resized { cols, rows } => Event::Resize {
            cols: cols as u32,
            rows: rows as u32,
        },
        termwiz::input::InputEvent::Paste(val) => Event::Paste(val),
        termwiz::input::InputEvent::PixelMouse(_) | termwiz::input::InputEvent::Wake => {
            Err(UnsupportedEvent(format!("{value:?}")))?
        }
    })
}

/// Converts the terminput [`Event`] to a termwiz [`InputEvent`](termwiz::input::InputEvent).
pub fn to_termwiz(value: Event) -> Result<termwiz::input::InputEvent, UnsupportedEvent> {
    Ok(match value {
        Event::Key(key_event) => termwiz::input::InputEvent::Key(to_termwiz_key(key_event)?),
        Event::Mouse(mouse_event) => {
            termwiz::input::InputEvent::Mouse(mouse_to_termwiz(mouse_event)?)
        }
        Event::Paste(val) => termwiz::input::InputEvent::Paste(val),
        Event::Resize { cols, rows } => termwiz::input::InputEvent::Resized {
            cols: cols as usize,
            rows: rows as usize,
        },
        Event::FocusGained | Event::FocusLost => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

/// Converts the termwiz [`KeyEvent`](termwiz::input::KeyEvent) to a terminput [`KeyEvent`].
pub fn to_terminput_key(value: termwiz::input::KeyEvent) -> Result<KeyEvent, UnsupportedEvent> {
    let (code, state) = match value.key {
        termwiz::input::KeyCode::Char(c) => (KeyCode::Char(c), KeyEventState::empty()),
        termwiz::input::KeyCode::Hyper => (
            KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Super => (
            KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Meta => (
            KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Backspace => (KeyCode::Backspace, KeyEventState::empty()),
        termwiz::input::KeyCode::Tab => (KeyCode::Tab, KeyEventState::empty()),
        termwiz::input::KeyCode::Enter => (KeyCode::Enter, KeyEventState::empty()),
        termwiz::input::KeyCode::Shift => (
            KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Escape => (KeyCode::Esc, KeyEventState::empty()),
        termwiz::input::KeyCode::LeftShift => (
            KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Left),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::RightShift => (
            KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Right),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Control => (
            KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::LeftControl => (
            KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Left),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::RightControl => (
            KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Right),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Alt => (
            KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Unknown),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::LeftAlt => (
            KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Left),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::RightAlt => (
            KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Menu => (KeyCode::Menu, KeyEventState::empty()),
        termwiz::input::KeyCode::LeftMenu => (KeyCode::Menu, KeyEventState::empty()),
        termwiz::input::KeyCode::RightMenu => (KeyCode::Menu, KeyEventState::empty()),
        termwiz::input::KeyCode::Pause => (KeyCode::Pause, KeyEventState::empty()),
        termwiz::input::KeyCode::CapsLock => (KeyCode::CapsLock, KeyEventState::CAPS_LOCK),
        termwiz::input::KeyCode::PageUp => (KeyCode::PageUp, KeyEventState::empty()),
        termwiz::input::KeyCode::PageDown => (KeyCode::PageDown, KeyEventState::empty()),
        termwiz::input::KeyCode::End => (KeyCode::End, KeyEventState::empty()),
        termwiz::input::KeyCode::Home => (KeyCode::Home, KeyEventState::empty()),
        termwiz::input::KeyCode::LeftArrow => (KeyCode::Left, KeyEventState::empty()),
        termwiz::input::KeyCode::RightArrow => (KeyCode::Right, KeyEventState::empty()),
        termwiz::input::KeyCode::UpArrow => (KeyCode::Up, KeyEventState::empty()),
        termwiz::input::KeyCode::DownArrow => (KeyCode::Down, KeyEventState::empty()),
        termwiz::input::KeyCode::Print => (KeyCode::PrintScreen, KeyEventState::empty()),
        termwiz::input::KeyCode::PrintScreen => (KeyCode::PrintScreen, KeyEventState::empty()),
        termwiz::input::KeyCode::Insert => (KeyCode::Insert, KeyEventState::empty()),
        termwiz::input::KeyCode::Delete => (KeyCode::Delete, KeyEventState::empty()),
        termwiz::input::KeyCode::LeftWindows => (
            KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Left),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::RightWindows => (
            KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Right),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::Numpad0 => (KeyCode::Char('0'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad1 => (KeyCode::Char('1'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad2 => (KeyCode::Char('2'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad3 => (KeyCode::Char('3'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad4 => (KeyCode::Char('4'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad5 => (KeyCode::Char('5'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad6 => (KeyCode::Char('6'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad7 => (KeyCode::Char('7'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad8 => (KeyCode::Char('8'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Numpad9 => (KeyCode::Char('9'), KeyEventState::KEYPAD),
        termwiz::input::KeyCode::Multiply => (KeyCode::Char('*'), KeyEventState::empty()),
        termwiz::input::KeyCode::Add => (KeyCode::Char('+'), KeyEventState::empty()),
        termwiz::input::KeyCode::Subtract => (KeyCode::Char('-'), KeyEventState::empty()),
        termwiz::input::KeyCode::Decimal => (KeyCode::Char('.'), KeyEventState::empty()),
        termwiz::input::KeyCode::Divide => (KeyCode::Char('/'), KeyEventState::empty()),
        termwiz::input::KeyCode::Function(f) => (KeyCode::F(f), KeyEventState::empty()),
        termwiz::input::KeyCode::NumLock => (KeyCode::NumLock, KeyEventState::NUM_LOCK),
        termwiz::input::KeyCode::ScrollLock => (KeyCode::ScrollLock, KeyEventState::empty()),
        termwiz::input::KeyCode::VolumeMute => (
            KeyCode::Media(MediaKeyCode::MuteVolume),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::VolumeDown => (
            KeyCode::Media(MediaKeyCode::LowerVolume),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::VolumeUp => (
            KeyCode::Media(MediaKeyCode::RaiseVolume),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::MediaNextTrack => (
            KeyCode::Media(MediaKeyCode::TrackNext),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::MediaPrevTrack => (
            KeyCode::Media(MediaKeyCode::TrackPrevious),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::MediaStop => {
            (KeyCode::Media(MediaKeyCode::Stop), KeyEventState::empty())
        }
        termwiz::input::KeyCode::MediaPlayPause => (
            KeyCode::Media(MediaKeyCode::PlayPause),
            KeyEventState::empty(),
        ),
        termwiz::input::KeyCode::ApplicationLeftArrow => (KeyCode::Left, KeyEventState::empty()),
        termwiz::input::KeyCode::ApplicationRightArrow => (KeyCode::Right, KeyEventState::empty()),
        termwiz::input::KeyCode::ApplicationUpArrow => (KeyCode::Up, KeyEventState::empty()),
        termwiz::input::KeyCode::ApplicationDownArrow => (KeyCode::Down, KeyEventState::empty()),
        termwiz::input::KeyCode::KeyPadHome => (KeyCode::Home, KeyEventState::KEYPAD),
        termwiz::input::KeyCode::KeyPadEnd => (KeyCode::End, KeyEventState::KEYPAD),
        termwiz::input::KeyCode::KeyPadPageUp => (KeyCode::PageUp, KeyEventState::KEYPAD),
        termwiz::input::KeyCode::KeyPadPageDown => (KeyCode::PageDown, KeyEventState::KEYPAD),
        termwiz::input::KeyCode::KeyPadBegin => (KeyCode::KeypadBegin, KeyEventState::KEYPAD),
        _ => Err(UnsupportedEvent(format!("{value:?}")))?,
    };
    Ok(KeyEvent {
        code,
        modifiers: to_terminput_key_modifiers(value.modifiers),
        kind: KeyEventKind::Press,
        state,
    })
}

/// Converts the terminput [`KeyEvent`] to a termwiz [`KeyEvent`](termwiz::input::KeyEvent).
pub fn to_termwiz_key(value: KeyEvent) -> Result<termwiz::input::KeyEvent, UnsupportedEvent> {
    let is_keypad = value.state.intersects(KeyEventState::KEYPAD);
    let key = match value.code {
        KeyCode::Backspace => termwiz::input::KeyCode::Backspace,
        KeyCode::Enter => termwiz::input::KeyCode::Enter,
        KeyCode::Left => termwiz::input::KeyCode::LeftArrow,
        KeyCode::Right => termwiz::input::KeyCode::RightArrow,
        KeyCode::Up => termwiz::input::KeyCode::UpArrow,
        KeyCode::Down => termwiz::input::KeyCode::DownArrow,
        KeyCode::Home if is_keypad => termwiz::input::KeyCode::KeyPadHome,
        KeyCode::Home => termwiz::input::KeyCode::Home,
        KeyCode::End if is_keypad => termwiz::input::KeyCode::KeyPadEnd,
        KeyCode::End => termwiz::input::KeyCode::End,
        KeyCode::PageUp if is_keypad => termwiz::input::KeyCode::KeyPadPageUp,
        KeyCode::PageUp => termwiz::input::KeyCode::PageUp,
        KeyCode::PageDown if is_keypad => termwiz::input::KeyCode::KeyPadPageDown,
        KeyCode::PageDown => termwiz::input::KeyCode::PageDown,
        KeyCode::Tab => termwiz::input::KeyCode::Tab,
        KeyCode::Delete => termwiz::input::KeyCode::Delete,
        KeyCode::Insert => termwiz::input::KeyCode::Insert,
        KeyCode::F(f) => termwiz::input::KeyCode::Function(f),
        KeyCode::Char('0') if is_keypad => termwiz::input::KeyCode::Numpad0,
        KeyCode::Char('1') if is_keypad => termwiz::input::KeyCode::Numpad1,
        KeyCode::Char('2') if is_keypad => termwiz::input::KeyCode::Numpad2,
        KeyCode::Char('3') if is_keypad => termwiz::input::KeyCode::Numpad3,
        KeyCode::Char('4') if is_keypad => termwiz::input::KeyCode::Numpad4,
        KeyCode::Char('5') if is_keypad => termwiz::input::KeyCode::Numpad5,
        KeyCode::Char('6') if is_keypad => termwiz::input::KeyCode::Numpad6,
        KeyCode::Char('7') if is_keypad => termwiz::input::KeyCode::Numpad7,
        KeyCode::Char('8') if is_keypad => termwiz::input::KeyCode::Numpad8,
        KeyCode::Char('9') if is_keypad => termwiz::input::KeyCode::Numpad9,
        KeyCode::Char(c) => termwiz::input::KeyCode::Char(c),
        KeyCode::Esc => termwiz::input::KeyCode::Escape,
        KeyCode::CapsLock => termwiz::input::KeyCode::CapsLock,
        KeyCode::ScrollLock => termwiz::input::KeyCode::ScrollLock,
        KeyCode::NumLock => termwiz::input::KeyCode::NumLock,
        KeyCode::PrintScreen => termwiz::input::KeyCode::PrintScreen,
        KeyCode::Pause => termwiz::input::KeyCode::Pause,
        KeyCode::Menu => termwiz::input::KeyCode::Menu,
        KeyCode::KeypadBegin => termwiz::input::KeyCode::KeyPadBegin,
        KeyCode::Media(MediaKeyCode::Play) => termwiz::input::KeyCode::MediaPlayPause,
        KeyCode::Media(MediaKeyCode::Pause) => termwiz::input::KeyCode::MediaPlayPause,
        KeyCode::Media(MediaKeyCode::PlayPause) => termwiz::input::KeyCode::MediaPlayPause,
        KeyCode::Media(MediaKeyCode::Stop) => termwiz::input::KeyCode::MediaStop,
        KeyCode::Media(
            MediaKeyCode::Reverse
            | MediaKeyCode::FastForward
            | MediaKeyCode::Rewind
            | MediaKeyCode::Record,
        ) => {
            return Err(UnsupportedEvent(format!("{value:?}")));
        }
        KeyCode::Media(MediaKeyCode::TrackNext) => termwiz::input::KeyCode::MediaNextTrack,
        KeyCode::Media(MediaKeyCode::TrackPrevious) => termwiz::input::KeyCode::MediaPrevTrack,
        KeyCode::Media(MediaKeyCode::LowerVolume) => termwiz::input::KeyCode::VolumeDown,
        KeyCode::Media(MediaKeyCode::RaiseVolume) => termwiz::input::KeyCode::VolumeUp,
        KeyCode::Media(MediaKeyCode::MuteVolume) => termwiz::input::KeyCode::VolumeMute,
        KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Left) => {
            termwiz::input::KeyCode::LeftAlt
        }
        KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right) => {
            termwiz::input::KeyCode::RightAlt
        }
        KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Unknown) => {
            termwiz::input::KeyCode::Alt
        }
        KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Left) => {
            termwiz::input::KeyCode::LeftControl
        }
        KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Right) => {
            termwiz::input::KeyCode::RightControl
        }
        KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Unknown) => {
            termwiz::input::KeyCode::Control
        }
        KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Left) => {
            termwiz::input::KeyCode::LeftShift
        }
        KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Right) => {
            termwiz::input::KeyCode::RightShift
        }
        KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Unknown) => {
            termwiz::input::KeyCode::Shift
        }
        KeyCode::Modifier(ModifierKeyCode::Super, _) => termwiz::input::KeyCode::Super,
        KeyCode::Modifier(ModifierKeyCode::Hyper, _) => termwiz::input::KeyCode::Hyper,
        KeyCode::Modifier(ModifierKeyCode::Meta, _) => termwiz::input::KeyCode::Meta,
        KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift | ModifierKeyCode::IsoLevel5Shift, _) => {
            return Err(UnsupportedEvent(format!("{value:?}")));
        }
    };

    Ok(termwiz::input::KeyEvent {
        key,
        modifiers: to_termwiz_key_modifiers(value.modifiers),
    })
}

fn to_terminput_key_modifiers(value: termwiz::input::Modifiers) -> KeyModifiers {
    let mut res = KeyModifiers::empty();
    if value.intersects(
        termwiz::input::Modifiers::ALT
            | termwiz::input::Modifiers::LEFT_ALT
            | termwiz::input::Modifiers::RIGHT_ALT,
    ) {
        res |= KeyModifiers::ALT;
    }
    if value.intersects(
        termwiz::input::Modifiers::SHIFT
            | termwiz::input::Modifiers::LEFT_SHIFT
            | termwiz::input::Modifiers::RIGHT_SHIFT,
    ) {
        res |= KeyModifiers::SHIFT;
    }

    if value.intersects(
        termwiz::input::Modifiers::CTRL
            | termwiz::input::Modifiers::LEFT_CTRL
            | termwiz::input::Modifiers::RIGHT_CTRL,
    ) {
        res |= KeyModifiers::CTRL;
    }
    if value.intersects(termwiz::input::Modifiers::SUPER) {
        res |= KeyModifiers::SUPER;
    }

    res
}

fn to_termwiz_key_modifiers(value: KeyModifiers) -> termwiz::input::Modifiers {
    let mut res = termwiz::input::Modifiers::empty();
    if value.intersects(KeyModifiers::ALT) {
        res |= termwiz::input::Modifiers::ALT;
    }
    if value.intersects(KeyModifiers::SHIFT) {
        res |= termwiz::input::Modifiers::SHIFT;
    }
    if value.intersects(KeyModifiers::CTRL) {
        res |= termwiz::input::Modifiers::CTRL;
    }
    if value.intersects(KeyModifiers::SUPER) {
        res |= termwiz::input::Modifiers::SUPER;
    }

    res
}
/// Converts the termwiz [`MouseEvent`](termwiz::input::MouseEvent) to a terminput [`MouseEvent`].
pub fn to_terminput_mouse(
    value: termwiz::input::MouseEvent,
) -> Result<MouseEvent, UnsupportedEvent> {
    if value
        .mouse_buttons
        .contains(termwiz::input::MouseButtons::LEFT)
    {
        return Ok(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value
        .mouse_buttons
        .contains(termwiz::input::MouseButtons::RIGHT)
    {
        return Ok(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value
        .mouse_buttons
        .contains(termwiz::input::MouseButtons::MIDDLE)
    {
        return Ok(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Middle),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }

    if value.mouse_buttons.contains(
        termwiz::input::MouseButtons::VERT_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
    ) {
        return Ok(MouseEvent {
            kind: MouseEventKind::Scroll(ScrollDirection::Up),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value
        .mouse_buttons
        .contains(termwiz::input::MouseButtons::VERT_WHEEL)
    {
        return Ok(MouseEvent {
            kind: MouseEventKind::Scroll(ScrollDirection::Down),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value.mouse_buttons.contains(
        termwiz::input::MouseButtons::HORZ_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
    ) {
        return Ok(MouseEvent {
            kind: MouseEventKind::Scroll(ScrollDirection::Left),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value
        .mouse_buttons
        .contains(termwiz::input::MouseButtons::HORZ_WHEEL)
    {
        return Ok(MouseEvent {
            kind: MouseEventKind::Scroll(ScrollDirection::Right),
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }
    if value.mouse_buttons == termwiz::input::MouseButtons::NONE {
        return Ok(MouseEvent {
            kind: MouseEventKind::Moved,
            column: value.x - 1,
            row: value.y - 1,
            modifiers: to_terminput_key_modifiers(value.modifiers),
        });
    }

    Err(UnsupportedEvent(format!("{value:?}")))
}

/// Converts the terminput [`MouseEvent`] to a termwiz [`MouseEvent`](termwiz::input::MouseEvent).
pub fn mouse_to_termwiz(value: MouseEvent) -> Result<termwiz::input::MouseEvent, UnsupportedEvent> {
    Ok(match value.kind {
        MouseEventKind::Down(MouseButton::Left | MouseButton::Unknown) => {
            termwiz::input::MouseEvent {
                mouse_buttons: termwiz::input::MouseButtons::LEFT,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: to_termwiz_key_modifiers(value.modifiers),
            }
        }
        MouseEventKind::Down(MouseButton::Right) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::RIGHT,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Down(MouseButton::Middle) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::MIDDLE,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Up(_) | MouseEventKind::Drag(_) => {
            Err(UnsupportedEvent(format!("{value:?}")))?
        }
        MouseEventKind::Moved => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::NONE,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Scroll(ScrollDirection::Down) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::VERT_WHEEL,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Scroll(ScrollDirection::Up) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::VERT_WHEEL
                | termwiz::input::MouseButtons::WHEEL_POSITIVE,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Scroll(ScrollDirection::Left) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::HORZ_WHEEL,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
        MouseEventKind::Scroll(ScrollDirection::Right) => termwiz::input::MouseEvent {
            mouse_buttons: termwiz::input::MouseButtons::HORZ_WHEEL
                | termwiz::input::MouseButtons::WHEEL_POSITIVE,
            x: value.column + 1,
            y: value.row + 1,
            modifiers: to_termwiz_key_modifiers(value.modifiers),
        },
    })
}
