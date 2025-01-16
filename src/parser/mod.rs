use std::io;

use crate::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};

// This is a lightly modified version of crossterm's ansi escape sequence parser:
// https://github.com/crossterm-rs/crossterm/blob/master/src/event/sys/unix/parse.rs

// Additional functionality is added here to encode events as escape sequences

fn could_not_parse_event_error() -> io::Error {
    io::Error::new(io::ErrorKind::Other, "Could not parse an event.")
}

impl Event {
    pub fn to_escape_sequence(&self) -> io::Result<Vec<u8>> {
        match self {
            Event::FocusGained => Ok(b"\x1B[I".to_vec()),
            Event::FocusLost => Ok(b"\x1B[O".to_vec()),
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return Err(io::Error::new(
                        io::ErrorKind::Unsupported,
                        "Only keypress events can be encoded.",
                    ));
                }
                let mut result = {
                    let suffix = self.keycode_suffix(key_event.code);
                    let Some(mut suffix) = suffix else {
                        return Err(io::Error::new(
                            io::ErrorKind::Unsupported,
                            "Unsupported keycode.",
                        ));
                    };
                    match key_event.code {
                        KeyCode::F(1..=4) => {
                            let mut res = b"\x1B".to_vec();
                            res.append(&mut suffix);
                            res
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
                            let mut res = b"\x1B[".to_vec();
                            res.append(&mut suffix);
                            res
                        }
                        _ => suffix,
                    }
                };

                if !key_event.modifiers.is_empty() {
                    match key_event.code {
                        KeyCode::Left
                        | KeyCode::Right
                        | KeyCode::Up
                        | KeyCode::Down
                        | KeyCode::Home
                        | KeyCode::End => {
                            let last = result.pop().unwrap();
                            result.append(&mut b"1;1".to_vec());
                            result.push(last);
                        }
                        KeyCode::F(1..=4) => {
                            let last = result.pop().unwrap();
                            result.pop().unwrap();
                            result.append(&mut b"[1;1".to_vec());
                            result.push(last);
                        }
                        KeyCode::PageUp
                        | KeyCode::PageDown
                        | KeyCode::Delete
                        | KeyCode::Insert
                        | KeyCode::F(_) => {
                            let last = result.pop().unwrap();
                            result.append(&mut b";1".to_vec());
                            result.push(last);
                        }
                        _ => {}
                    }
                }
                if key_event.modifiers.intersects(KeyModifiers::SHIFT) {
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
                            result[4] += 1;
                        }
                        KeyCode::F(_) => {
                            result[5] += 1;
                        }
                        _ => {}
                    }
                }
                if key_event.modifiers.intersects(KeyModifiers::ALT) {
                    match key_event.code {
                        KeyCode::Char(_)
                        | KeyCode::Esc
                        | KeyCode::Backspace
                        | KeyCode::Enter
                        | KeyCode::Tab
                        | KeyCode::BackTab => {
                            let mut prefix = b"\x1B".to_vec();
                            prefix.append(&mut result);
                            result = prefix;
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
                        | KeyCode::F(1..=4) => {
                            result[4] += 2;
                        }
                        KeyCode::F(_) => {
                            result[5] += 2;
                        }
                        _ => {}
                    }
                }
                if key_event.modifiers.intersects(KeyModifiers::CTRL) {
                    match key_event.code {
                        KeyCode::Char(c) => {
                            *result.last_mut().unwrap() = (c as u8) + 0x1 - b'a';
                        }
                        KeyCode::Backspace => {
                            *result.last_mut().unwrap() = b'\x08';
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
                        | KeyCode::F(1..=4) => {
                            result[4] += 4;
                        }
                        KeyCode::F(_) => {
                            result[5] += 4;
                        }
                        _ => {}
                    }
                }
                Ok(result)
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
                let mut res = b"\x1B[<".to_vec();
                res.append(&mut base.to_string().as_bytes().to_vec());
                res.push(b';');
                res.append(&mut (mouse_event.column + 1).to_string().as_bytes().to_vec());
                res.push(b';');
                res.append(&mut (mouse_event.row + 1).to_string().as_bytes().to_vec());

                if matches!(mouse_event.kind, MouseEventKind::Up(_)) {
                    res.push(b'm');
                } else {
                    res.push(b'M');
                }

                Ok(res)
            }
            Event::Paste(text) => {
                let mut res = b"\x1B[200~".to_vec();
                res.append(&mut text.as_bytes().to_vec());
                res.append(&mut b"\x1B[201~".to_vec());
                Ok(res)
            }
            Event::Resize(_, _) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Resize events cannot be encoded",
            )),
        }
    }

    fn keycode_suffix(&self, key_code: KeyCode) -> Option<Vec<u8>> {
        match key_code {
            KeyCode::Backspace => Some(b"\x7F".to_vec()),
            KeyCode::Enter => Some(b"\r".to_vec()),
            KeyCode::Left => Some(b"D".to_vec()),
            KeyCode::Right => Some(b"C".to_vec()),
            KeyCode::Up => Some(b"A".to_vec()),
            KeyCode::Down => Some(b"B".to_vec()),
            KeyCode::Home => Some(b"H".to_vec()),
            KeyCode::End => Some(b"F".to_vec()),
            KeyCode::PageUp => Some(b"5~".to_vec()),
            KeyCode::PageDown => Some(b"6~".to_vec()),
            KeyCode::Tab => Some(b"\t".to_vec()),
            KeyCode::BackTab => Some(b"Z".to_vec()),
            KeyCode::Delete => Some(b"3~".to_vec()),
            KeyCode::Insert => Some(b"2~".to_vec()),
            KeyCode::F(1) => Some(b"OP".to_vec()),
            KeyCode::F(2) => Some(b"OQ".to_vec()),
            KeyCode::F(3) => Some(b"OR".to_vec()),
            KeyCode::F(4) => Some(b"OS".to_vec()),
            KeyCode::F(5) => Some(b"15~".to_vec()),
            KeyCode::F(6) => Some(b"17~".to_vec()),
            KeyCode::F(7) => Some(b"18~".to_vec()),
            KeyCode::F(8) => Some(b"19~".to_vec()),
            KeyCode::F(9) => Some(b"20~".to_vec()),
            KeyCode::F(10) => Some(b"21~".to_vec()),
            KeyCode::F(11) => Some(b"23~".to_vec()),
            KeyCode::F(12) => Some(b"24~".to_vec()),
            KeyCode::F(_) => None,
            KeyCode::Char(c) => {
                let mut dst = vec![0; 1];
                c.encode_utf8(&mut dst);
                Some(dst)
            }
            // KeyCode::Null => None,
            KeyCode::Esc => Some(b"\x1B".to_vec()),
            _ => None,
        }
    }

    pub fn to_kitty_escape_sequence(&self) -> io::Result<Vec<u8>> {
        match self {
            Event::FocusGained => Ok(b"\x1B[I".to_vec()),
            Event::FocusLost => Ok(b"\x1B[O".to_vec()),
            Event::Key(key_event) => {
                let mut suffix = if let Some(suffix) = self.keycode_suffix(key_event.code) {
                    let suffix = suffix
                        .into_iter()
                        .map(|b| b.to_string())
                        .collect::<String>()
                        .into_bytes();

                    if let KeyCode::F(1..=4) = key_event.code {
                        suffix[1..].to_vec()
                    } else {
                        suffix
                    }
                } else {
                    match key_event.code {
                        KeyCode::CapsLock => b"57358".to_vec(),
                        KeyCode::ScrollLock => b"57359".to_vec(),
                        KeyCode::NumLock => b"57360".to_vec(),
                        KeyCode::PrintScreen => b"57361".to_vec(),
                        KeyCode::Pause => b"57362".to_vec(),
                        KeyCode::Menu => b"57363".to_vec(),
                        KeyCode::F(val @ 13..=35) => {
                            (57376 + (val as u16 - 13)).to_string().into_bytes()
                        }
                        KeyCode::Media(MediaKeyCode::Play) => b"57428".to_vec(),
                        KeyCode::Media(MediaKeyCode::Pause) => b"57429".to_vec(),
                        KeyCode::Media(MediaKeyCode::PlayPause) => b"57430".to_vec(),
                        KeyCode::Media(MediaKeyCode::Reverse) => b"57431".to_vec(),
                        KeyCode::Media(MediaKeyCode::Stop) => b"57432".to_vec(),
                        KeyCode::Media(MediaKeyCode::FastForward) => b"57433".to_vec(),
                        KeyCode::Media(MediaKeyCode::Rewind) => b"57434".to_vec(),
                        KeyCode::Media(MediaKeyCode::TrackNext) => b"57435".to_vec(),
                        KeyCode::Media(MediaKeyCode::TrackPrevious) => b"57436".to_vec(),
                        KeyCode::Media(MediaKeyCode::Record) => b"57437".to_vec(),
                        KeyCode::Media(MediaKeyCode::LowerVolume) => b"57438".to_vec(),
                        KeyCode::Media(MediaKeyCode::RaiseVolume) => b"57439".to_vec(),
                        KeyCode::Media(MediaKeyCode::MuteVolume) => b"57440".to_vec(),
                        KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Left) => {
                            b"57441".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Left) => {
                            b"57442".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Left) => {
                            b"57443".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Left) => {
                            b"57444".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Left) => {
                            b"57445".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Left) => {
                            b"57446".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Right) => {
                            b"57447".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Control, ModifierDirection::Right) => {
                            b"57448".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right) => {
                            b"57449".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Right) => {
                            b"57450".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Right) => {
                            b"57451".to_vec()
                        }
                        KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Right) => {
                            b"57452".to_vec()
                        }
                        KeyCode::Modifier(
                            ModifierKeyCode::IsoLevel3Shift,
                            ModifierDirection::Unknown,
                        ) => b"57453".to_vec(),
                        KeyCode::Modifier(
                            ModifierKeyCode::IsoLevel5Shift,
                            ModifierDirection::Unknown,
                        ) => b"57454".to_vec(),
                        KeyCode::Null => {
                            return Err(io::Error::new(io::ErrorKind::Unsupported, "unsupported"));
                        }
                        _ => unreachable!(),
                    }
                };
                let mut res = b"\x1B[".to_vec();
                res.append(&mut suffix);
                res.push(b'u');
                Ok(res)
            }
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => {
                todo!()
            }
            Event::Resize(_, _) => Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Resize events cannot be encoded",
            )),
        }
    }
}

pub fn parse_event(buffer: &[u8]) -> io::Result<Option<Event>> {
    if buffer.is_empty() {
        return Ok(None);
    }

    match buffer[0] {
        b'\x1B' => {
            if buffer.len() == 1 {
                Ok(Some(Event::Key(KeyCode::Esc.into())))
            } else {
                match buffer[1] {
                    b'O' => {
                        if buffer.len() == 2 {
                            Ok(None)
                        } else {
                            match buffer[2] {
                                b'D' => Ok(Some(Event::Key(KeyCode::Left.into()))),
                                b'C' => Ok(Some(Event::Key(KeyCode::Right.into()))),
                                b'A' => Ok(Some(Event::Key(KeyCode::Up.into()))),
                                b'B' => Ok(Some(Event::Key(KeyCode::Down.into()))),
                                b'H' => Ok(Some(Event::Key(KeyCode::Home.into()))),
                                b'F' => Ok(Some(Event::Key(KeyCode::End.into()))),
                                // F1-F4
                                val @ b'P'..=b'S' => {
                                    Ok(Some(Event::Key(KeyCode::F(1 + val - b'P').into())))
                                }
                                _ => Err(could_not_parse_event_error()),
                            }
                        }
                    }
                    b'[' => parse_csi(buffer),
                    b'\x1B' => {
                        if buffer.len() == 2 {
                            Ok(Some(Event::Key(KeyEvent::new(
                                KeyCode::Esc,
                                KeyModifiers::ALT,
                            ))))
                        } else {
                            match &buffer[2..] {
                                b"[Z" => Ok(Some(Event::Key(KeyEvent::new(
                                    KeyCode::BackTab,
                                    KeyModifiers::SHIFT | KeyModifiers::ALT,
                                )))),
                                _ => Err(could_not_parse_event_error()),
                            }
                        }
                    }
                    _ => parse_event(&buffer[1..]).map(|event_option| {
                        event_option.map(|event| {
                            if let Event::Key(key_event) = event {
                                let mut alt_key_event = key_event;
                                alt_key_event.modifiers |= KeyModifiers::ALT;
                                Event::Key(alt_key_event)
                            } else {
                                event
                            }
                        })
                    }),
                }
            }
        }
        b'\r' => Ok(Some(Event::Key(KeyCode::Enter.into()))),
        // Issue #371: \n = 0xA, which is also the keycode for Ctrl+J. The only reason we get
        // newlines as input is because the terminal converts \r into \n for us. When we
        // enter raw mode, we disable that, so \n no longer has any meaning - it's better to
        // use Ctrl+J. Waiting to handle it here means it gets picked up later
        // b'\n' if !crate::terminal::sys::is_raw_mode_enabled() => {
        //     Ok(Some(Event::Key(KeyCode::Enter.into())))
        // }
        b'\t' => Ok(Some(Event::Key(KeyCode::Tab.into()))),
        b'\x7F' => Ok(Some(Event::Key(KeyCode::Backspace.into()))),
        c @ b'\x01'..=b'\x1A' => Ok(Some(Event::Key(KeyEvent::new(
            KeyCode::Char((c - 0x1 + b'a') as char),
            KeyModifiers::CTRL,
        )))),
        c @ b'\x1C'..=b'\x1F' => Ok(Some(Event::Key(KeyEvent::new(
            KeyCode::Char((c - 0x1C + b'4') as char),
            KeyModifiers::CTRL,
        )))),
        b'\0' => Ok(Some(Event::Key(KeyEvent::new(
            KeyCode::Char(' '),
            KeyModifiers::CTRL,
        )))),
        _ => parse_utf8_char(buffer).map(|maybe_char| {
            maybe_char
                .map(KeyCode::Char)
                .map(char_code_to_event)
                .map(Event::Key)
        }),
    }
}

// converts KeyCode to KeyEvent (adds shift modifier in case of uppercase characters)
fn char_code_to_event(code: KeyCode) -> KeyEvent {
    let modifiers = match code {
        KeyCode::Char(c) if c.is_uppercase() => KeyModifiers::SHIFT,
        _ => KeyModifiers::empty(),
    };
    KeyEvent::new(code, modifiers)
}

pub(crate) fn parse_csi(buffer: &[u8]) -> io::Result<Option<Event>> {
    assert!(buffer.starts_with(b"\x1B[")); // ESC [

    if buffer.len() == 2 {
        return Ok(None);
    }

    let input_event = match buffer[2] {
        b'[' => {
            if buffer.len() == 3 {
                None
            } else {
                match buffer[3] {
                    // NOTE (@imdaveho): cannot find when this occurs;
                    // having another '[' after ESC[ not a likely scenario
                    val @ b'A'..=b'E' => Some(Event::Key(KeyCode::F(1 + val - b'A').into())),
                    _ => return Err(could_not_parse_event_error()),
                }
            }
        }
        b'D' => Some(Event::Key(KeyCode::Left.into())),
        b'C' => Some(Event::Key(KeyCode::Right.into())),
        b'A' => Some(Event::Key(KeyCode::Up.into())),
        b'B' => Some(Event::Key(KeyCode::Down.into())),
        b'H' => Some(Event::Key(KeyCode::Home.into())),
        b'F' => Some(Event::Key(KeyCode::End.into())),
        b'Z' => Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::BackTab,
            KeyModifiers::SHIFT,
            KeyEventKind::Press,
        ))),
        b'M' => return parse_csi_normal_mouse(buffer),
        b'<' => return parse_csi_sgr_mouse(buffer),
        b'I' => Some(Event::FocusGained),
        b'O' => Some(Event::FocusLost),
        b';' => return parse_csi_modifier_key_code(buffer),
        // P, Q, and S for compatibility with Kitty keyboard protocol,
        // as the 1 in 'CSI 1 P' etc. must be omitted if there are no
        // modifiers pressed:
        // https://sw.kovidgoyal.net/kitty/keyboard-protocol/#legacy-functional-keys
        b'P' => Some(Event::Key(KeyCode::F(1).into())),
        b'Q' => Some(Event::Key(KeyCode::F(2).into())),
        b'R' => Some(Event::Key(KeyCode::F(3).into())),
        b'S' => Some(Event::Key(KeyCode::F(4).into())),
        b'?' => match buffer[buffer.len() - 1] {
            b'u' => return Ok(None), //return parse_csi_keyboard_enhancement_flags(buffer),
            b'c' => return Ok(None), //return parse_csi_primary_device_attributes(buffer),
            _ => None,
        },
        b'0'..=b'9' => {
            // Numbered escape code.
            if buffer.len() == 3 {
                None
            } else {
                // The final byte of a CSI sequence can be in the range 64-126, so
                // let's keep reading anything else.
                let last_byte = buffer[buffer.len() - 1];
                if !(64..=126).contains(&last_byte) {
                    None
                } else {
                    if buffer.starts_with(b"\x1B[200~") {
                        return parse_csi_bracketed_paste(buffer);
                    }
                    match last_byte {
                        b'M' => return parse_csi_rxvt_mouse(buffer),
                        b'~' => return parse_csi_special_key_code(buffer),
                        b'u' => return parse_csi_u_encoded_key_code(buffer),
                        //  b'R' => return Ok(None), //parse_csi_cursor_position(buffer),
                        _ => return parse_csi_modifier_key_code(buffer),
                    }
                }
            }
        }
        _ => return Err(could_not_parse_event_error()),
    };

    Ok(input_event)
}

pub(crate) fn next_parsed<T>(iter: &mut dyn Iterator<Item = &str>) -> io::Result<T>
where
    T: std::str::FromStr,
{
    iter.next()
        .ok_or_else(could_not_parse_event_error)?
        .parse::<T>()
        .map_err(|_| could_not_parse_event_error())
}

fn modifier_and_kind_parsed(iter: &mut dyn Iterator<Item = &str>) -> io::Result<(u8, u8)> {
    let mut sub_split = iter
        .next()
        .ok_or_else(could_not_parse_event_error)?
        .split(':');

    let modifier_mask = next_parsed::<u8>(&mut sub_split)?;

    if let Ok(kind_code) = next_parsed::<u8>(&mut sub_split) {
        Ok((modifier_mask, kind_code))
    } else {
        Ok((modifier_mask, 1))
    }
}

fn parse_modifiers(mask: u8) -> KeyModifiers {
    let modifier_mask = mask.saturating_sub(1);
    let mut modifiers = KeyModifiers::empty();
    if modifier_mask & 1 != 0 {
        modifiers |= KeyModifiers::SHIFT;
    }
    if modifier_mask & 2 != 0 {
        modifiers |= KeyModifiers::ALT;
    }
    if modifier_mask & 4 != 0 {
        modifiers |= KeyModifiers::CTRL;
    }
    if modifier_mask & 8 != 0 {
        modifiers |= KeyModifiers::SUPER;
    }
    if modifier_mask & 16 != 0 {
        modifiers |= KeyModifiers::HYPER;
    }
    if modifier_mask & 32 != 0 {
        modifiers |= KeyModifiers::META;
    }
    modifiers
}

fn parse_modifiers_to_state(mask: u8) -> KeyEventState {
    let modifier_mask = mask.saturating_sub(1);
    let mut state = KeyEventState::empty();
    if modifier_mask & 64 != 0 {
        state |= KeyEventState::CAPS_LOCK;
    }
    if modifier_mask & 128 != 0 {
        state |= KeyEventState::NUM_LOCK;
    }
    state
}

fn parse_key_event_kind(kind: u8) -> KeyEventKind {
    match kind {
        1 => KeyEventKind::Press,
        2 => KeyEventKind::Repeat,
        3 => KeyEventKind::Release,
        _ => KeyEventKind::Press,
    }
}

pub(crate) fn parse_csi_modifier_key_code(buffer: &[u8]) -> io::Result<Option<Event>> {
    assert!(buffer.starts_with(b"\x1B[")); // ESC [
    //
    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    split.next();

    let (modifiers, kind) =
        if let Ok((modifier_mask, kind_code)) = modifier_and_kind_parsed(&mut split) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
            )
        } else if buffer.len() > 3 {
            (
                parse_modifiers(
                    (buffer[buffer.len() - 2] as char)
                        .to_digit(10)
                        .ok_or_else(could_not_parse_event_error)? as u8,
                ),
                KeyEventKind::Press,
            )
        } else {
            (KeyModifiers::NONE, KeyEventKind::Press)
        };
    let key = buffer[buffer.len() - 1];

    let keycode = match key {
        b'A' => KeyCode::Up,
        b'B' => KeyCode::Down,
        b'C' => KeyCode::Right,
        b'D' => KeyCode::Left,
        b'F' => KeyCode::End,
        b'H' => KeyCode::Home,
        b'P' => KeyCode::F(1),
        b'Q' => KeyCode::F(2),
        b'R' => KeyCode::F(3),
        b'S' => KeyCode::F(4),
        _ => return Err(could_not_parse_event_error()),
    };

    let input_event = Event::Key(KeyEvent::new_with_kind(keycode, modifiers, kind));

    Ok(Some(input_event))
}

fn translate_functional_key_code(codepoint: u32) -> Option<(KeyCode, KeyEventState)> {
    if let Some(keycode) = match codepoint {
        57399 => Some(KeyCode::Char('0')),
        57400 => Some(KeyCode::Char('1')),
        57401 => Some(KeyCode::Char('2')),
        57402 => Some(KeyCode::Char('3')),
        57403 => Some(KeyCode::Char('4')),
        57404 => Some(KeyCode::Char('5')),
        57405 => Some(KeyCode::Char('6')),
        57406 => Some(KeyCode::Char('7')),
        57407 => Some(KeyCode::Char('8')),
        57408 => Some(KeyCode::Char('9')),
        57409 => Some(KeyCode::Char('.')),
        57410 => Some(KeyCode::Char('/')),
        57411 => Some(KeyCode::Char('*')),
        57412 => Some(KeyCode::Char('-')),
        57413 => Some(KeyCode::Char('+')),
        57414 => Some(KeyCode::Enter),
        57415 => Some(KeyCode::Char('=')),
        57416 => Some(KeyCode::Char(',')),
        57417 => Some(KeyCode::Left),
        57418 => Some(KeyCode::Right),
        57419 => Some(KeyCode::Up),
        57420 => Some(KeyCode::Down),
        57421 => Some(KeyCode::PageUp),
        57422 => Some(KeyCode::PageDown),
        57423 => Some(KeyCode::Home),
        57424 => Some(KeyCode::End),
        57425 => Some(KeyCode::Insert),
        57426 => Some(KeyCode::Delete),
        57427 => Some(KeyCode::KeypadBegin),
        _ => None,
    } {
        return Some((keycode, KeyEventState::KEYPAD));
    }

    if let Some(keycode) = match codepoint {
        57358 => Some(KeyCode::CapsLock),
        57359 => Some(KeyCode::ScrollLock),
        57360 => Some(KeyCode::NumLock),
        57361 => Some(KeyCode::PrintScreen),
        57362 => Some(KeyCode::Pause),
        57363 => Some(KeyCode::Menu),
        57376 => Some(KeyCode::F(13)),
        57377 => Some(KeyCode::F(14)),
        57378 => Some(KeyCode::F(15)),
        57379 => Some(KeyCode::F(16)),
        57380 => Some(KeyCode::F(17)),
        57381 => Some(KeyCode::F(18)),
        57382 => Some(KeyCode::F(19)),
        57383 => Some(KeyCode::F(20)),
        57384 => Some(KeyCode::F(21)),
        57385 => Some(KeyCode::F(22)),
        57386 => Some(KeyCode::F(23)),
        57387 => Some(KeyCode::F(24)),
        57388 => Some(KeyCode::F(25)),
        57389 => Some(KeyCode::F(26)),
        57390 => Some(KeyCode::F(27)),
        57391 => Some(KeyCode::F(28)),
        57392 => Some(KeyCode::F(29)),
        57393 => Some(KeyCode::F(30)),
        57394 => Some(KeyCode::F(31)),
        57395 => Some(KeyCode::F(32)),
        57396 => Some(KeyCode::F(33)),
        57397 => Some(KeyCode::F(34)),
        57398 => Some(KeyCode::F(35)),
        57428 => Some(KeyCode::Media(MediaKeyCode::Play)),
        57429 => Some(KeyCode::Media(MediaKeyCode::Pause)),
        57430 => Some(KeyCode::Media(MediaKeyCode::PlayPause)),
        57431 => Some(KeyCode::Media(MediaKeyCode::Reverse)),
        57432 => Some(KeyCode::Media(MediaKeyCode::Stop)),
        57433 => Some(KeyCode::Media(MediaKeyCode::FastForward)),
        57434 => Some(KeyCode::Media(MediaKeyCode::Rewind)),
        57435 => Some(KeyCode::Media(MediaKeyCode::TrackNext)),
        57436 => Some(KeyCode::Media(MediaKeyCode::TrackPrevious)),
        57437 => Some(KeyCode::Media(MediaKeyCode::Record)),
        57438 => Some(KeyCode::Media(MediaKeyCode::LowerVolume)),
        57439 => Some(KeyCode::Media(MediaKeyCode::RaiseVolume)),
        57440 => Some(KeyCode::Media(MediaKeyCode::MuteVolume)),
        57441 => Some(KeyCode::Modifier(
            ModifierKeyCode::Shift,
            ModifierDirection::Left,
        )),
        57442 => Some(KeyCode::Modifier(
            ModifierKeyCode::Control,
            ModifierDirection::Left,
        )),
        57443 => Some(KeyCode::Modifier(
            ModifierKeyCode::Alt,
            ModifierDirection::Left,
        )),
        57444 => Some(KeyCode::Modifier(
            ModifierKeyCode::Super,
            ModifierDirection::Left,
        )),
        57445 => Some(KeyCode::Modifier(
            ModifierKeyCode::Hyper,
            ModifierDirection::Left,
        )),
        57446 => Some(KeyCode::Modifier(
            ModifierKeyCode::Meta,
            ModifierDirection::Left,
        )),
        57447 => Some(KeyCode::Modifier(
            ModifierKeyCode::Shift,
            ModifierDirection::Right,
        )),
        57448 => Some(KeyCode::Modifier(
            ModifierKeyCode::Control,
            ModifierDirection::Right,
        )),
        57449 => Some(KeyCode::Modifier(
            ModifierKeyCode::Alt,
            ModifierDirection::Right,
        )),
        57450 => Some(KeyCode::Modifier(
            ModifierKeyCode::Super,
            ModifierDirection::Right,
        )),
        57451 => Some(KeyCode::Modifier(
            ModifierKeyCode::Hyper,
            ModifierDirection::Right,
        )),
        57452 => Some(KeyCode::Modifier(
            ModifierKeyCode::Meta,
            ModifierDirection::Right,
        )),
        57453 => Some(KeyCode::Modifier(
            ModifierKeyCode::IsoLevel3Shift,
            ModifierDirection::Unknown,
        )),
        57454 => Some(KeyCode::Modifier(
            ModifierKeyCode::IsoLevel5Shift,
            ModifierDirection::Unknown,
        )),
        _ => None,
    } {
        return Some((keycode, KeyEventState::empty()));
    }

    None
}

pub(crate) fn parse_csi_u_encoded_key_code(buffer: &[u8]) -> io::Result<Option<Event>> {
    assert!(buffer.starts_with(b"\x1B[")); // ESC [
    assert!(buffer.ends_with(b"u"));

    // This function parses `CSI … u` sequences. These are sequences defined in either
    // the `CSI u` (a.k.a. "Fix Keyboard Input on Terminals - Please", https://www.leonerd.org.uk/hacks/fixterms/)
    // or Kitty Keyboard Protocol (https://sw.kovidgoyal.net/kitty/keyboard-protocol/) specifications.
    // This CSI sequence is a tuple of semicolon-separated numbers.
    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    // In `CSI u`, this is parsed as:
    //
    //     CSI codepoint ; modifiers u
    //     codepoint: ASCII Dec value
    //
    // The Kitty Keyboard Protocol extends this with optional components that can be
    // enabled progressively. The full sequence is parsed as:
    //
    //     CSI unicode-key-code:alternate-key-codes ; modifiers:event-type ; text-as-codepoints u
    let mut codepoints = split
        .next()
        .ok_or_else(could_not_parse_event_error)?
        .split(':');

    let codepoint = codepoints
        .next()
        .ok_or_else(could_not_parse_event_error)?
        .parse::<u32>()
        .map_err(|_| could_not_parse_event_error())?;

    let (mut modifiers, kind, state_from_modifiers) =
        if let Ok((modifier_mask, kind_code)) = modifier_and_kind_parsed(&mut split) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
                parse_modifiers_to_state(modifier_mask),
            )
        } else {
            (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
        };

    let (mut keycode, state_from_keycode) = {
        if let Some((special_key_code, state)) = translate_functional_key_code(codepoint) {
            (special_key_code, state)
        } else if let Some(c) = char::from_u32(codepoint) {
            (
                match c {
                    '\x1B' => KeyCode::Esc,
                    '\r' => KeyCode::Enter,
                    // Issue #371: \n = 0xA, which is also the keycode for Ctrl+J. The only reason
                    // we get newlines as input is because the terminal converts
                    // \r into \n for us. When we enter raw mode, we disable
                    // that, so \n no longer has any meaning - it's better to
                    // use Ctrl+J. Waiting to handle it here means it gets picked up later
                    // '\n' if !crate::terminal::sys::is_raw_mode_enabled() => KeyCode::Enter,
                    '\t' => {
                        if modifiers.contains(KeyModifiers::SHIFT) {
                            KeyCode::BackTab
                        } else {
                            KeyCode::Tab
                        }
                    }
                    '\x7F' => KeyCode::Backspace,
                    _ => KeyCode::Char(c),
                },
                KeyEventState::empty(),
            )
        } else {
            return Err(could_not_parse_event_error());
        }
    };

    if let KeyCode::Modifier(modifier_keycode, _) = keycode {
        match modifier_keycode {
            ModifierKeyCode::Alt => {
                modifiers.set(KeyModifiers::ALT, true);
            }
            ModifierKeyCode::Control => {
                modifiers.set(KeyModifiers::CTRL, true);
            }
            ModifierKeyCode::Shift => {
                modifiers.set(KeyModifiers::SHIFT, true);
            }
            ModifierKeyCode::Super => {
                modifiers.set(KeyModifiers::SUPER, true);
            }
            ModifierKeyCode::Hyper => {
                modifiers.set(KeyModifiers::HYPER, true);
            }
            ModifierKeyCode::Meta => {
                modifiers.set(KeyModifiers::META, true);
            }
            _ => {}
        }
    }

    // When the "report alternate keys" flag is enabled in the Kitty Keyboard Protocol
    // and the terminal sends a keyboard event containing shift, the sequence will
    // contain an additional codepoint separated by a ':' character which contains
    // the shifted character according to the keyboard layout.
    if modifiers.contains(KeyModifiers::SHIFT) {
        if let Some(shifted_c) = codepoints
            .next()
            .and_then(|codepoint| codepoint.parse::<u32>().ok())
            .and_then(char::from_u32)
        {
            keycode = KeyCode::Char(shifted_c);
            modifiers.set(KeyModifiers::SHIFT, false);
        }
    }

    let input_event = Event::Key(KeyEvent::new_with_kind_and_state(
        keycode,
        modifiers,
        kind,
        state_from_keycode | state_from_modifiers,
    ));

    Ok(Some(input_event))
}

pub(crate) fn parse_csi_special_key_code(buffer: &[u8]) -> io::Result<Option<Event>> {
    assert!(buffer.starts_with(b"\x1B[")); // ESC [
    assert!(buffer.ends_with(b"~"));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    // This CSI sequence can be a list of semicolon-separated numbers.
    let first = next_parsed::<u8>(&mut split)?;

    let (modifiers, kind, state) =
        if let Ok((modifier_mask, kind_code)) = modifier_and_kind_parsed(&mut split) {
            (
                parse_modifiers(modifier_mask),
                parse_key_event_kind(kind_code),
                parse_modifiers_to_state(modifier_mask),
            )
        } else {
            (KeyModifiers::NONE, KeyEventKind::Press, KeyEventState::NONE)
        };

    let keycode = match first {
        1 | 7 => KeyCode::Home,
        2 => KeyCode::Insert,
        3 => KeyCode::Delete,
        4 | 8 => KeyCode::End,
        5 => KeyCode::PageUp,
        6 => KeyCode::PageDown,
        v @ 11..=15 => KeyCode::F(v - 10),
        v @ 17..=21 => KeyCode::F(v - 11),
        v @ 23..=26 => KeyCode::F(v - 12),
        v @ 28..=29 => KeyCode::F(v - 15),
        v @ 31..=34 => KeyCode::F(v - 17),
        _ => return Err(could_not_parse_event_error()),
    };

    let input_event = Event::Key(KeyEvent::new_with_kind_and_state(
        keycode, modifiers, kind, state,
    ));

    Ok(Some(input_event))
}

pub(crate) fn parse_csi_rxvt_mouse(buffer: &[u8]) -> io::Result<Option<Event>> {
    // rxvt mouse encoding:
    // ESC [ Cb ; Cx ; Cy ; M

    assert!(buffer.starts_with(b"\x1B[")); // ESC [
    assert!(buffer.ends_with(b"M"));

    let s = std::str::from_utf8(&buffer[2..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    let cb = next_parsed::<u8>(&mut split)?
        .checked_sub(32)
        .ok_or_else(could_not_parse_event_error)?;
    let (kind, modifiers) = parse_cb(cb)?;

    let cx = next_parsed::<u16>(&mut split)? - 1;
    let cy = next_parsed::<u16>(&mut split)? - 1;

    Ok(Some(Event::Mouse(MouseEvent {
        kind,
        column: cx,
        row: cy,
        modifiers,
    })))
}

pub(crate) fn parse_csi_normal_mouse(buffer: &[u8]) -> io::Result<Option<Event>> {
    // Normal mouse encoding: ESC [ M CB Cx Cy (6 characters only).

    assert!(buffer.starts_with(b"\x1B[M")); // ESC [ M

    if buffer.len() < 6 {
        return Ok(None);
    }

    let cb = buffer[3]
        .checked_sub(32)
        .ok_or_else(could_not_parse_event_error)?;
    let (kind, modifiers) = parse_cb(cb)?;

    // See http://www.xfree86.org/current/ctlseqs.html#Mouse%20Tracking
    // The upper left character position on the terminal is denoted as 1,1.
    // Subtract 1 to keep it synced with cursor
    let cx = u16::from(buffer[4].saturating_sub(32)) - 1;
    let cy = u16::from(buffer[5].saturating_sub(32)) - 1;

    Ok(Some(Event::Mouse(MouseEvent {
        kind,
        column: cx,
        row: cy,
        modifiers,
    })))
}

pub(crate) fn parse_csi_sgr_mouse(buffer: &[u8]) -> io::Result<Option<Event>> {
    // ESC [ < Cb ; Cx ; Cy (;) (M or m)

    assert!(buffer.starts_with(b"\x1B[<")); // ESC [ <

    if !buffer.ends_with(b"m") && !buffer.ends_with(b"M") {
        return Ok(None);
    }

    let s = std::str::from_utf8(&buffer[3..buffer.len() - 1])
        .map_err(|_| could_not_parse_event_error())?;
    let mut split = s.split(';');

    let cb = next_parsed::<u8>(&mut split)?;
    let (kind, modifiers) = parse_cb(cb)?;

    // See http://www.xfree86.org/current/ctlseqs.html#Mouse%20Tracking
    // The upper left character position on the terminal is denoted as 1,1.
    // Subtract 1 to keep it synced with cursor
    let cx = next_parsed::<u16>(&mut split)? - 1;
    let cy = next_parsed::<u16>(&mut split)? - 1;

    // When button 3 in Cb is used to represent mouse release, you can't tell which button was
    // released. SGR mode solves this by having the sequence end with a lowercase m if it's a
    // button release and an uppercase M if it's a button press.
    //
    // We've already checked that the last character is a lowercase or uppercase M at the start of
    // this function, so we just need one if.
    let kind = if buffer.last() == Some(&b'm') {
        match kind {
            MouseEventKind::Down(button) => MouseEventKind::Up(button),
            other => other,
        }
    } else {
        kind
    };

    Ok(Some(Event::Mouse(MouseEvent {
        kind,
        column: cx,
        row: cy,
        modifiers,
    })))
}

/// Cb is the byte of a mouse input that contains the button being used, the key modifiers being
/// held and whether the mouse is dragging or not.
///
/// Bit layout of cb, from low to high:
///
/// - button number
/// - button number
/// - shift
/// - meta (alt)
/// - control
/// - mouse is dragging
/// - button number
/// - button number
fn parse_cb(cb: u8) -> io::Result<(MouseEventKind, KeyModifiers)> {
    let button_number = (cb & 0b0000_0011) | ((cb & 0b1100_0000) >> 4);
    let dragging = cb & 0b0010_0000 == 0b0010_0000;

    let kind = match (button_number, dragging) {
        (0, false) => MouseEventKind::Down(MouseButton::Left),
        (1, false) => MouseEventKind::Down(MouseButton::Middle),
        (2, false) => MouseEventKind::Down(MouseButton::Right),
        (0, true) => MouseEventKind::Drag(MouseButton::Left),
        (1, true) => MouseEventKind::Drag(MouseButton::Middle),
        (2, true) => MouseEventKind::Drag(MouseButton::Right),
        (3, false) => MouseEventKind::Up(MouseButton::Left),
        (3, true) | (4, true) | (5, true) => MouseEventKind::Moved,
        (4, false) => MouseEventKind::ScrollUp,
        (5, false) => MouseEventKind::ScrollDown,
        (6, false) => MouseEventKind::ScrollLeft,
        (7, false) => MouseEventKind::ScrollRight,
        // We do not support other buttons.
        _ => return Err(could_not_parse_event_error()),
    };

    let mut modifiers = KeyModifiers::empty();

    if cb & 0b0000_0100 == 0b0000_0100 {
        modifiers |= KeyModifiers::SHIFT;
    }
    if cb & 0b0000_1000 == 0b0000_1000 {
        modifiers |= KeyModifiers::ALT;
    }
    if cb & 0b0001_0000 == 0b0001_0000 {
        modifiers |= KeyModifiers::CTRL;
    }

    Ok((kind, modifiers))
}

pub(crate) fn parse_csi_bracketed_paste(buffer: &[u8]) -> io::Result<Option<Event>> {
    // ESC [ 2 0 0 ~ pasted text ESC 2 0 1 ~
    assert!(buffer.starts_with(b"\x1B[200~"));

    if !buffer.ends_with(b"\x1b[201~") {
        Ok(None)
    } else {
        let paste = String::from_utf8_lossy(&buffer[6..buffer.len() - 6]).to_string();
        Ok(Some(Event::Paste(paste)))
    }
}

pub(crate) fn parse_utf8_char(buffer: &[u8]) -> io::Result<Option<char>> {
    match std::str::from_utf8(buffer) {
        Ok(s) => {
            let ch = s.chars().next().ok_or_else(could_not_parse_event_error)?;

            Ok(Some(ch))
        }
        Err(_) => {
            // from_utf8 failed, but we have to check if we need more bytes for code point
            // and if all the bytes we have no are valid

            let required_bytes = match buffer[0] {
                // https://en.wikipedia.org/wiki/UTF-8#Description
                (0x00..=0x7F) => 1, // 0xxxxxxx
                (0xC0..=0xDF) => 2, // 110xxxxx 10xxxxxx
                (0xE0..=0xEF) => 3, // 1110xxxx 10xxxxxx 10xxxxxx
                (0xF0..=0xF7) => 4, // 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx
                (0x80..=0xBF) | (0xF8..=0xFF) => return Err(could_not_parse_event_error()),
            };

            // More than 1 byte, check them for 10xxxxxx pattern
            if required_bytes > 1 && buffer.len() > 1 {
                for byte in &buffer[1..] {
                    if byte & !0b0011_1111 != 0b1000_0000 {
                        return Err(could_not_parse_event_error());
                    }
                }
            }

            if buffer.len() < required_bytes {
                // All bytes looks good so far, but we need more of them
                Ok(None)
            } else {
                Err(could_not_parse_event_error())
            }
        }
    }
}

#[cfg(test)]
#[path = "./parser_test.rs"]
mod tests;
