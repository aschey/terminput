use std::io::{self, Cursor, Seek, Write};

use bitflags::bitflags;

use crate::{
    Event, KeyCode, KeyEventKind, KeyModifiers, MediaKeyCode, ModifierDirection, ModifierKeyCode,
    MouseButton, MouseEventKind,
};

bitflags! {
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KeyboardEnhancementFlags: u8 {
        /// Represent Escape and modified keys using CSI-u sequences, so they can be unambiguously
        /// read.
        const DISAMBIGUATE_ESCAPE_CODES = 1<<1;
        /// Add extra events with [`KeyEvent.kind`] set to [`KeyEventKind::Repeat`] or
        /// [`KeyEventKind::Release`] when keys are autorepeated or released.
        const REPORT_EVENT_TYPES = 1<<2;
        /// Send [alternate keycodes](https://sw.kovidgoyal.net/kitty/keyboard-protocol/#key-codes)
        /// in addition to the base keycode. The alternate keycode overrides the base keycode in
        /// resulting `KeyEvent`s.
        const REPORT_ALTERNATE_KEYS = 1<<3;
        /// Represent all keyboard events as CSI-u sequences. This is required to get repeat/release
        /// events for plain-text keys.
        const REPORT_ALL_KEYS_AS_ESCAPE_CODES = 1<<4;

    }
}

impl Event {
    pub fn to_escape_sequence(&self, buf: &mut [u8]) -> io::Result<usize> {
        let mut buf = Cursor::new(buf);
        match self {
            Event::FocusGained => {
                buf.write_all(b"\x1B[I")?;
                Ok(buf.position() as usize)
            }
            Event::FocusLost => {
                buf.write_all(b"\x1B[O")?;
                Ok(buf.position() as usize)
            }
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "Only keypress events can be encoded.",
                    ));
                }

                if key_event.modifiers.intersects(KeyModifiers::ALT) {
                    match key_event.code {
                        KeyCode::Char(_)
                        | KeyCode::Esc
                        | KeyCode::Backspace
                        | KeyCode::Enter
                        | KeyCode::Tab
                        | KeyCode::BackTab => {
                            buf.write_all(b"\x1B")?;
                        }
                        _ => {}
                    }
                }
                match key_event.code {
                    KeyCode::F(1..=4) => {
                        buf.write_all(b"\x1B")?;
                        self.keycode_suffix(key_event.code, &mut buf)?;
                    }
                    KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Home
                    | KeyCode::End
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::BackTab
                    | KeyCode::Delete
                    | KeyCode::Insert
                    | KeyCode::F(_) => {
                        buf.write_all(b"\x1B[")?;
                        self.keycode_suffix(key_event.code, &mut buf)?;
                    }
                    _ => {
                        let handled = self.keycode_suffix(key_event.code, &mut buf)?;
                        if !handled {
                            return Err(io::Error::new(
                                io::ErrorKind::Unsupported,
                                "unsupported key",
                            ));
                        }
                    }
                }

                if !key_event.modifiers.is_empty() {
                    match key_event.code {
                        KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Home
                        | KeyCode::End => {
                            let pos = buf.position() as usize;
                            let last = buf.get_mut()[pos - 1];
                            buf.seek_relative(-1)?;
                            buf.write_all(b"1;1")?;
                            buf.write_all(&[last])?;
                        }
                        KeyCode::F(1..=4) => {
                            let pos = buf.position() as usize;
                            let last = buf.get_ref()[pos - 1];
                            buf.seek_relative(-2)?;
                            buf.write_all(b"[1;1")?;
                            buf.write_all(&[last])?;
                        }
                        KeyCode::PageUp
                        | KeyCode::PageDown
                        | KeyCode::Delete
                        | KeyCode::Insert
                        | KeyCode::F(_) => {
                            let pos = buf.position() as usize;
                            let last = buf.get_ref()[pos - 1];
                            buf.seek_relative(-1)?;
                            buf.write_all(b";1")?;
                            buf.write_all(&[last])?;
                        }
                        _ => {}
                    }
                }

                if key_event.modifiers.intersects(KeyModifiers::CTRL) {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            let pos = buf.position() as usize;
                            buf.get_mut()[pos - 1] = (c as u8) + 0x1 - b'a';
                        }
                        KeyCode::Backspace => {
                            let pos = buf.position() as usize;
                            buf.get_mut()[pos - 1] = b'\x08';
                        }
                        _ => {}
                    }
                }
                match key_event.code {
                    KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Home
                    | KeyCode::End
                    | KeyCode::PageUp
                    | KeyCode::PageDown
                    | KeyCode::Delete
                    | KeyCode::Insert
                    | KeyCode::F(1..=4) => {
                        buf.get_mut()[4] += key_event.modifiers.bits();
                    }
                    KeyCode::F(_) => {
                        buf.get_mut()[5] += key_event.modifiers.bits();
                    }
                    _ => {}
                }
                Ok(buf.position() as usize)
            }
            Event::Mouse(mouse_event) => {
                let mut base = match mouse_event.kind {
                    MouseEventKind::Moved => 35,
                    MouseEventKind::Down(MouseButton::Left | MouseButton::Unknown)
                    | MouseEventKind::Up(MouseButton::Left | MouseButton::Unknown) => 0,
                    MouseEventKind::Down(MouseButton::Middle)
                    | MouseEventKind::Up(MouseButton::Middle) => 1,
                    MouseEventKind::Down(MouseButton::Right)
                    | MouseEventKind::Up(MouseButton::Right) => 2,
                    MouseEventKind::Drag(MouseButton::Left | MouseButton::Unknown) => 32,
                    MouseEventKind::Drag(MouseButton::Middle) => 33,
                    MouseEventKind::Drag(MouseButton::Right) => 34,
                    MouseEventKind::ScrollDown => 65,
                    MouseEventKind::ScrollUp => 64,
                    MouseEventKind::ScrollLeft => 66,
                    MouseEventKind::ScrollRight => 67,
                };
                if mouse_event.modifiers.intersects(KeyModifiers::SHIFT) {
                    base += 4;
                }
                if mouse_event.modifiers.intersects(KeyModifiers::ALT) {
                    base += 8;
                }
                if mouse_event.modifiers.intersects(KeyModifiers::CTRL) {
                    base += 16;
                }
                buf.write_all(b"\x1B[<")?;
                buf.write_all(base.to_string().as_bytes())?;
                buf.write_all(b";")?;
                buf.write_all((mouse_event.column + 1).to_string().as_bytes())?;
                buf.write_all(b";")?;
                buf.write_all((mouse_event.row + 1).to_string().as_bytes())?;

                if matches!(mouse_event.kind, MouseEventKind::Up(_)) {
                    buf.write_all(b"m")?;
                } else {
                    buf.write_all(b"M")?;
                }

                Ok(buf.position() as usize)
            }
            Event::Paste(text) => {
                buf.write_all(b"\x1B[200~")?;
                buf.write_all(text.as_bytes())?;
                buf.write_all(b"\x1B[201~")?;
                Ok(buf.position() as usize)
            }
            Event::Resize(_, _) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Resize events cannot be encoded",
            )),
        }
    }

    fn keycode_suffix(&self, key_code: KeyCode, buf: &mut Cursor<&mut [u8]>) -> io::Result<bool> {
        match key_code {
            KeyCode::Backspace => buf.write_all(b"\x7F"),
            KeyCode::Enter => buf.write_all(b"\r"),
            KeyCode::Left => buf.write_all(b"D"),
            KeyCode::Right => buf.write_all(b"C"),
            KeyCode::Up => buf.write_all(b"A"),
            KeyCode::Down => buf.write_all(b"B"),
            KeyCode::Home => buf.write_all(b"H"),
            KeyCode::End => buf.write_all(b"F"),
            KeyCode::PageUp => buf.write_all(b"5~"),
            KeyCode::PageDown => buf.write_all(b"6~"),
            KeyCode::Tab => buf.write_all(b"\t"),
            KeyCode::BackTab => buf.write_all(b"Z"),
            KeyCode::Delete => buf.write_all(b"3~"),
            KeyCode::Insert => buf.write_all(b"2~"),
            KeyCode::F(1) => buf.write_all(b"OP"),
            KeyCode::F(2) => buf.write_all(b"OQ"),
            KeyCode::F(3) => buf.write_all(b"OR"),
            KeyCode::F(4) => buf.write_all(b"OS"),
            KeyCode::F(5) => buf.write_all(b"15~"),
            KeyCode::F(6) => buf.write_all(b"17~"),
            KeyCode::F(7) => buf.write_all(b"18~"),
            KeyCode::F(8) => buf.write_all(b"19~"),
            KeyCode::F(9) => buf.write_all(b"20~"),
            KeyCode::F(10) => buf.write_all(b"21~"),
            KeyCode::F(11) => buf.write_all(b"23~"),
            KeyCode::F(12) => buf.write_all(b"24~"),
            KeyCode::Char(c) => {
                let pos = buf.position() as usize;
                let len = c.encode_utf8(&mut buf.get_mut()[pos..]).len();
                buf.seek_relative(len as i64)
            }
            KeyCode::Esc => buf.write_all(b"\x1B"),
            _ => return Ok(false),
        }?;
        Ok(true)
    }

    pub fn to_kitty_escape_sequence(
        &self,
        buf: &mut [u8],
        flags: KeyboardEnhancementFlags,
    ) -> io::Result<usize> {
        match self {
            Event::Key(mut key_event) => {
                if !flags.intersects(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES) {
                    return self.to_escape_sequence(buf);
                }

                if !flags.intersects(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES)
                    && key_event.kind == KeyEventKind::Press
                    && !key_event.modifiers.intersects(
                        KeyModifiers::CTRL
                            | KeyModifiers::ALT
                            | KeyModifiers::SUPER
                            | KeyModifiers::HYPER
                            | KeyModifiers::META,
                    )
                    && matches!(key_event.code, KeyCode::Char(_))
                {
                    return self.to_escape_sequence(buf);
                }

                let mut buf = Cursor::new(buf);
                buf.write_all(b"\x1B[")?;
                match key_event.code {
                    KeyCode::CapsLock => buf.write_all(b"57358")?,
                    KeyCode::ScrollLock => buf.write_all(b"57359")?,
                    KeyCode::NumLock => buf.write_all(b"57360")?,
                    KeyCode::PrintScreen => buf.write_all(b"57361")?,
                    KeyCode::Pause => buf.write_all(b"57362")?,
                    KeyCode::Menu => buf.write_all(b"57363")?,
                    KeyCode::F(val @ 13..=35) => {
                        buf.write_all(&(57376 + (val as u16 - 13)).to_string().into_bytes())?;
                    }
                    KeyCode::F(36..) => {
                        return Err(io::Error::new(io::ErrorKind::Unsupported, "unsupported"));
                    }
                    KeyCode::Media(MediaKeyCode::Play) => buf.write_all(b"57428")?,
                    KeyCode::Media(MediaKeyCode::Pause) => buf.write_all(b"57429")?,
                    KeyCode::Media(MediaKeyCode::PlayPause) => buf.write_all(b"57430")?,
                    KeyCode::Media(MediaKeyCode::Reverse) => buf.write_all(b"57431")?,
                    KeyCode::Media(MediaKeyCode::Stop) => buf.write_all(b"57432")?,
                    KeyCode::Media(MediaKeyCode::FastForward) => buf.write_all(b"57433")?,
                    KeyCode::Media(MediaKeyCode::Rewind) => buf.write_all(b"57434")?,
                    KeyCode::Media(MediaKeyCode::TrackNext) => buf.write_all(b"57435")?,
                    KeyCode::Media(MediaKeyCode::TrackPrevious) => buf.write_all(b"57436")?,
                    KeyCode::Media(MediaKeyCode::Record) => buf.write_all(b"57437")?,
                    KeyCode::Media(MediaKeyCode::LowerVolume) => buf.write_all(b"57438")?,
                    KeyCode::Media(MediaKeyCode::RaiseVolume) => buf.write_all(b"57439")?,
                    KeyCode::Media(MediaKeyCode::MuteVolume) => buf.write_all(b"57440")?,
                    KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Left) => {
                        buf.write_all(b"57441")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Left) => {
                        buf.write_all(b"57442")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Left) => {
                        buf.write_all(b"57443")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Left) => {
                        buf.write_all(b"57444")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Left) => {
                        buf.write_all(b"57445")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Left) => {
                        buf.write_all(b"57446")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Right) => {
                        buf.write_all(b"57447")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Right) => {
                        buf.write_all(b"57448")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right) => {
                        buf.write_all(b"57449")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Right) => {
                        buf.write_all(b"57450")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Right) => {
                        buf.write_all(b"57451")?
                    }
                    KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Right) => {
                        buf.write_all(b"57452")?
                    }
                    KeyCode::Modifier(
                        ModifierKeyCode::IsoLevel3Shift,
                        ModifierDirection::Unknown,
                    ) => buf.write_all(b"57453")?,
                    KeyCode::Modifier(
                        ModifierKeyCode::IsoLevel5Shift,
                        ModifierDirection::Unknown,
                    ) => buf.write_all(b"57454")?,
                    KeyCode::Null => {
                        return Err(io::Error::new(io::ErrorKind::Unsupported, "unsupported"));
                    }
                    mut key_code => {
                        if let KeyCode::Char(c) = &mut key_code {
                            if !c.is_ascii_lowercase() {
                                *c = c.to_ascii_lowercase();
                                if !key_event.modifiers.intersects(KeyModifiers::SHIFT) {
                                    key_event.modifiers |= KeyModifiers::SHIFT;
                                }
                            }
                        }
                        let old_pos = buf.position() as usize;
                        self.keycode_suffix(key_code, &mut buf)?;
                        let new_pos = buf.position() as usize;
                        let suffix_bytes = buf.get_ref()[old_pos..new_pos]
                            .iter()
                            .map(|b| b.to_string())
                            .collect::<String>()
                            .into_bytes();
                        buf.seek_relative(old_pos as i64 - new_pos as i64)?;

                        match key_code {
                            KeyCode::Char(c) => {
                                buf.write_all(&suffix_bytes)?;
                                if flags.intersects(KeyboardEnhancementFlags::REPORT_ALTERNATE_KEYS)
                                    && key_event.modifiers.intersects(KeyModifiers::SHIFT)
                                {
                                    let upper = c.to_ascii_uppercase();
                                    if upper != c {
                                        buf.write_all(b":")?;
                                        buf.write_all(&(upper as u8).to_string().into_bytes())?;
                                    }
                                }
                            }
                            KeyCode::F(1..=4) => {
                                buf.write_all(&suffix_bytes[1..])?;
                            }
                            _ => {
                                buf.write_all(&suffix_bytes)?;
                            }
                        }
                    }
                }

                if !key_event.modifiers.is_empty() || key_event.kind != KeyEventKind::Press {
                    buf.write_all(b";")?;
                    let pos = buf.position() as usize;
                    (key_event.modifiers.bits() + 1)
                        .to_string()
                        .chars()
                        .next()
                        .expect("should have exactly one char")
                        .encode_utf8(&mut buf.get_mut()[pos..]);
                    buf.seek_relative(1)?;
                }
                if flags.intersects(KeyboardEnhancementFlags::REPORT_EVENT_TYPES) {
                    match key_event.kind {
                        KeyEventKind::Repeat => {
                            buf.write_all(b":2")?;
                        }
                        KeyEventKind::Release => {
                            buf.write_all(b":3")?;
                        }
                        KeyEventKind::Press => {}
                    };
                }
                buf.write_all(b"u")?;
                Ok(buf.position() as usize)
            }
            _ => self.to_escape_sequence(buf),
        }
    }
}
