use std::io::{self, Cursor, Seek, Write};

use bitflags::bitflags;

use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};

bitflags! {
    #[derive(Debug, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
    pub struct KittyFlags: u8 {
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

pub enum Encoding {
    Xterm,
    Kitty(KittyFlags),
}

impl Event {
    pub fn encode(&self, buf: &mut [u8], encoding: Encoding) -> io::Result<usize> {
        match encoding {
            Encoding::Xterm => self.to_escape_sequence(buf),
            Encoding::Kitty(flags) => self.to_kitty_escape_sequence(buf, flags),
        }
    }
    fn to_escape_sequence(&self, buf: &mut [u8]) -> io::Result<usize> {
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
            Event::Key(key_event) => encode_key_event(key_event, &mut buf),
            Event::Mouse(mouse_event) => encode_mouse_event(mouse_event, &mut buf),
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

    fn to_kitty_escape_sequence(&self, buf: &mut [u8], flags: KittyFlags) -> io::Result<usize> {
        match self {
            Event::Key(key_event) => self.encode_kitty_key_event(buf, key_event, flags),
            _ => self.to_escape_sequence(buf),
        }
    }

    fn encode_kitty_key_event(
        &self,
        buf: &mut [u8],
        key_event: &KeyEvent,
        flags: KittyFlags,
    ) -> io::Result<usize> {
        if !flags.intersects(
            KittyFlags::DISAMBIGUATE_ESCAPE_CODES | KittyFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
        ) {
            return self.to_escape_sequence(buf);
        }

        // If this flag is disabled, normal text keys with no special modifiers should use
        // simple encoding
        if !flags.intersects(KittyFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES)
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

        let key_event = key_event.normalize_case();
        let mut buf = Cursor::new(buf);
        buf.write_all(b"\x1B[")?;
        let mut trailing_char = b'u';
        let is_keypad = key_event.state.intersects(KeyEventState::KEYPAD);

        // legacy encoding keys
        if !is_keypad
            && matches!(
                key_event.code,
                KeyCode::Home
                    | KeyCode::End
                    | KeyCode::Delete
                    | KeyCode::Insert
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::F(1..=12)
            )
        {
            if key_event.kind == KeyEventKind::Press && !matches!(key_event.code, KeyCode::F(1..=4))
            {
                buf.set_position(0);
                let pos = self.to_escape_sequence(buf.get_mut())?;
                return Ok(pos);
            }
            write_keycode_suffix(key_event.code, key_event.modifiers, false, &mut buf)?;
            let pos = buf.position();
            // Instead of the usual 'u' suffix, we need to use the last character from
            // the legacy encoding
            trailing_char = buf.get_ref()[pos as usize - 1];
            // Encodings with only 2 characters require adding this placeholder
            let mut add_placeholder = pos == 3;

            if matches!(key_event.code, KeyCode::F(1..=4)) {
                // F(1-4) require overwriting the second-last character from the legacy
                // encoding
                buf.set_position(pos - 2);
                if !key_event.modifiers.is_empty() || key_event.kind != KeyEventKind::Press {
                    // We need the placeholder if the F(1-4) key requires any kind of
                    // modifiers
                    add_placeholder = true;
                }
            } else {
                buf.set_position(pos - 1);
            }

            if add_placeholder {
                buf.write_all(b"1")?;
            }
        } else {
            write_kitty_encoding(key_event, flags, &mut buf)?;
        }
        write_kitty_modifiers(key_event, flags, trailing_char, &mut buf)?;
        Ok(buf.position() as usize)
    }
}

fn encode_key_event(key_event: &KeyEvent, buf: &mut Cursor<&mut [u8]>) -> io::Result<usize> {
    let key_event = key_event.normalize_case();
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
            | KeyCode::Tab => {
                buf.write_all(b"\x1B")?;
            }
            _ => {}
        }
    }
    match key_event.code {
        KeyCode::F(1..=4) => {
            buf.write_all(b"\x1B")?;
            write_keycode_suffix(key_event.code, key_event.modifiers, true, buf)?;
        }
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
        | KeyCode::F(_) => {
            buf.write_all(b"\x1B[")?;
            write_keycode_suffix(key_event.code, key_event.modifiers, true, buf)?;
        }
        KeyCode::Tab if key_event.modifiers.intersects(KeyModifiers::SHIFT) => {
            buf.write_all(b"\x1B[")?;
            write_keycode_suffix(key_event.code, key_event.modifiers, true, buf)?;
        }
        _ => {
            let handled = write_keycode_suffix(key_event.code, key_event.modifiers, true, buf)?;
            if !handled {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "unsupported key",
                ));
            }
        }
    }

    if !key_event.modifiers.is_empty() {
        write_modifier_prefix(key_event.code, buf)?;
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

fn encode_mouse_event(mouse_event: &MouseEvent, buf: &mut Cursor<&mut [u8]>) -> io::Result<usize> {
    let mut base = match mouse_event.kind {
        MouseEventKind::Moved => 35,
        MouseEventKind::Down(MouseButton::Left | MouseButton::Unknown)
        | MouseEventKind::Up(MouseButton::Left | MouseButton::Unknown) => 0,
        MouseEventKind::Down(MouseButton::Middle) | MouseEventKind::Up(MouseButton::Middle) => 1,
        MouseEventKind::Down(MouseButton::Right) | MouseEventKind::Up(MouseButton::Right) => 2,
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

fn write_kitty_modifiers(
    key_event: KeyEvent,
    flags: KittyFlags,
    trailing_char: u8,
    buf: &mut Cursor<&mut [u8]>,
) -> io::Result<()> {
    let report_event_types = flags.intersects(KittyFlags::REPORT_EVENT_TYPES);
    let extra_modifiers = key_event
        .state
        .intersection(KeyEventState::CAPS_LOCK | KeyEventState::NUM_LOCK);

    if !key_event.modifiers.is_empty()
        || !extra_modifiers.is_empty()
        || (key_event.kind != KeyEventKind::Press && report_event_types)
    {
        buf.write_all(b";")?;
        let modifier_sum = key_event.modifiers.bits() + (extra_modifiers.bits() << 5) + 1;
        buf.write_all(&modifier_sum.to_string().into_bytes())?;
    }
    if report_event_types {
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
    buf.write_all(&[trailing_char])?;
    Ok(())
}

fn write_kitty_encoding(
    key_event: KeyEvent,
    flags: KittyFlags,
    buf: &mut Cursor<&mut [u8]>,
) -> io::Result<()> {
    let is_keypad = key_event.state.intersects(KeyEventState::KEYPAD);
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
        KeyCode::Modifier(ModifierKeyCode::IsoLevel3Shift, ModifierDirection::Unknown) => {
            buf.write_all(b"57453")?
        }
        KeyCode::Modifier(ModifierKeyCode::IsoLevel5Shift, ModifierDirection::Unknown) => {
            buf.write_all(b"57454")?
        }
        KeyCode::Null => {
            return Err(io::Error::new(io::ErrorKind::Unsupported, "unsupported"));
        }
        KeyCode::Char(val @ '0'..='9') if is_keypad => {
            buf.write_all(&(57399 + (val as u16 - 48)).to_string().into_bytes())?
        }
        KeyCode::Char('.') if is_keypad => buf.write_all(b"57409")?,
        KeyCode::Char('/') if is_keypad => buf.write_all(b"57410")?,
        KeyCode::Char('*') if is_keypad => buf.write_all(b"57411")?,
        KeyCode::Char('-') if is_keypad => buf.write_all(b"57412")?,
        KeyCode::Char('+') if is_keypad => buf.write_all(b"57413")?,
        KeyCode::Enter if is_keypad => buf.write_all(b"57414")?,
        KeyCode::Char('=') if is_keypad => buf.write_all(b"57415")?,
        KeyCode::Char(',') if is_keypad => buf.write_all(b"57416")?,
        KeyCode::Left if is_keypad => buf.write_all(b"57417")?,
        KeyCode::Right if is_keypad => buf.write_all(b"57418")?,
        KeyCode::Up if is_keypad => buf.write_all(b"57419")?,
        KeyCode::Down if is_keypad => buf.write_all(b"57420")?,
        KeyCode::PageUp if is_keypad => buf.write_all(b"57421")?,
        KeyCode::PageDown if is_keypad => buf.write_all(b"57422")?,
        KeyCode::Home if is_keypad => buf.write_all(b"57423")?,
        KeyCode::End if is_keypad => buf.write_all(b"57424")?,
        KeyCode::Insert if is_keypad => buf.write_all(b"57425")?,
        KeyCode::Delete if is_keypad => buf.write_all(b"57426")?,
        KeyCode::KeypadBegin if is_keypad => buf.write_all(b"57427")?,
        KeyCode::Char(c) => {
            // We should always use the lower-cased key for the first value
            let c = c.to_ascii_lowercase();
            convert_suffix_code(KeyCode::Char(c), key_event.modifiers, buf)?;
            if flags.intersects(KittyFlags::REPORT_ALTERNATE_KEYS)
                && key_event.modifiers.intersects(KeyModifiers::SHIFT)
            {
                // Ideally we could do this for other chars besides just ascii,
                // but that requires knowing the keyboard layout
                let upper = c.to_ascii_uppercase();
                if upper != c {
                    buf.write_all(b":")?;
                    buf.write_all(&(upper as u8).to_string().into_bytes())?;
                }
            }
        }
        KeyCode::Esc | KeyCode::Enter | KeyCode::Tab | KeyCode::Backspace => {
            convert_suffix_code(key_event.code, key_event.modifiers, buf)?;
        }
        key_code => {
            write_keycode_suffix(key_code, key_event.modifiers, false, buf)?;
        }
    }
    Ok(())
}

fn write_keycode_suffix(
    key_code: KeyCode,
    modifiers: KeyModifiers,
    special_back_tab: bool,
    buf: &mut Cursor<&mut [u8]>,
) -> io::Result<bool> {
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
        KeyCode::Tab if modifiers.intersects(KeyModifiers::SHIFT) && special_back_tab => {
            buf.write_all(b"Z")
        }
        KeyCode::Tab => buf.write_all(b"\t"),
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

fn write_modifier_prefix(key_code: KeyCode, buf: &mut Cursor<&mut [u8]>) -> io::Result<()> {
    let pos = buf.position() as usize;
    let last = buf.get_mut()[pos - 1];
    match key_code {
        KeyCode::Left
        | KeyCode::Right
        | KeyCode::Up
        | KeyCode::Down
        | KeyCode::Home
        | KeyCode::End => {
            buf.seek_relative(-1)?;
            buf.write_all(b"1;1")?;
            buf.write_all(&[last])?;
        }
        KeyCode::F(1..=4) => {
            buf.seek_relative(-2)?;
            buf.write_all(b"[1;1")?;
            buf.write_all(&[last])?;
        }
        KeyCode::PageUp | KeyCode::PageDown | KeyCode::Delete | KeyCode::Insert | KeyCode::F(_) => {
            buf.seek_relative(-1)?;
            buf.write_all(b";1")?;
            buf.write_all(&[last])?;
        }
        _ => {}
    }
    Ok(())
}

fn convert_suffix_code(
    key_code: KeyCode,
    modifiers: KeyModifiers,
    buf: &mut Cursor<&mut [u8]>,
) -> io::Result<()> {
    let old_pos = buf.position() as usize;
    write_keycode_suffix(key_code, modifiers, false, buf)?;
    let new_pos = buf.position() as usize;
    let suffix_bytes = buf.get_ref()[old_pos..new_pos]
        .iter()
        .map(|b| b.to_string())
        .collect::<String>()
        .into_bytes();
    buf.seek_relative(old_pos as i64 - new_pos as i64)?;
    buf.write_all(&suffix_bytes)?;
    Ok(())
}
