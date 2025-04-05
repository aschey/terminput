#![deny(missing_docs)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, ScrollDirection,
    UnsupportedEvent,
};

/// Converts the crossterm [event](crossterm::event::Event) to a terminput [event](Event).
pub fn to_terminput(value: crossterm::event::Event) -> Result<Event, UnsupportedEvent> {
    Ok(match value {
        crossterm::event::Event::FocusGained => Event::FocusGained,
        crossterm::event::Event::FocusLost => Event::FocusLost,
        crossterm::event::Event::Key(key_event) => Event::Key(key_event_to_terminput(key_event)?),
        crossterm::event::Event::Mouse(mouse_event) => {
            Event::Mouse(mouse_to_terminput(mouse_event))
        }
        crossterm::event::Event::Paste(value) => Event::Paste(value),
        crossterm::event::Event::Resize(cols, rows) => Event::Resize {
            cols: cols as u32,
            rows: rows as u32,
        },
    })
}

/// Converts the terminput [event](Event) to a crossterm [event](crossterm::event::Event).
pub fn to_crossterm(value: Event) -> Result<crossterm::event::Event, UnsupportedEvent> {
    Ok(match value {
        Event::FocusGained => crossterm::event::Event::FocusGained,
        Event::FocusLost => crossterm::event::Event::FocusLost,
        Event::Key(key_event) => crossterm::event::Event::Key(key_event_to_crossterm(key_event)),
        Event::Mouse(mouse_event) => {
            crossterm::event::Event::Mouse(mouse_to_crossterm(mouse_event)?)
        }
        Event::Paste(value) => crossterm::event::Event::Paste(value),
        Event::Resize { cols, rows } => crossterm::event::Event::Resize(
            cols.try_into()
                .map_err(|e| UnsupportedEvent(format!("{e:?}")))?,
            rows.try_into()
                .map_err(|e| UnsupportedEvent(format!("{e:?}")))?,
        ),
    })
}

fn mouse_to_terminput(value: crossterm::event::MouseEvent) -> MouseEvent {
    MouseEvent {
        kind: mouse_kind_to_terminput(value.kind),
        column: value.column,
        row: value.row,
        modifiers: key_modifiers_to_terminput(value.modifiers),
    }
}

fn mouse_to_crossterm(value: MouseEvent) -> Result<crossterm::event::MouseEvent, UnsupportedEvent> {
    Ok(crossterm::event::MouseEvent {
        kind: mouse_kind_to_crossterm(value.kind)?,
        column: value.column,
        row: value.row,
        modifiers: key_modifiers_to_crossterm(value.modifiers),
    })
}

fn mouse_kind_to_terminput(value: crossterm::event::MouseEventKind) -> MouseEventKind {
    match value {
        crossterm::event::MouseEventKind::Down(button) => {
            MouseEventKind::Down(mouse_button_to_terminput(button))
        }
        crossterm::event::MouseEventKind::Up(button) => {
            MouseEventKind::Up(mouse_button_to_terminput(button))
        }
        crossterm::event::MouseEventKind::Drag(button) => {
            MouseEventKind::Drag(mouse_button_to_terminput(button))
        }
        crossterm::event::MouseEventKind::Moved => MouseEventKind::Moved,
        crossterm::event::MouseEventKind::ScrollDown => {
            MouseEventKind::Scroll(ScrollDirection::Down)
        }
        crossterm::event::MouseEventKind::ScrollUp => MouseEventKind::Scroll(ScrollDirection::Up),
        crossterm::event::MouseEventKind::ScrollLeft => {
            MouseEventKind::Scroll(ScrollDirection::Left)
        }
        crossterm::event::MouseEventKind::ScrollRight => {
            MouseEventKind::Scroll(ScrollDirection::Right)
        }
    }
}

fn mouse_kind_to_crossterm(
    value: MouseEventKind,
) -> Result<crossterm::event::MouseEventKind, UnsupportedEvent> {
    Ok(match value {
        MouseEventKind::Down(button) => {
            crossterm::event::MouseEventKind::Down(mouse_button_to_crossterm(button)?)
        }
        MouseEventKind::Up(button) => {
            crossterm::event::MouseEventKind::Up(mouse_button_to_crossterm(button)?)
        }
        MouseEventKind::Drag(button) => {
            crossterm::event::MouseEventKind::Drag(mouse_button_to_crossterm(button)?)
        }
        MouseEventKind::Moved => crossterm::event::MouseEventKind::Moved,
        MouseEventKind::Scroll(ScrollDirection::Down) => {
            crossterm::event::MouseEventKind::ScrollDown
        }
        MouseEventKind::Scroll(ScrollDirection::Up) => crossterm::event::MouseEventKind::ScrollUp,
        MouseEventKind::Scroll(ScrollDirection::Left) => {
            crossterm::event::MouseEventKind::ScrollLeft
        }
        MouseEventKind::Scroll(ScrollDirection::Right) => {
            crossterm::event::MouseEventKind::ScrollRight
        }
    })
}

fn mouse_button_to_terminput(value: crossterm::event::MouseButton) -> MouseButton {
    match value {
        crossterm::event::MouseButton::Left => MouseButton::Left,
        crossterm::event::MouseButton::Right => MouseButton::Right,
        crossterm::event::MouseButton::Middle => MouseButton::Middle,
    }
}

fn mouse_button_to_crossterm(
    value: MouseButton,
) -> Result<crossterm::event::MouseButton, UnsupportedEvent> {
    Ok(match value {
        MouseButton::Left => crossterm::event::MouseButton::Left,
        MouseButton::Right => crossterm::event::MouseButton::Right,
        MouseButton::Middle => crossterm::event::MouseButton::Middle,
        val @ MouseButton::Unknown => Err(UnsupportedEvent(format!("{val:?}")))?,
    })
}

fn key_event_to_terminput(value: crossterm::event::KeyEvent) -> Result<KeyEvent, UnsupportedEvent> {
    Ok(KeyEvent {
        code: key_code_to_terminput(value.code)?,
        modifiers: key_modifiers_to_terminput(value.modifiers),
        kind: key_kind_to_terminput(value.kind),
        state: key_state_to_terminput(value.state),
    })
}

fn key_event_to_crossterm(value: KeyEvent) -> crossterm::event::KeyEvent {
    crossterm::event::KeyEvent {
        code: convert_crossterm_key_code(
            value.code,
            value.modifiers.intersects(KeyModifiers::SHIFT),
        ),
        modifiers: key_modifiers_to_crossterm(value.modifiers),
        kind: key_kind_to_crossterm(value.kind),
        state: key_state_to_crossterm(value.state),
    }
}

fn key_code_to_terminput(value: crossterm::event::KeyCode) -> Result<KeyCode, UnsupportedEvent> {
    Ok(match value {
        crossterm::event::KeyCode::Backspace => KeyCode::Backspace,
        crossterm::event::KeyCode::Enter => KeyCode::Enter,
        crossterm::event::KeyCode::Left => KeyCode::Left,
        crossterm::event::KeyCode::Right => KeyCode::Right,
        crossterm::event::KeyCode::Up => KeyCode::Up,
        crossterm::event::KeyCode::Down => KeyCode::Down,
        crossterm::event::KeyCode::Home => KeyCode::Home,
        crossterm::event::KeyCode::End => KeyCode::End,
        crossterm::event::KeyCode::PageUp => KeyCode::PageUp,
        crossterm::event::KeyCode::PageDown => KeyCode::PageDown,
        crossterm::event::KeyCode::Tab => KeyCode::Tab,
        crossterm::event::KeyCode::BackTab => KeyCode::Tab,
        crossterm::event::KeyCode::Delete => KeyCode::Delete,
        crossterm::event::KeyCode::Insert => KeyCode::Insert,
        crossterm::event::KeyCode::F(f) => KeyCode::F(f),
        crossterm::event::KeyCode::Char(c) => KeyCode::Char(c),
        crossterm::event::KeyCode::Esc => KeyCode::Esc,
        crossterm::event::KeyCode::CapsLock => KeyCode::CapsLock,
        crossterm::event::KeyCode::ScrollLock => KeyCode::ScrollLock,
        crossterm::event::KeyCode::NumLock => KeyCode::NumLock,
        crossterm::event::KeyCode::PrintScreen => KeyCode::PrintScreen,
        crossterm::event::KeyCode::Pause => KeyCode::Pause,
        crossterm::event::KeyCode::Menu => KeyCode::Menu,
        crossterm::event::KeyCode::KeypadBegin => KeyCode::KeypadBegin,
        crossterm::event::KeyCode::Media(m) => KeyCode::Media(media_code_to_terminput(m)),
        crossterm::event::KeyCode::Modifier(m) => {
            let (code, direction) = convert_modifier_key_code(m);
            KeyCode::Modifier(code, direction)
        }
        crossterm::event::KeyCode::Null => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

fn convert_crossterm_key_code(value: KeyCode, shift: bool) -> crossterm::event::KeyCode {
    match value {
        KeyCode::Backspace => crossterm::event::KeyCode::Backspace,
        KeyCode::Enter => crossterm::event::KeyCode::Enter,
        KeyCode::Left => crossterm::event::KeyCode::Left,
        KeyCode::Right => crossterm::event::KeyCode::Right,
        KeyCode::Up => crossterm::event::KeyCode::Up,
        KeyCode::Down => crossterm::event::KeyCode::Down,
        KeyCode::Home => crossterm::event::KeyCode::Home,
        KeyCode::End => crossterm::event::KeyCode::End,
        KeyCode::PageUp => crossterm::event::KeyCode::PageUp,
        KeyCode::PageDown => crossterm::event::KeyCode::PageDown,
        KeyCode::Tab if shift => crossterm::event::KeyCode::BackTab,
        KeyCode::Tab => crossterm::event::KeyCode::Tab,
        KeyCode::Delete => crossterm::event::KeyCode::Delete,
        KeyCode::Insert => crossterm::event::KeyCode::Insert,
        KeyCode::F(f) => crossterm::event::KeyCode::F(f),
        KeyCode::Char(c) => crossterm::event::KeyCode::Char(c),
        KeyCode::Esc => crossterm::event::KeyCode::Esc,
        KeyCode::CapsLock => crossterm::event::KeyCode::CapsLock,
        KeyCode::ScrollLock => crossterm::event::KeyCode::ScrollLock,
        KeyCode::NumLock => crossterm::event::KeyCode::NumLock,
        KeyCode::PrintScreen => crossterm::event::KeyCode::PrintScreen,
        KeyCode::Pause => crossterm::event::KeyCode::Pause,
        KeyCode::Menu => crossterm::event::KeyCode::Menu,
        KeyCode::KeypadBegin => crossterm::event::KeyCode::KeypadBegin,
        KeyCode::Media(m) => crossterm::event::KeyCode::Media(media_code_to_crossterm(m)),
        KeyCode::Modifier(code, direction) => crossterm::event::KeyCode::Modifier(
            convert_crossterm_modifier_key_code(code, direction),
        ),
    }
}

fn key_modifiers_to_terminput(value: crossterm::event::KeyModifiers) -> KeyModifiers {
    let mut res = KeyModifiers::empty();
    if value.intersects(crossterm::event::KeyModifiers::ALT) {
        res |= KeyModifiers::ALT;
    }
    if value.intersects(crossterm::event::KeyModifiers::SHIFT) {
        res |= KeyModifiers::SHIFT;
    }
    if value.intersects(crossterm::event::KeyModifiers::CONTROL) {
        res |= KeyModifiers::CTRL;
    }
    if value.intersects(crossterm::event::KeyModifiers::SUPER) {
        res |= KeyModifiers::SUPER;
    }

    res
}

fn key_modifiers_to_crossterm(value: KeyModifiers) -> crossterm::event::KeyModifiers {
    let mut res = crossterm::event::KeyModifiers::empty();
    if value.intersects(KeyModifiers::ALT) {
        res |= crossterm::event::KeyModifiers::ALT;
    }
    if value.intersects(KeyModifiers::SHIFT) {
        res |= crossterm::event::KeyModifiers::SHIFT;
    }
    if value.intersects(KeyModifiers::CTRL) {
        res |= crossterm::event::KeyModifiers::CONTROL;
    }
    if value.intersects(KeyModifiers::SUPER) {
        res |= crossterm::event::KeyModifiers::SUPER;
    }

    res
}

fn key_kind_to_terminput(value: crossterm::event::KeyEventKind) -> KeyEventKind {
    match value {
        crossterm::event::KeyEventKind::Press => KeyEventKind::Press,
        crossterm::event::KeyEventKind::Repeat => KeyEventKind::Repeat,
        crossterm::event::KeyEventKind::Release => KeyEventKind::Release,
    }
}

fn key_kind_to_crossterm(value: KeyEventKind) -> crossterm::event::KeyEventKind {
    match value {
        KeyEventKind::Press => crossterm::event::KeyEventKind::Press,
        KeyEventKind::Repeat => crossterm::event::KeyEventKind::Repeat,
        KeyEventKind::Release => crossterm::event::KeyEventKind::Release,
    }
}

fn media_code_to_terminput(value: crossterm::event::MediaKeyCode) -> MediaKeyCode {
    match value {
        crossterm::event::MediaKeyCode::Play => MediaKeyCode::Play,
        crossterm::event::MediaKeyCode::Pause => MediaKeyCode::Pause,
        crossterm::event::MediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
        crossterm::event::MediaKeyCode::Reverse => MediaKeyCode::Reverse,
        crossterm::event::MediaKeyCode::Stop => MediaKeyCode::Stop,
        crossterm::event::MediaKeyCode::FastForward => MediaKeyCode::FastForward,
        crossterm::event::MediaKeyCode::Rewind => MediaKeyCode::Rewind,
        crossterm::event::MediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
        crossterm::event::MediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
        crossterm::event::MediaKeyCode::Record => MediaKeyCode::Record,
        crossterm::event::MediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
        crossterm::event::MediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
        crossterm::event::MediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume,
    }
}

fn media_code_to_crossterm(value: MediaKeyCode) -> crossterm::event::MediaKeyCode {
    match value {
        MediaKeyCode::Play => crossterm::event::MediaKeyCode::Play,
        MediaKeyCode::Pause => crossterm::event::MediaKeyCode::Pause,
        MediaKeyCode::PlayPause => crossterm::event::MediaKeyCode::PlayPause,
        MediaKeyCode::Reverse => crossterm::event::MediaKeyCode::Reverse,
        MediaKeyCode::Stop => crossterm::event::MediaKeyCode::Stop,
        MediaKeyCode::FastForward => crossterm::event::MediaKeyCode::FastForward,
        MediaKeyCode::Rewind => crossterm::event::MediaKeyCode::Rewind,
        MediaKeyCode::TrackNext => crossterm::event::MediaKeyCode::TrackNext,
        MediaKeyCode::TrackPrevious => crossterm::event::MediaKeyCode::TrackPrevious,
        MediaKeyCode::Record => crossterm::event::MediaKeyCode::Record,
        MediaKeyCode::LowerVolume => crossterm::event::MediaKeyCode::LowerVolume,
        MediaKeyCode::RaiseVolume => crossterm::event::MediaKeyCode::RaiseVolume,
        MediaKeyCode::MuteVolume => crossterm::event::MediaKeyCode::MuteVolume,
    }
}

fn convert_modifier_key_code(
    value: crossterm::event::ModifierKeyCode,
) -> (ModifierKeyCode, ModifierDirection) {
    match value {
        crossterm::event::ModifierKeyCode::LeftShift => {
            (ModifierKeyCode::Shift, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::LeftControl => {
            (ModifierKeyCode::Control, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::LeftAlt => {
            (ModifierKeyCode::Alt, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::LeftSuper => {
            (ModifierKeyCode::Super, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::LeftHyper => {
            (ModifierKeyCode::Hyper, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::LeftMeta => {
            (ModifierKeyCode::Meta, ModifierDirection::Left)
        }
        crossterm::event::ModifierKeyCode::RightShift => {
            (ModifierKeyCode::Shift, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::RightControl => {
            (ModifierKeyCode::Control, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::RightAlt => {
            (ModifierKeyCode::Alt, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::RightSuper => {
            (ModifierKeyCode::Super, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::RightHyper => {
            (ModifierKeyCode::Hyper, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::RightMeta => {
            (ModifierKeyCode::Meta, ModifierDirection::Right)
        }
        crossterm::event::ModifierKeyCode::IsoLevel3Shift => {
            (ModifierKeyCode::IsoLevel3Shift, ModifierDirection::Unknown)
        }
        crossterm::event::ModifierKeyCode::IsoLevel5Shift => {
            (ModifierKeyCode::IsoLevel5Shift, ModifierDirection::Unknown)
        }
    }
}

fn convert_crossterm_modifier_key_code(
    code: ModifierKeyCode,
    direction: ModifierDirection,
) -> crossterm::event::ModifierKeyCode {
    match (code, direction) {
        (ModifierKeyCode::Shift, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftShift
        }
        (ModifierKeyCode::Control, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftControl
        }
        (ModifierKeyCode::Alt, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftAlt
        }
        (ModifierKeyCode::Super, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftSuper
        }
        (ModifierKeyCode::Hyper, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftHyper
        }
        (ModifierKeyCode::Meta, ModifierDirection::Left | ModifierDirection::Unknown) => {
            crossterm::event::ModifierKeyCode::LeftMeta
        }
        (ModifierKeyCode::Shift, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightShift
        }
        (ModifierKeyCode::Control, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightControl
        }
        (ModifierKeyCode::Alt, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightAlt
        }
        (ModifierKeyCode::Super, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightSuper
        }
        (ModifierKeyCode::Hyper, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightHyper
        }
        (ModifierKeyCode::Meta, ModifierDirection::Right) => {
            crossterm::event::ModifierKeyCode::RightMeta
        }
        (ModifierKeyCode::IsoLevel3Shift, _) => crossterm::event::ModifierKeyCode::IsoLevel3Shift,
        (ModifierKeyCode::IsoLevel5Shift, _) => crossterm::event::ModifierKeyCode::IsoLevel5Shift,
    }
}

fn key_state_to_terminput(value: crossterm::event::KeyEventState) -> KeyEventState {
    KeyEventState::from_bits_retain(value.bits())
}

fn key_state_to_crossterm(value: KeyEventState) -> crossterm::event::KeyEventState {
    crossterm::event::KeyEventState::from_bits_retain(value.bits())
}
