use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, UnsupportedEvent,
};

impl TryFrom<termwiz::input::InputEvent> for Event {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::InputEvent) -> Result<Self, UnsupportedEvent> {
        Ok(match value {
            termwiz::input::InputEvent::Key(key_event) => Event::Key(key_event.try_into()?),
            termwiz::input::InputEvent::Mouse(mouse_event) => Event::Mouse(mouse_event.try_into()?),
            val @ termwiz::input::InputEvent::PixelMouse(_) => {
                Err(UnsupportedEvent(format!("{val:?}")))?
            }
            termwiz::input::InputEvent::Resized { cols, rows } => {
                Event::Resize(cols as u16, rows as u16)
            }
            termwiz::input::InputEvent::Paste(val) => Event::Paste(val),
            termwiz::input::InputEvent::Wake => Event::FocusGained,
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
            val => Err(UnsupportedEvent(format!("{val:?}")))?,
        };
        Ok(Self {
            code,
            modifiers: value.modifiers.try_into()?,
            kind: KeyEventKind::Press,
            state,
        })
    }
}

impl TryFrom<termwiz::input::Modifiers> for KeyModifiers {
    type Error = UnsupportedEvent;

    fn try_from(value: termwiz::input::Modifiers) -> Result<Self, Self::Error> {
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
            return Ok(MouseEvent {
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
            return Ok(MouseEvent {
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
            return Ok(MouseEvent {
                kind: MouseEventKind::Down(MouseButton::Middle),
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }

        if value.mouse_buttons.contains(
            termwiz::input::MouseButtons::VERT_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
        ) {
            return Ok(MouseEvent {
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
            return Ok(MouseEvent {
                kind: MouseEventKind::ScrollDown,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value.mouse_buttons.contains(
            termwiz::input::MouseButtons::HORZ_WHEEL | termwiz::input::MouseButtons::WHEEL_POSITIVE,
        ) {
            return Ok(MouseEvent {
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
            return Ok(MouseEvent {
                kind: MouseEventKind::ScrollRight,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }
        if value.mouse_buttons == termwiz::input::MouseButtons::NONE {
            return Ok(MouseEvent {
                kind: MouseEventKind::Moved,
                column: value.x - 1,
                row: value.y - 1,
                modifiers: value.modifiers.try_into()?,
            });
        }

        Err(UnsupportedEvent(format!("{value:?}")))
    }
}
