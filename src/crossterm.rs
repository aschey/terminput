use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, UnsupportedEvent,
};

impl TryFrom<crossterm::event::Event> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::Event) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::Event::FocusGained => Self::FocusGained,
            crossterm::event::Event::FocusLost => Self::FocusLost,
            crossterm::event::Event::Key(key_event) => Self::Key(key_event.try_into()?),
            crossterm::event::Event::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            crossterm::event::Event::Paste(value) => Self::Paste(value),
            crossterm::event::Event::Resize(cols, rows) => Self::Resize(cols, rows),
        })
    }
}

impl TryFrom<Event> for crossterm::event::Event {
    type Error = UnsupportedEvent;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::FocusGained => Self::FocusGained,
            Event::FocusLost => Self::FocusLost,
            Event::Key(key_event) => Self::Key(key_event.try_into()?),
            Event::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            Event::Paste(value) => Self::Paste(value),
            Event::Resize(cols, rows) => Self::Resize(cols, rows),
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
            crossterm::event::MouseEventKind::Down(button) => Self::Down(button.try_into()?),
            crossterm::event::MouseEventKind::Up(button) => Self::Up(button.try_into()?),
            crossterm::event::MouseEventKind::Drag(button) => Self::Drag(button.try_into()?),
            crossterm::event::MouseEventKind::Moved => Self::Moved,
            crossterm::event::MouseEventKind::ScrollDown => Self::ScrollDown,
            crossterm::event::MouseEventKind::ScrollUp => Self::ScrollUp,
            crossterm::event::MouseEventKind::ScrollLeft => Self::ScrollLeft,
            crossterm::event::MouseEventKind::ScrollRight => Self::ScrollRight,
        })
    }
}

impl TryFrom<MouseEventKind> for crossterm::event::MouseEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            MouseEventKind::Down(button) => Self::Down(button.try_into()?),
            MouseEventKind::Up(button) => Self::Up(button.try_into()?),
            MouseEventKind::Drag(button) => Self::Drag(button.try_into()?),
            MouseEventKind::Moved => Self::Moved,
            MouseEventKind::ScrollDown => Self::ScrollDown,
            MouseEventKind::ScrollUp => Self::ScrollUp,
            MouseEventKind::ScrollLeft => Self::ScrollLeft,
            MouseEventKind::ScrollRight => Self::ScrollRight,
        })
    }
}

impl TryFrom<crossterm::event::MouseButton> for MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MouseButton) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::MouseButton::Left => Self::Left,
            crossterm::event::MouseButton::Right => Self::Right,
            crossterm::event::MouseButton::Middle => Self::Middle,
        })
    }
}

impl TryFrom<MouseButton> for crossterm::event::MouseButton {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseButton) -> Result<Self, Self::Error> {
        Ok(match value {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
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
            crossterm::event::KeyCode::Backspace => Self::Backspace,
            crossterm::event::KeyCode::Enter => Self::Enter,
            crossterm::event::KeyCode::Left => Self::Left,
            crossterm::event::KeyCode::Right => Self::Right,
            crossterm::event::KeyCode::Up => Self::Up,
            crossterm::event::KeyCode::Down => Self::Down,
            crossterm::event::KeyCode::Home => Self::Home,
            crossterm::event::KeyCode::End => Self::End,
            crossterm::event::KeyCode::PageUp => Self::PageUp,
            crossterm::event::KeyCode::PageDown => Self::PageDown,
            crossterm::event::KeyCode::Tab => Self::Tab,
            crossterm::event::KeyCode::BackTab => Self::Tab,
            crossterm::event::KeyCode::Delete => Self::Delete,
            crossterm::event::KeyCode::Insert => Self::Insert,
            crossterm::event::KeyCode::F(f) => Self::F(f),
            crossterm::event::KeyCode::Char(c) => Self::Char(c),
            crossterm::event::KeyCode::Esc => Self::Esc,
            crossterm::event::KeyCode::CapsLock => Self::CapsLock,
            crossterm::event::KeyCode::ScrollLock => Self::ScrollLock,
            crossterm::event::KeyCode::NumLock => Self::NumLock,
            crossterm::event::KeyCode::PrintScreen => Self::PrintScreen,
            crossterm::event::KeyCode::Pause => Self::Pause,
            crossterm::event::KeyCode::Menu => Self::Menu,
            crossterm::event::KeyCode::KeypadBegin => Self::KeypadBegin,
            crossterm::event::KeyCode::Media(m) => Self::Media(m.try_into()?),
            crossterm::event::KeyCode::Modifier(m) => {
                let (code, direction) = convert_modifier_key_code(m);
                Self::Modifier(code, direction)
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
        let mut res = Self::empty();
        if value.intersects(crossterm::event::KeyModifiers::ALT) {
            res |= Self::ALT;
        }
        if value.intersects(crossterm::event::KeyModifiers::SHIFT) {
            res |= Self::SHIFT;
        }
        if value.intersects(crossterm::event::KeyModifiers::CONTROL) {
            res |= Self::CTRL;
        }
        if value.intersects(crossterm::event::KeyModifiers::SUPER) {
            res |= Self::SUPER;
        }

        Ok(res)
    }
}

impl TryFrom<KeyModifiers> for crossterm::event::KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyModifiers) -> Result<Self, Self::Error> {
        let mut res = Self::empty();
        if value.intersects(KeyModifiers::ALT) {
            res |= Self::ALT;
        }
        if value.intersects(KeyModifiers::SHIFT) {
            res |= Self::SHIFT;
        }
        if value.intersects(KeyModifiers::CTRL) {
            res |= Self::CONTROL;
        }
        if value.intersects(KeyModifiers::SUPER) {
            res |= Self::SUPER;
        }

        Ok(res)
    }
}

impl TryFrom<crossterm::event::KeyEventKind> for KeyEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::KeyEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::KeyEventKind::Press => Self::Press,
            crossterm::event::KeyEventKind::Repeat => Self::Repeat,
            crossterm::event::KeyEventKind::Release => Self::Release,
        })
    }
}

impl TryFrom<KeyEventKind> for crossterm::event::KeyEventKind {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEventKind) -> Result<Self, Self::Error> {
        Ok(match value {
            KeyEventKind::Press => Self::Press,
            KeyEventKind::Repeat => Self::Repeat,
            KeyEventKind::Release => Self::Release,
        })
    }
}

impl TryFrom<crossterm::event::MediaKeyCode> for MediaKeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: crossterm::event::MediaKeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            crossterm::event::MediaKeyCode::Play => Self::Play,
            crossterm::event::MediaKeyCode::Pause => Self::Pause,
            crossterm::event::MediaKeyCode::PlayPause => Self::PlayPause,
            crossterm::event::MediaKeyCode::Reverse => Self::Reverse,
            crossterm::event::MediaKeyCode::Stop => Self::Stop,
            crossterm::event::MediaKeyCode::FastForward => Self::FastForward,
            crossterm::event::MediaKeyCode::Rewind => Self::Rewind,
            crossterm::event::MediaKeyCode::TrackNext => Self::TrackNext,
            crossterm::event::MediaKeyCode::TrackPrevious => Self::TrackPrevious,
            crossterm::event::MediaKeyCode::Record => Self::Record,
            crossterm::event::MediaKeyCode::LowerVolume => Self::LowerVolume,
            crossterm::event::MediaKeyCode::RaiseVolume => Self::RaiseVolume,
            crossterm::event::MediaKeyCode::MuteVolume => Self::MuteVolume,
        })
    }
}

impl TryFrom<MediaKeyCode> for crossterm::event::MediaKeyCode {
    type Error = UnsupportedEvent;

    fn try_from(value: MediaKeyCode) -> Result<Self, Self::Error> {
        Ok(match value {
            MediaKeyCode::Play => Self::Play,
            MediaKeyCode::Pause => Self::Pause,
            MediaKeyCode::PlayPause => Self::PlayPause,
            MediaKeyCode::Reverse => Self::Reverse,
            MediaKeyCode::Stop => Self::Stop,
            MediaKeyCode::FastForward => Self::FastForward,
            MediaKeyCode::Rewind => Self::Rewind,
            MediaKeyCode::TrackNext => Self::TrackNext,
            MediaKeyCode::TrackPrevious => Self::TrackPrevious,
            MediaKeyCode::Record => Self::Record,
            MediaKeyCode::LowerVolume => Self::LowerVolume,
            MediaKeyCode::RaiseVolume => Self::RaiseVolume,
            MediaKeyCode::MuteVolume => Self::MuteVolume,
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
