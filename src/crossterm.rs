use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, UnsupportedEvent,
};

impl TryFrom<crossterm::event::Event> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::Event) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::Event::FocusGained => Event::FocusGained,
            crossterm::event::Event::FocusLost => Event::FocusLost,
            crossterm::event::Event::Key(key_event) => Event::Key(key_event.try_into()?),
            crossterm::event::Event::Mouse(mouse_event) => Event::Mouse(mouse_event.try_into()?),
            crossterm::event::Event::Paste(value) => Event::Paste(value),
            crossterm::event::Event::Resize(cols, rows) => Event::Resize(cols, rows),
        })
    }
}

impl TryFrom<Event> for crossterm::event::Event {
    type Error = UnsupportedEvent;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::FocusGained => crossterm::event::Event::FocusGained,
            Event::FocusLost => crossterm::event::Event::FocusLost,
            Event::Key(key_event) => crossterm::event::Event::Key(key_event.try_into()?),
            Event::Mouse(mouse_event) => crossterm::event::Event::Mouse(mouse_event.try_into()?),
            Event::Paste(value) => crossterm::event::Event::Paste(value),
            Event::Resize(cols, rows) => crossterm::event::Event::Resize(cols, rows),
        })
    }
}

impl TryFrom<crossterm::event::MouseEvent> for MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MouseEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: value.kind.try_into()?,
            column: value.column,
            row: value.row,
            modifiers: value.modifiers.try_into()?,
        })
    }
}

impl TryFrom<MouseEvent> for crossterm::event::MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            kind: value.kind.try_into()?,
            column: value.column,
            row: value.row,
            modifiers: value.modifiers.try_into()?,
        })
    }
}

impl TryFrom<crossterm::event::MouseEventKind> for MouseEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MouseEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::MouseEventKind::Down(button) => {
                MouseEventKind::Down(button.try_into()?)
            }
            crossterm::event::MouseEventKind::Up(button) => MouseEventKind::Up(button.try_into()?),
            crossterm::event::MouseEventKind::Drag(button) => {
                MouseEventKind::Drag(button.try_into()?)
            }
            crossterm::event::MouseEventKind::Moved => MouseEventKind::Moved,
            crossterm::event::MouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            crossterm::event::MouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
            crossterm::event::MouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            crossterm::event::MouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
        })
    }
}

impl TryFrom<MouseEventKind> for crossterm::event::MouseEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            MouseEventKind::Down(button) => {
                crossterm::event::MouseEventKind::Down(button.try_into()?)
            }
            MouseEventKind::Up(button) => crossterm::event::MouseEventKind::Up(button.try_into()?),
            MouseEventKind::Drag(button) => {
                crossterm::event::MouseEventKind::Drag(button.try_into()?)
            }
            MouseEventKind::Moved => crossterm::event::MouseEventKind::Moved,
            MouseEventKind::ScrollDown => crossterm::event::MouseEventKind::ScrollDown,
            MouseEventKind::ScrollUp => crossterm::event::MouseEventKind::ScrollUp,
            MouseEventKind::ScrollLeft => crossterm::event::MouseEventKind::ScrollLeft,
            MouseEventKind::ScrollRight => crossterm::event::MouseEventKind::ScrollRight,
        })
    }
}

impl TryFrom<crossterm::event::MouseButton> for MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MouseButton) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::MouseButton::Left => MouseButton::Left,
            crossterm::event::MouseButton::Right => MouseButton::Right,
            crossterm::event::MouseButton::Middle => MouseButton::Middle,
        })
    }
}

impl TryFrom<MouseButton> for crossterm::event::MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseButton) -> Result<Self, Self::Error> {
        Ok(match value {
            MouseButton::Left => crossterm::event::MouseButton::Left,
            MouseButton::Right => crossterm::event::MouseButton::Right,
            MouseButton::Middle => crossterm::event::MouseButton::Middle,
            val @ MouseButton::Unknown => Err(UnsupportedEvent(format!("{val:?}")))?,
        })
    }
}

impl TryFrom<crossterm::event::KeyEvent> for KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            code: value.code.try_into()?,
            modifiers: value.modifiers.try_into()?,
            kind: value.kind.try_into()?,
            state: value.state.try_into()?,
        })
    }
}

impl TryFrom<KeyEvent> for crossterm::event::KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        Ok(Self {
            code: convert_crossterm_key_code(
                value.code,
                value.modifiers.intersects(KeyModifiers::SHIFT),
            )?,
            modifiers: value.modifiers.try_into()?,
            kind: value.kind.try_into()?,
            state: value.state.try_into()?,
        })
    }
}

impl TryFrom<crossterm::event::KeyCode> for KeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyCode) -> Result<Self, Self::Error> {
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
            crossterm::event::KeyCode::Media(m) => KeyCode::Media(m.try_into()?),
            crossterm::event::KeyCode::Modifier(m) => {
                let (code, direction) = convert_modifier_key_code(m);
                KeyCode::Modifier(code, direction)
            }
            crossterm::event::KeyCode::Null => Err(UnsupportedEvent(format!("{value:?}")))?,
        })
    }
}

fn convert_crossterm_key_code(
    value: KeyCode,
    shift: bool,
) -> Result<crossterm::event::KeyCode, UnsupportedEvent> {
    Ok(match value {
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
        KeyCode::Media(m) => crossterm::event::KeyCode::Media(m.try_into()?),
        KeyCode::Modifier(code, direction) => crossterm::event::KeyCode::Modifier(
            convert_crossterm_modifier_key_code(code, direction),
        ),
    })
}
impl TryFrom<crossterm::event::KeyModifiers> for KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyModifiers) -> Result<Self, Self::Error> {
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

        Ok(res)
    }
}

impl TryFrom<KeyModifiers> for crossterm::event::KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyModifiers) -> Result<Self, Self::Error> {
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

        Ok(res)
    }
}

impl TryFrom<crossterm::event::KeyEventKind> for KeyEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::KeyEventKind::Press => KeyEventKind::Press,
            crossterm::event::KeyEventKind::Repeat => KeyEventKind::Repeat,
            crossterm::event::KeyEventKind::Release => KeyEventKind::Release,
        })
    }
}

impl TryFrom<KeyEventKind> for crossterm::event::KeyEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyEventKind::Press => crossterm::event::KeyEventKind::Press,
            KeyEventKind::Repeat => crossterm::event::KeyEventKind::Repeat,
            KeyEventKind::Release => crossterm::event::KeyEventKind::Release,
        })
    }
}

impl TryFrom<crossterm::event::MediaKeyCode> for MediaKeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MediaKeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
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
        })
    }
}

impl TryFrom<MediaKeyCode> for crossterm::event::MediaKeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: MediaKeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
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
        })
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

impl TryFrom<crossterm::event::KeyEventState> for KeyEventState {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyEventState) -> Result<Self, Self::Error> {
        Ok(Self::from_bits_retain(value.bits()))
    }
}

impl TryFrom<KeyEventState> for crossterm::event::KeyEventState {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEventState) -> Result<Self, Self::Error> {
        Ok(Self::from_bits_retain(value.bits()))
    }
}
