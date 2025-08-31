#[cfg(feature = "termina_0_1")]
use termina_0_1 as termina;
use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, ScrollDirection,
    UnsupportedEvent,
};

/// Converts the termina [`Event`](termina::Event) to a terminput [`Event`].
pub fn to_terminput(value: termina::Event) -> Result<Event, UnsupportedEvent> {
    Ok(match value {
        termina::Event::FocusIn => Event::FocusGained,
        termina::Event::FocusOut => Event::FocusLost,
        termina::Event::Key(key_event) => Event::Key(to_terminput_key(key_event)?),
        termina::Event::Mouse(mouse_event) => Event::Mouse(to_terminput_mouse(mouse_event)),
        termina::Event::Paste(value) => Event::Paste(value),
        termina::Event::WindowResized(window_size) => Event::Resize {
            cols: window_size.cols as u32,
            rows: window_size.rows as u32,
        },
        event @ (termina::Event::Dcs(_) | termina::Event::Csi(_)) => {
            Err(UnsupportedEvent(format!("{event:?}")))?
        }
    })
}

/// Converts the terminput [`Event`] to a termina [`Event`](termina::Event).
pub fn to_termina(value: Event) -> Result<termina::Event, UnsupportedEvent> {
    Ok(match value {
        Event::FocusGained => termina::Event::FocusIn,
        Event::FocusLost => termina::Event::FocusOut,
        Event::Key(key_event) => termina::Event::Key(to_termina_key(key_event)),
        Event::Mouse(mouse_event) => termina::Event::Mouse(to_termina_mouse(mouse_event)?),
        Event::Paste(value) => termina::Event::Paste(value),
        Event::Resize { cols, rows } => termina::Event::WindowResized(termina::WindowSize {
            cols: cols
                .try_into()
                .map_err(|e| UnsupportedEvent(format!("{e:?}")))?,
            rows: rows
                .try_into()
                .map_err(|e| UnsupportedEvent(format!("{e:?}")))?,
            pixel_width: None,
            pixel_height: None,
        }),
    })
}

/// Converts the termina [`MouseEvent`](termina::event::MouseEvent) to a terminput [`MouseEvent`].
pub fn to_terminput_mouse(value: termina::event::MouseEvent) -> MouseEvent {
    MouseEvent {
        kind: mouse_kind_to_terminput(value.kind),
        column: value.column,
        row: value.row,
        modifiers: key_modifiers_to_terminput(value.modifiers),
    }
}

/// Converts the terminput [`MouseEvent`] to a termina [`MouseEvent`](termina::event::MouseEvent).
pub fn to_termina_mouse(value: MouseEvent) -> Result<termina::event::MouseEvent, UnsupportedEvent> {
    Ok(termina::event::MouseEvent {
        kind: mouse_kind_to_termina(value.kind)?,
        column: value.column,
        row: value.row,
        modifiers: key_modifiers_to_termina(value.modifiers),
    })
}

fn mouse_kind_to_terminput(value: termina::event::MouseEventKind) -> MouseEventKind {
    match value {
        termina::event::MouseEventKind::Down(button) => {
            MouseEventKind::Down(mouse_button_to_terminput(button))
        }
        termina::event::MouseEventKind::Up(button) => {
            MouseEventKind::Up(mouse_button_to_terminput(button))
        }
        termina::event::MouseEventKind::Drag(button) => {
            MouseEventKind::Drag(mouse_button_to_terminput(button))
        }
        termina::event::MouseEventKind::Moved => MouseEventKind::Moved,
        termina::event::MouseEventKind::ScrollDown => MouseEventKind::Scroll(ScrollDirection::Down),
        termina::event::MouseEventKind::ScrollUp => MouseEventKind::Scroll(ScrollDirection::Up),
        termina::event::MouseEventKind::ScrollLeft => MouseEventKind::Scroll(ScrollDirection::Left),
        termina::event::MouseEventKind::ScrollRight => {
            MouseEventKind::Scroll(ScrollDirection::Right)
        }
    }
}

fn mouse_kind_to_termina(
    value: MouseEventKind,
) -> Result<termina::event::MouseEventKind, UnsupportedEvent> {
    Ok(match value {
        MouseEventKind::Down(button) => {
            termina::event::MouseEventKind::Down(mouse_button_to_termina(button)?)
        }
        MouseEventKind::Up(button) => {
            termina::event::MouseEventKind::Up(mouse_button_to_termina(button)?)
        }
        MouseEventKind::Drag(button) => {
            termina::event::MouseEventKind::Drag(mouse_button_to_termina(button)?)
        }
        MouseEventKind::Moved => termina::event::MouseEventKind::Moved,
        MouseEventKind::Scroll(ScrollDirection::Down) => termina::event::MouseEventKind::ScrollDown,
        MouseEventKind::Scroll(ScrollDirection::Up) => termina::event::MouseEventKind::ScrollUp,
        MouseEventKind::Scroll(ScrollDirection::Left) => termina::event::MouseEventKind::ScrollLeft,
        MouseEventKind::Scroll(ScrollDirection::Right) => {
            termina::event::MouseEventKind::ScrollRight
        }
    })
}

fn mouse_button_to_terminput(value: termina::event::MouseButton) -> MouseButton {
    match value {
        termina::event::MouseButton::Left => MouseButton::Left,
        termina::event::MouseButton::Right => MouseButton::Right,
        termina::event::MouseButton::Middle => MouseButton::Middle,
    }
}

fn mouse_button_to_termina(
    value: MouseButton,
) -> Result<termina::event::MouseButton, UnsupportedEvent> {
    Ok(match value {
        MouseButton::Left => termina::event::MouseButton::Left,
        MouseButton::Right => termina::event::MouseButton::Right,
        MouseButton::Middle => termina::event::MouseButton::Middle,
        val @ MouseButton::Unknown => Err(UnsupportedEvent(format!("{val:?}")))?,
    })
}

/// Converts the termina [`KeyEvent`](termina::event::KeyEvent) to a terminput [`KeyEvent`].
pub fn to_terminput_key(value: termina::event::KeyEvent) -> Result<KeyEvent, UnsupportedEvent> {
    Ok(KeyEvent {
        code: key_code_to_terminput(value.code)?,
        modifiers: key_modifiers_to_terminput(value.modifiers),
        kind: key_kind_to_terminput(value.kind),
        state: key_state_to_terminput(value.state),
    })
}

/// Converts the terminput [`KeyEvent`] to a termina [`KeyEvent`](termina::event::KeyEvent).
pub fn to_termina_key(value: KeyEvent) -> termina::event::KeyEvent {
    termina::event::KeyEvent {
        code: convert_termina_key_code(value.code, value.modifiers.intersects(KeyModifiers::SHIFT)),
        modifiers: key_modifiers_to_termina(value.modifiers),
        kind: key_kind_to_termina(value.kind),
        state: key_state_to_termina(value.state),
    }
}

fn key_code_to_terminput(value: termina::event::KeyCode) -> Result<KeyCode, UnsupportedEvent> {
    Ok(match value {
        termina::event::KeyCode::Backspace => KeyCode::Backspace,
        termina::event::KeyCode::Enter => KeyCode::Enter,
        termina::event::KeyCode::Left => KeyCode::Left,
        termina::event::KeyCode::Right => KeyCode::Right,
        termina::event::KeyCode::Up => KeyCode::Up,
        termina::event::KeyCode::Down => KeyCode::Down,
        termina::event::KeyCode::Home => KeyCode::Home,
        termina::event::KeyCode::End => KeyCode::End,
        termina::event::KeyCode::PageUp => KeyCode::PageUp,
        termina::event::KeyCode::PageDown => KeyCode::PageDown,
        termina::event::KeyCode::Tab => KeyCode::Tab,
        termina::event::KeyCode::BackTab => KeyCode::Tab,
        termina::event::KeyCode::Delete => KeyCode::Delete,
        termina::event::KeyCode::Insert => KeyCode::Insert,
        termina::event::KeyCode::Function(f) => KeyCode::F(f),
        termina::event::KeyCode::Char(c) => KeyCode::Char(c),
        termina::event::KeyCode::Escape => KeyCode::Esc,
        termina::event::KeyCode::CapsLock => KeyCode::CapsLock,
        termina::event::KeyCode::ScrollLock => KeyCode::ScrollLock,
        termina::event::KeyCode::NumLock => KeyCode::NumLock,
        termina::event::KeyCode::PrintScreen => KeyCode::PrintScreen,
        termina::event::KeyCode::Pause => KeyCode::Pause,
        termina::event::KeyCode::Menu => KeyCode::Menu,
        termina::event::KeyCode::KeypadBegin => KeyCode::KeypadBegin,
        termina::event::KeyCode::Media(m) => KeyCode::Media(media_code_to_terminput(m)),
        termina::event::KeyCode::Modifier(m) => {
            let (code, direction) = convert_modifier_key_code(m);
            KeyCode::Modifier(code, direction)
        }
        termina::event::KeyCode::Null => Err(UnsupportedEvent(format!("{value:?}")))?,
    })
}

fn convert_termina_key_code(value: KeyCode, shift: bool) -> termina::event::KeyCode {
    match value {
        KeyCode::Backspace => termina::event::KeyCode::Backspace,
        KeyCode::Enter => termina::event::KeyCode::Enter,
        KeyCode::Left => termina::event::KeyCode::Left,
        KeyCode::Right => termina::event::KeyCode::Right,
        KeyCode::Up => termina::event::KeyCode::Up,
        KeyCode::Down => termina::event::KeyCode::Down,
        KeyCode::Home => termina::event::KeyCode::Home,
        KeyCode::End => termina::event::KeyCode::End,
        KeyCode::PageUp => termina::event::KeyCode::PageUp,
        KeyCode::PageDown => termina::event::KeyCode::PageDown,
        KeyCode::Tab if shift => termina::event::KeyCode::BackTab,
        KeyCode::Tab => termina::event::KeyCode::Tab,
        KeyCode::Delete => termina::event::KeyCode::Delete,
        KeyCode::Insert => termina::event::KeyCode::Insert,
        KeyCode::F(f) => termina::event::KeyCode::Function(f),
        KeyCode::Char(c) => termina::event::KeyCode::Char(c),
        KeyCode::Esc => termina::event::KeyCode::Escape,
        KeyCode::CapsLock => termina::event::KeyCode::CapsLock,
        KeyCode::ScrollLock => termina::event::KeyCode::ScrollLock,
        KeyCode::NumLock => termina::event::KeyCode::NumLock,
        KeyCode::PrintScreen => termina::event::KeyCode::PrintScreen,
        KeyCode::Pause => termina::event::KeyCode::Pause,
        KeyCode::Menu => termina::event::KeyCode::Menu,
        KeyCode::KeypadBegin => termina::event::KeyCode::KeypadBegin,
        KeyCode::Media(m) => termina::event::KeyCode::Media(media_code_to_termina(m)),
        KeyCode::Modifier(code, direction) => {
            termina::event::KeyCode::Modifier(convert_termina_modifier_key_code(code, direction))
        }
    }
}

fn key_modifiers_to_terminput(value: termina::event::Modifiers) -> KeyModifiers {
    let mut res = KeyModifiers::empty();
    if value.intersects(termina::event::Modifiers::ALT) {
        res |= KeyModifiers::ALT;
    }
    if value.intersects(termina::event::Modifiers::SHIFT) {
        res |= KeyModifiers::SHIFT;
    }
    if value.intersects(termina::event::Modifiers::CONTROL) {
        res |= KeyModifiers::CTRL;
    }
    if value.intersects(termina::event::Modifiers::SUPER) {
        res |= KeyModifiers::SUPER;
    }

    res
}

fn key_modifiers_to_termina(value: KeyModifiers) -> termina::event::Modifiers {
    let mut res = termina::event::Modifiers::empty();
    if value.intersects(KeyModifiers::ALT) {
        res |= termina::event::Modifiers::ALT;
    }
    if value.intersects(KeyModifiers::SHIFT) {
        res |= termina::event::Modifiers::SHIFT;
    }
    if value.intersects(KeyModifiers::CTRL) {
        res |= termina::event::Modifiers::CONTROL;
    }
    if value.intersects(KeyModifiers::SUPER) {
        res |= termina::event::Modifiers::SUPER;
    }

    res
}

fn key_kind_to_terminput(value: termina::event::KeyEventKind) -> KeyEventKind {
    match value {
        termina::event::KeyEventKind::Press => KeyEventKind::Press,
        termina::event::KeyEventKind::Repeat => KeyEventKind::Repeat,
        termina::event::KeyEventKind::Release => KeyEventKind::Release,
    }
}

fn key_kind_to_termina(value: KeyEventKind) -> termina::event::KeyEventKind {
    match value {
        KeyEventKind::Press => termina::event::KeyEventKind::Press,
        KeyEventKind::Repeat => termina::event::KeyEventKind::Repeat,
        KeyEventKind::Release => termina::event::KeyEventKind::Release,
    }
}

fn media_code_to_terminput(value: termina::event::MediaKeyCode) -> MediaKeyCode {
    match value {
        termina::event::MediaKeyCode::Play => MediaKeyCode::Play,
        termina::event::MediaKeyCode::Pause => MediaKeyCode::Pause,
        termina::event::MediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
        termina::event::MediaKeyCode::Reverse => MediaKeyCode::Reverse,
        termina::event::MediaKeyCode::Stop => MediaKeyCode::Stop,
        termina::event::MediaKeyCode::FastForward => MediaKeyCode::FastForward,
        termina::event::MediaKeyCode::Rewind => MediaKeyCode::Rewind,
        termina::event::MediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
        termina::event::MediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
        termina::event::MediaKeyCode::Record => MediaKeyCode::Record,
        termina::event::MediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
        termina::event::MediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
        termina::event::MediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume,
    }
}

fn media_code_to_termina(value: MediaKeyCode) -> termina::event::MediaKeyCode {
    match value {
        MediaKeyCode::Play => termina::event::MediaKeyCode::Play,
        MediaKeyCode::Pause => termina::event::MediaKeyCode::Pause,
        MediaKeyCode::PlayPause => termina::event::MediaKeyCode::PlayPause,
        MediaKeyCode::Reverse => termina::event::MediaKeyCode::Reverse,
        MediaKeyCode::Stop => termina::event::MediaKeyCode::Stop,
        MediaKeyCode::FastForward => termina::event::MediaKeyCode::FastForward,
        MediaKeyCode::Rewind => termina::event::MediaKeyCode::Rewind,
        MediaKeyCode::TrackNext => termina::event::MediaKeyCode::TrackNext,
        MediaKeyCode::TrackPrevious => termina::event::MediaKeyCode::TrackPrevious,
        MediaKeyCode::Record => termina::event::MediaKeyCode::Record,
        MediaKeyCode::LowerVolume => termina::event::MediaKeyCode::LowerVolume,
        MediaKeyCode::RaiseVolume => termina::event::MediaKeyCode::RaiseVolume,
        MediaKeyCode::MuteVolume => termina::event::MediaKeyCode::MuteVolume,
    }
}

fn convert_modifier_key_code(
    value: termina::event::ModifierKeyCode,
) -> (ModifierKeyCode, ModifierDirection) {
    match value {
        termina::event::ModifierKeyCode::LeftShift => {
            (ModifierKeyCode::Shift, ModifierDirection::Left)
        }
        termina::event::ModifierKeyCode::LeftControl => {
            (ModifierKeyCode::Control, ModifierDirection::Left)
        }
        termina::event::ModifierKeyCode::LeftAlt => (ModifierKeyCode::Alt, ModifierDirection::Left),
        termina::event::ModifierKeyCode::LeftSuper => {
            (ModifierKeyCode::Super, ModifierDirection::Left)
        }
        termina::event::ModifierKeyCode::LeftHyper => {
            (ModifierKeyCode::Hyper, ModifierDirection::Left)
        }
        termina::event::ModifierKeyCode::LeftMeta => {
            (ModifierKeyCode::Meta, ModifierDirection::Left)
        }
        termina::event::ModifierKeyCode::RightShift => {
            (ModifierKeyCode::Shift, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::RightControl => {
            (ModifierKeyCode::Control, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::RightAlt => {
            (ModifierKeyCode::Alt, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::RightSuper => {
            (ModifierKeyCode::Super, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::RightHyper => {
            (ModifierKeyCode::Hyper, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::RightMeta => {
            (ModifierKeyCode::Meta, ModifierDirection::Right)
        }
        termina::event::ModifierKeyCode::IsoLevel3Shift => {
            (ModifierKeyCode::IsoLevel3Shift, ModifierDirection::Unknown)
        }
        termina::event::ModifierKeyCode::IsoLevel5Shift => {
            (ModifierKeyCode::IsoLevel5Shift, ModifierDirection::Unknown)
        }
    }
}

fn convert_termina_modifier_key_code(
    code: ModifierKeyCode,
    direction: ModifierDirection,
) -> termina::event::ModifierKeyCode {
    match (code, direction) {
        (ModifierKeyCode::Shift, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftShift
        }
        (ModifierKeyCode::Control, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftControl
        }
        (ModifierKeyCode::Alt, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftAlt
        }
        (ModifierKeyCode::Super, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftSuper
        }
        (ModifierKeyCode::Hyper, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftHyper
        }
        (ModifierKeyCode::Meta, ModifierDirection::Left | ModifierDirection::Unknown) => {
            termina::event::ModifierKeyCode::LeftMeta
        }
        (ModifierKeyCode::Shift, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightShift
        }
        (ModifierKeyCode::Control, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightControl
        }
        (ModifierKeyCode::Alt, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightAlt
        }
        (ModifierKeyCode::Super, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightSuper
        }
        (ModifierKeyCode::Hyper, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightHyper
        }
        (ModifierKeyCode::Meta, ModifierDirection::Right) => {
            termina::event::ModifierKeyCode::RightMeta
        }
        (ModifierKeyCode::IsoLevel3Shift, _) => termina::event::ModifierKeyCode::IsoLevel3Shift,
        (ModifierKeyCode::IsoLevel5Shift, _) => termina::event::ModifierKeyCode::IsoLevel5Shift,
    }
}

fn key_state_to_terminput(value: termina::event::KeyEventState) -> KeyEventState {
    let mut state = KeyEventState::empty();
    if value.intersects(termina::event::KeyEventState::KEYPAD) {
        state |= KeyEventState::KEYPAD;
    }
    if value.intersects(termina::event::KeyEventState::CAPS_LOCK) {
        state |= KeyEventState::CAPS_LOCK;
    }
    if value.intersects(termina::event::KeyEventState::NUM_LOCK) {
        state |= KeyEventState::NUM_LOCK;
    }
    state
}

fn key_state_to_termina(value: KeyEventState) -> termina::event::KeyEventState {
    let mut state = termina::event::KeyEventState::empty();
    if value.intersects(KeyEventState::KEYPAD) {
        state |= termina::event::KeyEventState::KEYPAD;
    }
    if value.intersects(KeyEventState::CAPS_LOCK) {
        state |= termina::event::KeyEventState::CAPS_LOCK;
    }
    if value.intersects(KeyEventState::NUM_LOCK) {
        state |= termina::event::KeyEventState::NUM_LOCK;
    }
    state
}
