use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, UnsupportedEvent,
};

impl TryFrom<termwiz::input::InputEvent> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::InputEvent) -> Result<Self, UnsupportedEvent> {
        Ok(match value {
            termwiz::input::InputEvent::Key(key_event) => Self::Key(key_event.try_into()?),
            termwiz::input::InputEvent::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            termwiz::input::InputEvent::Resized { cols, rows } => {
                Self::Resize(cols as u16, rows as u16)
            }
            termwiz::input::InputEvent::Paste(val) => Self::Paste(val),
            termwiz::input::InputEvent::PixelMouse(_) | termwiz::input::InputEvent::Wake => {
                Err(UnsupportedEvent(format!("{value:?}")))?
            }
        })
    }
}

impl TryFrom<Event> for termwiz::input::InputEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: Event) -> Result<Self, Self::Error> {
        Ok(match value {
            Event::Key(key_event) => Self::Key(key_event.try_into()?),
            Event::Mouse(mouse_event) => Self::Mouse(mouse_event.try_into()?),
            Event::Paste(val) => Self::Paste(val),
            Event::Resize(_, _) => todo!(),
            Event::FocusGained | Event::FocusLost => Err(UnsupportedEvent(format!("{value:?}")))?,
        })
    }
}

impl TryFrom<termwiz::input::KeyEvent> for KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::KeyEvent) -> Result<Self, Self::Error> {
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
            termwiz::input::KeyCode::ApplicationLeftArrow => {
                (KeyCode::Left, KeyEventState::empty())
            }
            termwiz::input::KeyCode::ApplicationRightArrow => {
                (KeyCode::Right, KeyEventState::empty())
            }
            termwiz::input::KeyCode::ApplicationUpArrow => (KeyCode::Up, KeyEventState::empty()),
            termwiz::input::KeyCode::ApplicationDownArrow => {
                (KeyCode::Down, KeyEventState::empty())
            }
            termwiz::input::KeyCode::KeyPadHome => (KeyCode::Home, KeyEventState::KEYPAD),
            termwiz::input::KeyCode::KeyPadEnd => (KeyCode::End, KeyEventState::KEYPAD),
            termwiz::input::KeyCode::KeyPadPageUp => (KeyCode::PageUp, KeyEventState::KEYPAD),
            termwiz::input::KeyCode::KeyPadPageDown => (KeyCode::PageDown, KeyEventState::KEYPAD),
            termwiz::input::KeyCode::KeyPadBegin => (KeyCode::KeypadBegin, KeyEventState::KEYPAD),
            _ => Err(UnsupportedEvent(format!("{value:?}")))?,
        };
        Ok(Self {
            code,
            modifiers: value.modifiers.try_into()?,
            kind: KeyEventKind::Press,
            state,
        })
    }
}

impl TryFrom<KeyEvent> for termwiz::input::KeyEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
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
            KeyCode::Modifier(
                ModifierKeyCode::IsoLevel3Shift | ModifierKeyCode::IsoLevel5Shift,
                _,
            ) => {
                return Err(UnsupportedEvent(format!("{value:?}")));
            }
        };

        Ok(Self {
            key,
            modifiers: value.modifiers.try_into()?,
        })
    }
}

impl TryFrom<termwiz::input::Modifiers> for KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::Modifiers) -> Result<Self, Self::Error> {
        let mut res = Self::empty();
        if value.intersects(
            termwiz::input::Modifiers::ALT
                | termwiz::input::Modifiers::LEFT_ALT
                | termwiz::input::Modifiers::RIGHT_ALT,
        ) {
            res |= Self::ALT;
        }
        if value.intersects(
            termwiz::input::Modifiers::SHIFT
                | termwiz::input::Modifiers::LEFT_SHIFT
                | termwiz::input::Modifiers::RIGHT_SHIFT,
        ) {
            res |= Self::SHIFT;
        }

        if value.intersects(
            termwiz::input::Modifiers::CTRL
                | termwiz::input::Modifiers::LEFT_CTRL
                | termwiz::input::Modifiers::RIGHT_CTRL,
        ) {
            res |= Self::CTRL;
        }
        if value.intersects(termwiz::input::Modifiers::SUPER) {
            res |= Self::SUPER;
        }

        Ok(res)
    }
}

impl TryFrom<KeyModifiers> for termwiz::input::Modifiers {
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
            res |= Self::CTRL;
        }
        if value.intersects(KeyModifiers::SUPER) {
            res |= Self::SUPER;
        }

        Ok(res)
    }
}

impl TryFrom<termwiz::input::MouseEvent> for MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::MouseEvent) -> Result<Self, Self::Error> {
        if value
            .mouse_buttons
            .contains(termwiz::input::MouseButtons::LEFT)
        {
            return Ok(Self {
                kind: MouseEventKind::Down(MouseButton::Left),
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value
            .mouse_buttons
            .contains(termwiz::input::MouseButtons::RIGHT)
        {
            return Ok(Self {
                kind: MouseEventKind::Down(MouseButton::Right),
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value
            .mouse_buttons
            .contains(termwiz::input::MouseButtons::MIDDLE)
        {
            return Ok(Self {
                kind: MouseEventKind::Down(MouseButton::Middle),
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }

        if value.mouse_buttons.contains(
            termwiz::input::MouseButtons::VERT_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
        ) {
            return Ok(Self {
                kind: MouseEventKind::ScrollUp,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value
            .mouse_buttons
            .contains(termwiz::input::MouseButtons::VERT_WHEEL)
        {
            return Ok(Self {
                kind: MouseEventKind::ScrollDown,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value.mouse_buttons.contains(
            termwiz::input::MouseButtons::HORZ_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
        ) {
            return Ok(Self {
                kind: MouseEventKind::ScrollLeft,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value
            .mouse_buttons
            .contains(termwiz::input::MouseButtons::HORZ_WHEEL)
        {
            return Ok(Self {
                kind: MouseEventKind::ScrollRight,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value.mouse_buttons == termwiz::input::MouseButtons::NONE {
            return Ok(Self {
                kind: MouseEventKind::Moved,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }

        Err(UnsupportedEvent(format!("{value:?}")))
    }
}

impl TryFrom<MouseEvent> for termwiz::input::MouseEvent {
    type Error = UnsupportedEvent;

    fn try_from(value: MouseEvent) -> Result<Self, Self::Error> {
        Ok(match value.kind {
            MouseEventKind::Down(MouseButton::Left | MouseButton::Unknown) => Self {
                mouse_buttons: termwiz::input::MouseButtons::LEFT,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::Down(MouseButton::Right) => Self {
                mouse_buttons: termwiz::input::MouseButtons::RIGHT,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::Down(MouseButton::Middle) => Self {
                mouse_buttons: termwiz::input::MouseButtons::MIDDLE,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::Up(_) | MouseEventKind::Drag(_) => {
                Err(UnsupportedEvent(format!("{value:?}")))?
            }
            MouseEventKind::Moved => Self {
                mouse_buttons: termwiz::input::MouseButtons::NONE,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::ScrollDown => Self {
                mouse_buttons: termwiz::input::MouseButtons::VERT_WHEEL,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::ScrollUp => Self {
                mouse_buttons: termwiz::input::MouseButtons::VERT_WHEEL
                    | termwiz::input::MouseButtons::WHEEL_POSITIVE,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::ScrollLeft => Self {
                mouse_buttons: termwiz::input::MouseButtons::HORZ_WHEEL,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
            MouseEventKind::ScrollRight => Self {
                mouse_buttons: termwiz::input::MouseButtons::HORZ_WHEEL
                    | termwiz::input::MouseButtons::WHEEL_POSITIVE,
                x: value.column + 1,
                y: value.row + 1,
                modifiers: value.modifiers.try_into()?,
            },
        })
    }
}
