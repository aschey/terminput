use std::io;

use crate::{
    Event, KeyCode, KeyEventKind, KeyModifiers, MediaKeyCode, ModifierDirection, ModifierKeyCode,
    MouseButton, MouseEventKind,
};

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
