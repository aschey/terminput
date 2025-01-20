use terminput::encoder::{Encoding, KittyFlags};
use terminput::parser::parse_event;
use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};

#[test]
fn test_esc_key() {
    assert_eq!(
        parse_event(b"\x1B").unwrap(),
        Some(Event::Key(KeyCode::Esc.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Esc.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B");

    assert_eq!(
        parse_event(b"\x1B\x1B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::ALT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x1B");

    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();

    assert_eq!(buf[..written], *b"\x1B");
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::ALT | KeyModifiers::CTRL))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x1B");
}

#[test]
fn test_backspace() {
    assert_eq!(
        parse_event(b"\x7F").unwrap(),
        Some(Event::Key(KeyCode::Backspace.into()))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Backspace.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x7F");

    assert_eq!(
        parse_event(b"\x1B\x7F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Backspace).modifiers(KeyModifiers::ALT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Backspace).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x7F");

    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Backspace).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x08");

    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Backspace).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x08");
}

#[test]
fn test_kitty_backspace() {
    assert_eq!(
        parse_event(b"\x1B[127u").unwrap(),
        Some(Event::Key(KeyCode::Backspace.into()))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Backspace.into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[127u");
}

#[test]
fn test_focus_gained() {
    assert_eq!(parse_event(b"\x1B[I").unwrap(), Some(Event::FocusGained));
    let mut buf = [0; 8];
    let written = Event::FocusGained
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[I");
    let mut buf = [0; 8];
    let written = Event::FocusGained
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[I");
}

#[test]
fn test_focus_lost() {
    assert_eq!(parse_event(b"\x1B[O").unwrap(), Some(Event::FocusLost));
    let mut buf = [0; 8];
    let written = Event::FocusLost.encode(&mut buf, Encoding::Xterm).unwrap();
    assert_eq!(buf[..written], *b"\x1B[O");
    let mut buf = [0; 8];
    let written = Event::FocusLost
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[O");
}

#[test]
fn test_enter() {
    assert_eq!(
        parse_event(b"\r").unwrap(),
        Some(Event::Key(KeyCode::Enter.into()))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Enter.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\r");

    assert_eq!(
        parse_event(b"\x1B\r").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Enter).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Enter).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B\r");
}

#[test]
fn test_alt_key() {
    assert_eq!(
        parse_event(b"\x1Bc").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::ALT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1Bc");
}

#[test]
fn test_ctrl_key() {
    assert_eq!(
        parse_event(b"\x03").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x03");
}

#[test]
fn test_alt_shift() {
    assert_eq!(
        parse_event(b"\x1BH").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('H')).modifiers(KeyModifiers::ALT | KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('H')).modifiers(KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1BH")
}

#[test]
fn test_ctrl_alt() {
    assert_eq!(
        parse_event(b"\x1B\x14").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('t')).modifiers(KeyModifiers::ALT | KeyModifiers::CTRL)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('t')).modifiers(KeyModifiers::ALT | KeyModifiers::CTRL),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x14");
}

#[test]
fn test_kitty_modifiers() {
    assert_eq!(
        parse_event(b"\x1B[99;5u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;5u");

    assert_eq!(
        parse_event(b"\x1B[99:67;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('C')).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('C')).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99:67;2u");
    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99:67;2u");
    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('C')).modifiers(KeyModifiers::NONE))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99:67;2u");

    assert_eq!(
        parse_event(b"\x1B[99;7u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;7u");

    assert_eq!(
        parse_event(b"\x1B[99;71u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c'))
                .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
                .state(KeyEventState::CAPS_LOCK)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('c'))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
            .state(KeyEventState::CAPS_LOCK),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;71u");

    assert_eq!(
        parse_event(b"\x1B[99;199u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c'))
                .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT,)
                .state(KeyEventState::CAPS_LOCK | KeyEventState::NUM_LOCK,)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('c'))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
            .state(KeyEventState::CAPS_LOCK | KeyEventState::NUM_LOCK),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;199u");

    assert_eq!(
        parse_event(b"\x1B[57408;200u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('9'))
                .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT,)
                .state(KeyEventState::CAPS_LOCK | KeyEventState::NUM_LOCK | KeyEventState::KEYPAD,)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('9'))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT)
            .state(KeyEventState::CAPS_LOCK | KeyEventState::NUM_LOCK | KeyEventState::KEYPAD),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57408;200u");
}

#[test]
fn test_kitty_alternate_keys() {
    assert_eq!(
        parse_event(b"\x1B[99:67;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99:67;2u");

    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::SHIFT))
        .encode(
            &mut buf,
            Encoding::Kitty(
                KittyFlags::DISAMBIGUATE_ESCAPE_CODES | KittyFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
            ),
        )
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;2u");
}

#[test]
fn test_kitty_event_types() {
    assert_eq!(
        parse_event(b"\x1B[99;1:3u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c'))
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('c'))
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99;1:3u");

    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Char('c'))
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Release),
    )
    .encode(
        &mut buf,
        Encoding::Kitty(
            KittyFlags::DISAMBIGUATE_ESCAPE_CODES | KittyFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES,
        ),
    )
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[99u");
}

#[test]
fn test_kitty_arrow() {
    assert_eq!(
        parse_event(b"\x1B[D").unwrap(),
        Some(Event::Key(KeyCode::Left.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Left.into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[D");

    assert_eq!(
        parse_event(b"\x1B[1;2D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2D");

    assert_eq!(
        parse_event(b"\x1B[1;1:3D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Left)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;1:3D");

    assert_eq!(
        parse_event(b"\x1B[1;1:2D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Repeat)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Left)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Repeat),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;1:2D");

    assert_eq!(
        parse_event(b"\x1B[1;5:3D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left)
                .modifiers(KeyModifiers::CTRL)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Left)
            .modifiers(KeyModifiers::CTRL)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5:3D");

    assert_eq!(
        parse_event(b"\x1B[1;3:3D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left)
                .modifiers(KeyModifiers::ALT)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Left)
            .modifiers(KeyModifiers::ALT)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3:3D");
}

#[test]
fn test_home_key() {
    assert_eq!(
        parse_event(b"\x1B[H").unwrap(),
        Some(Event::Key(KeyCode::Home.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Home.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[H");

    assert_eq!(
        parse_event(b"\x1B[2H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2H");

    assert_eq!(
        parse_event(b"\x1B[1;5H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5H");

    assert_eq!(
        parse_event(b"\x1B[1;3H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3H");

    assert_eq!(
        parse_event(b"\x1B[1;7H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7H");

    assert_eq!(
        parse_event(b"\x1B[1;8H").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Home).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    Event::Key(
        KeyEvent::new(KeyCode::Home)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8H");
}

#[test]
fn test_kitty_home() {
    assert_eq!(
        parse_event(b"\x1B[H").unwrap(),
        Some(Event::Key(KeyCode::Home.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Home.into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[H");

    assert_eq!(
        parse_event(b"\x1B[1;2H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Home).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2H");

    assert_eq!(
        parse_event(b"\x1B[1;1:3H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Home)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;1:3H");

    assert_eq!(
        parse_event(b"\x1B[1;1:2H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Repeat)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Home)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Repeat),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;1:2H");

    assert_eq!(
        parse_event(b"\x1B[1;5:3H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home)
                .modifiers(KeyModifiers::CTRL)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Home)
            .modifiers(KeyModifiers::CTRL)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5:3H");

    assert_eq!(
        parse_event(b"\x1B[1;3:3H").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Home)
                .modifiers(KeyModifiers::ALT)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Home)
            .modifiers(KeyModifiers::ALT)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3:3H");
}

#[test]
fn test_end_key() {
    assert_eq!(
        parse_event(b"\x1B[F").unwrap(),
        Some(Event::Key(KeyCode::End.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::End.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[F");

    assert_eq!(
        parse_event(b"\x1B[2F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2F");

    assert_eq!(
        parse_event(b"\x1B[1;5F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5F");

    assert_eq!(
        parse_event(b"\x1B[1;3F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3F");

    assert_eq!(
        parse_event(b"\x1B[1;7F").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::End).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7F");

    assert_eq!(
        parse_event(b"\x1B[1;8F").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::End).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::End)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8F");
}

#[test]
fn test_page_up() {
    assert_eq!(
        parse_event(b"\x1B[5~").unwrap(),
        Some(Event::Key(KeyCode::PageUp.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::PageUp.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5~");

    assert_eq!(
        parse_event(b"\x1B[5;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5;2~");

    assert_eq!(
        parse_event(b"\x1B[5;5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5;5~");

    assert_eq!(
        parse_event(b"\x1B[5;3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5;3~");

    assert_eq!(
        parse_event(b"\x1B[5;7~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5;7~");

    assert_eq!(
        parse_event(b"\x1B[5;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::PageUp).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::PageUp)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5;8~");
}

#[test]
fn test_page_down() {
    assert_eq!(
        parse_event(b"\x1B[6~").unwrap(),
        Some(Event::Key(KeyCode::PageDown.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::PageDown.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6~");

    assert_eq!(
        parse_event(b"\x1B[6;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6;2~");

    assert_eq!(
        parse_event(b"\x1B[6;5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6;5~");

    assert_eq!(
        parse_event(b"\x1B[6;3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6;3~");

    assert_eq!(
        parse_event(b"\x1B[6;7~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6;7~");

    assert_eq!(
        parse_event(b"\x1B[6;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::PageDown).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    Event::Key(
        KeyEvent::new(KeyCode::PageDown)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6;8~");
}

#[test]
fn test_left_arrow() {
    assert_eq!(
        parse_event(b"\x1B[D").unwrap(),
        Some(Event::Key(KeyCode::Left.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Left.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[D");

    assert_eq!(
        parse_event(b"\x1B[2D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2D");

    assert_eq!(
        parse_event(b"\x1B[1;5D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5D");

    assert_eq!(
        parse_event(b"\x1B[1;3D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3D");

    assert_eq!(
        parse_event(b"\x1B[1;7D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7D");

    assert_eq!(
        parse_event(b"\x1B[1;8D").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Left).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Left)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8D");
}

#[test]
fn test_right_arrow() {
    assert_eq!(
        parse_event(b"\x1B[C").unwrap(),
        Some(Event::Key(KeyCode::Right.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Right.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[C");

    assert_eq!(
        parse_event(b"\x1B[2C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2C");

    assert_eq!(
        parse_event(b"\x1B[1;5C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5C");

    assert_eq!(
        parse_event(b"\x1B[1;3C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3C");

    assert_eq!(
        parse_event(b"\x1B[1;7C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Right).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7C");

    assert_eq!(
        parse_event(b"\x1B[1;8C").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Right).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Right)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8C");
}

#[test]
fn test_up_arrow() {
    assert_eq!(
        parse_event(b"\x1B[A").unwrap(),
        Some(Event::Key(KeyCode::Up.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Up.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[A");

    assert_eq!(
        parse_event(b"\x1B[2A").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2A").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2A");

    assert_eq!(
        parse_event(b"\x1B[1;5A").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5A");

    assert_eq!(
        parse_event(b"\x1B[1;3A").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3A");

    assert_eq!(
        parse_event(b"\x1B[1;7A").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Up).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7A");

    assert_eq!(
        parse_event(b"\x1B[1;8A").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Up).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Up)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8A");
}

#[test]
fn test_down_arrow() {
    assert_eq!(
        parse_event(b"\x1B[B").unwrap(),
        Some(Event::Key(KeyCode::Down.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Down.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[B");

    assert_eq!(
        parse_event(b"\x1B[2B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::SHIFT)
        ))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2B");

    assert_eq!(
        parse_event(b"\x1B[1;5B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5B");

    assert_eq!(
        parse_event(b"\x1B[1;3B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;3B");

    assert_eq!(
        parse_event(b"\x1B[1;7B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Down).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;7B");

    assert_eq!(
        parse_event(b"\x1B[1;8B").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Down).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Down)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8B");
}

#[test]
fn test_delete() {
    assert_eq!(
        parse_event(b"\x1B[3~").unwrap(),
        Some(Event::Key(KeyCode::Delete.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Delete.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3~");

    assert_eq!(
        parse_event(b"\x1B[3;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3;2~");

    assert_eq!(
        parse_event(b"\x1B[3;5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3;5~");

    assert_eq!(
        parse_event(b"\x1B[3;3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3;3~");

    assert_eq!(
        parse_event(b"\x1B[3;7~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3;7~");

    assert_eq!(
        parse_event(b"\x1B[3;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Delete).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Delete)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[3;8~");
}

#[test]
fn test_insert() {
    assert_eq!(
        parse_event(b"\x1B[2~").unwrap(),
        Some(Event::Key(KeyCode::Insert.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Insert.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2~");

    assert_eq!(
        parse_event(b"\x1B[2;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;2~");

    assert_eq!(
        parse_event(b"\x1B[2;5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;5~");

    assert_eq!(
        parse_event(b"\x1B[2;3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;3~");

    assert_eq!(
        parse_event(b"\x1B[2;7~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::CTRL | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;7~");

    assert_eq!(
        parse_event(b"\x1B[2;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Insert).modifiers(
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert)
            .modifiers(KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;8~");
}

#[test]
fn test_kitty_insert() {
    assert_eq!(
        parse_event(b"\x1B[2~").unwrap(),
        Some(Event::Key(KeyCode::Insert.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Insert.into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2~");

    assert_eq!(
        parse_event(b"\x1B[2;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Insert).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;2~");

    assert_eq!(
        parse_event(b"\x1B[2;1:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;1:3~");

    assert_eq!(
        parse_event(b"\x1B[2;1:2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Repeat)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert)
            .modifiers(KeyModifiers::empty())
            .kind(KeyEventKind::Repeat),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;1:2~");

    assert_eq!(
        parse_event(b"\x1B[2;5:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert)
                .modifiers(KeyModifiers::CTRL)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert)
            .modifiers(KeyModifiers::CTRL)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;5:3~");

    assert_eq!(
        parse_event(b"\x1B[2;3:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Insert)
                .modifiers(KeyModifiers::ALT)
                .kind(KeyEventKind::Release)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::Insert)
            .modifiers(KeyModifiers::ALT)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[2;3:3~");
}

#[test]
fn test_parse_event_subsequent_calls() {
    // parse_csi_bracketed_paste
    assert_eq!(
        parse_event(b"\x1B[200~on and on and on\x1B[201~").unwrap(),
        Some(Event::Paste("on and on and on".to_string())),
    );
    let mut buf = [0; 32];
    let written = Event::Paste("on and on and on".to_string())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[200~on and on and on\x1B[201~");
    let mut buf = [0; 32];
    let written = Event::Paste("on and on and on".to_string())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[200~on and on and on\x1B[201~");

    // parse_csi_rxvt_mouse
    assert_eq!(
        parse_event(b"\x1B[32;30;40;M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 29,
            row: 39,
            modifiers: KeyModifiers::empty(),
        }))
    );

    // parse_csi_normal_mouse
    assert_eq!(
        parse_event(b"\x1B[M0\x60\x70").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 63,
            row: 79,
            modifiers: KeyModifiers::CTRL,
        }))
    );

    // parse_csi_sgr_mouse
    assert_eq!(
        parse_event(b"\x1B[<0;20;10;M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );

    // parse_utf8_char
    assert_eq!(
        parse_event("Ž".as_bytes()).unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('Ž')).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('Ž')).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *"Ž".as_bytes());
}

#[test]
fn test_parse_tab() {
    assert_eq!(
        parse_event(b"\t").unwrap(),
        Some(Event::Key(KeyCode::Tab.into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Tab.into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\t");

    assert_eq!(
        parse_event(b"\x1B[Z").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::SHIFT,)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[Z");

    assert_eq!(
        parse_event(b"\x1B\x09").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::ALT)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::ALT))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x09");

    assert_eq!(
        parse_event(b"\x1B\x1B[Z").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::ALT | KeyModifiers::SHIFT)
        ))
    );
    let mut buf = [0; 8];
    let written =
        Event::Key(KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::ALT | KeyModifiers::SHIFT))
            .encode(&mut buf, Encoding::Xterm)
            .unwrap();
    assert_eq!(buf[..written], *b"\x1B\x1B[Z");
}

#[test]
fn test_kitty_tab() {
    assert_eq!(
        parse_event(b"\x1B[9u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[9u");

    assert_eq!(
        parse_event(b"\x1B[9;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Tab).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[9;2u");
}

#[test]
fn test_parse_csi() {
    assert_eq!(
        parse_event(b"\x1B[D").unwrap(),
        Some(Event::Key(KeyCode::Left.into())),
    );
}

#[test]
fn test_parse_csi_modifier_key_code() {
    assert_eq!(
        parse_event(b"\x1B[2D").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Left).modifiers(KeyModifiers::SHIFT)
        )),
    );
}

#[test]
fn test_parse_csi_special_key_code() {
    assert_eq!(
        parse_event(b"\x1B[3~").unwrap(),
        Some(Event::Key(KeyCode::Delete.into())),
    );
}

#[test]
fn test_parse_csi_special_key_code_multiple_values_not_supported() {
    assert_eq!(
        parse_event(b"\x1B[3;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Delete).modifiers(KeyModifiers::SHIFT)
        )),
    );
}

#[test]
fn test_parse_csi_bracketed_paste() {
    assert_eq!(
        parse_event(b"\x1B[200~o").unwrap(),
        None,
        "A partial bracketed paste isn't parsed"
    );
    assert_eq!(
        parse_event(b"\x1B[200~o\x1B[2D").unwrap(),
        None,
        "A partial bracketed paste containing another escape code isn't parsed"
    );
    assert_eq!(
        parse_event(b"\x1B[200~o\x1B[2D\x1B[201~").unwrap(),
        Some(Event::Paste("o\x1B[2D".to_string()))
    );
}

#[test]
fn test_parse_csi_focus() {
    assert_eq!(parse_event(b"\x1B[O").unwrap(), Some(Event::FocusLost));
}

#[test]
fn test_parse_csi_rxvt_mouse() {
    assert_eq!(
        parse_event(b"\x1B[32;30;40;M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 29,
            row: 39,
            modifiers: KeyModifiers::empty(),
        }))
    );
}

#[test]
fn test_parse_csi_normal_mouse() {
    assert_eq!(
        parse_event(b"\x1B[M0\x60\x70").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 63,
            row: 79,
            modifiers: KeyModifiers::CTRL,
        }))
    );
}

#[test]
fn test_parse_csi_sgr_mouse() {
    assert_eq!(
        parse_event(b"\x1B[<35;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty()
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Moved,
        column: 19,
        row: 9,
        modifiers: KeyModifiers::empty(),
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<35;20;10M");
    assert_eq!(
        parse_event(b"\x1B[<39;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 19,
            row: 9,
            modifiers: KeyModifiers::SHIFT
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Moved,
        column: 19,
        row: 9,
        modifiers: KeyModifiers::SHIFT,
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<39;20;10M");

    assert_eq!(
        parse_event(b"\x1B[<0;20;10;M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    assert_eq!(
        parse_event(b"\x1B[<0;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Left),
        column: 19,
        row: 9,
        modifiers: KeyModifiers::empty(),
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<0;20;10M");

    assert_eq!(
        parse_event(b"\x1B[<0;20;10;m").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    assert_eq!(
        parse_event(b"\x1B[<0;20;10m").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Up(MouseButton::Left),
        column: 19,
        row: 9,
        modifiers: KeyModifiers::empty(),
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<0;20;10m");

    assert_eq!(
        parse_event(b"\x1B[<2;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Right),
        column: 19,
        row: 9,
        modifiers: KeyModifiers::empty(),
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<2;20;10M");

    assert_eq!(
        parse_event(b"\x1B[<1;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Middle),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    let mut buf = [0; 16];
    let written = Event::Mouse(MouseEvent {
        kind: MouseEventKind::Down(MouseButton::Middle),
        column: 19,
        row: 9,
        modifiers: KeyModifiers::empty(),
    })
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[<1;20;10M");
}

#[test]
fn test_utf8() {
    // https://www.php.net/manual/en/reference.pcre.pattern.modifiers.php#54805

    // 'Valid ASCII' => "a",
    assert_eq!(
        parse_event(b"a").unwrap(),
        Some(Event::Key(KeyCode::Char('a').into()))
    );

    // 'Valid 2 Octet Sequence' => "\xc3\xb1",
    assert_eq!(
        parse_event(&[0xC3, 0xB1]).unwrap(),
        Some(Event::Key(KeyCode::Char('ñ').into()))
    );

    // 'Invalid 2 Octet Sequence' => "\xc3\x28",
    assert!(parse_event(&[0xC3, 0x28]).is_err());

    // 'Invalid Sequence Identifier' => "\xa0\xa1",
    assert!(parse_event(&[0xA0, 0xA1]).is_err());

    // 'Valid 3 Octet Sequence' => "\xe2\x82\xa1",
    assert_eq!(
        parse_event(&[0xE2, 0x81, 0xA1]).unwrap(),
        Some(Event::Key(KeyCode::Char('\u{2061}').into()))
    );

    // 'Invalid 3 Octet Sequence (in 2nd Octet)' => "\xe2\x28\xa1",
    assert!(parse_event(&[0xE2, 0x28, 0xA1]).is_err());

    // 'Invalid 3 Octet Sequence (in 3rd Octet)' => "\xe2\x82\x28",
    assert!(parse_event(&[0xE2, 0x82, 0x28]).is_err());

    // 'Valid 4 Octet Sequence' => "\xf0\x90\x8c\xbc",
    assert_eq!(
        parse_event(&[0xF0, 0x90, 0x8C, 0xBC]).unwrap(),
        Some(Event::Key(KeyCode::Char('𐌼').into()))
    );

    // 'Invalid 4 Octet Sequence (in 2nd Octet)' => "\xf0\x28\x8c\xbc",
    assert!(parse_event(&[0xF0, 0x28, 0x8C, 0xBC]).is_err());

    // 'Invalid 4 Octet Sequence (in 3rd Octet)' => "\xf0\x90\x28\xbc",
    assert!(parse_event(&[0xF0, 0x90, 0x28, 0xBC]).is_err());

    // 'Invalid 4 Octet Sequence (in 4th Octet)' => "\xf0\x28\x8c\x28",
    assert!(parse_event(&[0xF0, 0x28, 0x8C, 0x28]).is_err());
}

#[test]
fn test_parse_char_event_lowercase() {
    assert_eq!(
        parse_event(b"c").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('c')).modifiers(KeyModifiers::empty())
        )),
    );

    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Char('c').into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"c");
}

#[test]
fn test_parse_char_event_uppercase() {
    assert_eq!(
        parse_event(b"C").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('C')).modifiers(KeyModifiers::SHIFT)
        )),
    );

    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::Char('C').into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"C");
}

#[test]
fn test_parse_basic_csi_u_encoded_key_code() {
    assert_eq!(
        parse_event(b"\x1B[97u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::empty())
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('A')).modifiers(KeyModifiers::SHIFT)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97:65;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('A')).modifiers(KeyModifiers::SHIFT)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;7u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::ALT | KeyModifiers::CTRL)
        )),
    );
}

#[test]
fn test_parse_fn_keys() {
    assert_eq!(
        parse_event(b"\x1B[11~").unwrap(),
        Some(Event::Key(KeyCode::F(1).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOP").unwrap(),
        Some(Event::Key(KeyCode::F(1).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(1).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1BOP");
    assert_eq!(
        parse_event(b"\x1B[1;5P").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(1)).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(1)).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5P");
    assert_eq!(
        parse_event(b"\x1B[1;8P").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(1)).modifiers(
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(1))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8P");

    assert_eq!(
        parse_event(b"\x1B[12~").unwrap(),
        Some(Event::Key(KeyCode::F(2).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOQ").unwrap(),
        Some(Event::Key(KeyCode::F(2).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(2).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1BOQ");
    assert_eq!(
        parse_event(b"\x1B[1;5Q").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(2)).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(2)).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5Q");
    assert_eq!(
        parse_event(b"\x1B[1;8Q").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(2)).modifiers(
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(2))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8Q");

    assert_eq!(
        parse_event(b"\x1B[13~").unwrap(),
        Some(Event::Key(KeyCode::F(3).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOR").unwrap(),
        Some(Event::Key(KeyCode::F(3).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(3).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1BOR");
    assert_eq!(
        parse_event(b"\x1B[1;5R").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(3)).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(3)).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5R");
    assert_eq!(
        parse_event(b"\x1B[1;8R").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(3)).modifiers(
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(3))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8R");

    assert_eq!(
        parse_event(b"\x1B[14~").unwrap(),
        Some(Event::Key(KeyCode::F(4).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOS").unwrap(),
        Some(Event::Key(KeyCode::F(4).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(4).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1BOS");
    assert_eq!(
        parse_event(b"\x1B[1;5S").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(4)).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(4)).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;5S");
    assert_eq!(
        parse_event(b"\x1B[1;8S").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(4)).modifiers(
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(4))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;8S");

    assert_eq!(
        parse_event(b"\x1B[15~").unwrap(),
        Some(Event::Key(KeyCode::F(5).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(5).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15~");
    assert_eq!(
        parse_event(b"\x1B[15;5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(5)).modifiers(KeyModifiers::CTRL)
        ))
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(5)).modifiers(KeyModifiers::CTRL))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15;5~");
    assert_eq!(
        parse_event(b"\x1B[15;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(5)).modifiers(
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(5))
            .modifiers(KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT),
    )
    .encode(&mut buf, Encoding::Xterm)
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15;8~");

    assert_eq!(
        parse_event(b"\x1B[17~").unwrap(),
        Some(Event::Key(KeyCode::F(6).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(6).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[17~");

    assert_eq!(
        parse_event(b"\x1B[18~").unwrap(),
        Some(Event::Key(KeyCode::F(7).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(7).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[18~");

    assert_eq!(
        parse_event(b"\x1B[19~").unwrap(),
        Some(Event::Key(KeyCode::F(8).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(8).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[19~");

    assert_eq!(
        parse_event(b"\x1B[20~").unwrap(),
        Some(Event::Key(KeyCode::F(9).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(9).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[20~");

    assert_eq!(
        parse_event(b"\x1B[21~").unwrap(),
        Some(Event::Key(KeyCode::F(10).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(10).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[21~");

    assert_eq!(
        parse_event(b"\x1B[23~").unwrap(),
        Some(Event::Key(KeyCode::F(11).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(11).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[23~");

    assert_eq!(
        parse_event(b"\x1B[24~").unwrap(),
        Some(Event::Key(KeyCode::F(12).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(12).into())
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[24~");
}

#[test]
fn test_kitty_fn_keys() {
    assert_eq!(
        parse_event(b"\x1B[P").unwrap(),
        Some(Event::Key(KeyCode::F(1).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(1).into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[P");

    assert_eq!(
        parse_event(b"\x1B[1;2P").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(1)).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(1)).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2P");

    assert_eq!(
        parse_event(b"\x1B[1;1:3P").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(1))
                .modifiers(KeyModifiers::NONE)
                .kind(KeyEventKind::Release,)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(1))
            .modifiers(KeyModifiers::NONE)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;1:3P");

    assert_eq!(
        parse_event(b"\x1B[1;2:3P").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(1))
                .modifiers(KeyModifiers::SHIFT)
                .kind(KeyEventKind::Release,)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(1))
            .modifiers(KeyModifiers::SHIFT)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[1;2:3P");

    assert_eq!(
        parse_event(b"\x1B[15~").unwrap(),
        Some(Event::Key(KeyCode::F(5).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(5).into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15~");

    assert_eq!(
        parse_event(b"\x1B[15;2~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(5)).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::F(5)).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15;2~");

    assert_eq!(
        parse_event(b"\x1B[15;1:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(5))
                .modifiers(KeyModifiers::NONE)
                .kind(KeyEventKind::Release,)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(5))
            .modifiers(KeyModifiers::NONE)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15;1:3~");

    assert_eq!(
        parse_event(b"\x1B[15;2:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(5))
                .modifiers(KeyModifiers::SHIFT)
                .kind(KeyEventKind::Release,)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(
        KeyEvent::new(KeyCode::F(5))
            .modifiers(KeyModifiers::SHIFT)
            .kind(KeyEventKind::Release),
    )
    .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
    .unwrap();
    assert_eq!(buf[..written], *b"\x1B[15;2:3~");

    assert_eq!(
        parse_event(b"\x1B[57376u").unwrap(),
        Some(Event::Key(KeyCode::F(13).into())),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyCode::F(13).into())
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57376u");

    assert_eq!(
        parse_event(b"\x1B[57376;2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(13)).modifiers(KeyModifiers::SHIFT)
        )),
    );
    let mut buf = [0; 16];
    let written = Event::Key(KeyEvent::new(KeyCode::F(13)).modifiers(KeyModifiers::SHIFT))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57376;2u");
}

#[test]
fn test_parse_basic_csi_u_encoded_key_code_special_keys() {
    assert_eq!(
        parse_event(b"\x1B[13u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Enter).modifiers(KeyModifiers::empty())
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[27u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Esc).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[27u");

    assert_eq!(
        parse_event(b"\x1B[57358u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::CapsLock).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::CapsLock).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57358u");

    assert_eq!(
        parse_event(b"\x1B[57359u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::ScrollLock).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::ScrollLock).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57359u");

    assert_eq!(
        parse_event(b"\x1B[57360u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::NumLock).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::NumLock).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57360u");

    assert_eq!(
        parse_event(b"\x1B[57361u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PrintScreen).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PrintScreen).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57361u");

    assert_eq!(
        parse_event(b"\x1B[57362u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Pause).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Pause).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57362u");

    assert_eq!(
        parse_event(b"\x1B[57363u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Menu).modifiers(KeyModifiers::empty())
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Menu).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[57363u");

    assert_eq!(
        parse_event(b"\x1B[57376u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::F(13)).modifiers(KeyModifiers::empty())
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57428u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Media(MediaKeyCode::Play)).modifiers(KeyModifiers::empty())
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57441u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Shift,
                ModifierDirection::Left
            ))
            .modifiers(KeyModifiers::SHIFT,)
        )),
    );
}

#[test]
fn test_parse_csi_u_encoded_keypad_code() {
    assert_eq!(
        parse_event(b"\x1B[57399u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('0'))
                .modifiers(KeyModifiers::empty(),)
                .state(KeyEventState::KEYPAD,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57419u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Up)
                .modifiers(KeyModifiers::empty(),)
                .state(KeyEventState::KEYPAD,)
        )),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[97;1u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::empty(),)
        )),
    );
    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Kitty(KittyFlags::all()))
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[97u");

    assert_eq!(
        parse_event(b"\x1B[97;1:1u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::empty(),)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;5:1u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::CTRL,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;1:2u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a'))
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Repeat,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;1:3u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a'))
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release,)
        )),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_has_modifier_on_modifier_press() {
    assert_eq!(
        parse_event(b"\x1B[57449u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Alt,
                ModifierDirection::Right
            ))
            .modifiers(KeyModifiers::ALT,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57449;3:3u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Alt,
                ModifierDirection::Right
            ))
            .modifiers(KeyModifiers::ALT)
            .kind(KeyEventKind::Release,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57450u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Super,
                ModifierDirection::Right
            ))
            .modifiers(KeyModifiers::SUPER,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57451u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Hyper,
                ModifierDirection::Right
            ))
            .modifiers(KeyModifiers::HYPER,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[57452u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Modifier(
                ModifierKeyCode::Meta,
                ModifierDirection::Right
            ))
            .modifiers(KeyModifiers::META,)
        )),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_extra_modifiers() {
    assert_eq!(
        parse_event(b"\x1B[97;9u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::SUPER)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;17u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::HYPER,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[97;33u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a')).modifiers(KeyModifiers::META,)
        )),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_extra_state() {
    assert_eq!(
        parse_event(b"\x1B[97;65u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('a'))
                .modifiers(KeyModifiers::empty(),)
                .state(KeyEventState::CAPS_LOCK,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[49;129u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('1'))
                .modifiers(KeyModifiers::empty(),)
                .state(KeyEventState::NUM_LOCK,)
        )),
    );
}

#[test]
fn test_parse_csi_u_with_shifted_keycode() {
    assert_eq!(
        // A-S-9 is equivalent to A-(
        parse_event(b"\x1B[57:40;4u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('(')).modifiers(KeyModifiers::ALT,)
        )),
    );
    assert_eq!(
        // A-S-minus is equivalent to A-_
        parse_event(b"\x1B[45:95;4u").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Char('_')).modifiers(KeyModifiers::ALT,)
        )),
    );
}

#[test]
fn test_parse_csi_special_key_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[;1:3B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[1;1:3B").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::Down)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release,)
        )),
    );
}

#[test]
fn test_parse_csi_numbered_escape_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[5~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::empty(),)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[5;1:1~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::empty(),)
        )),
    );

    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageUp).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[5~");

    assert_eq!(
        parse_event(b"\x1B[5;1:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageUp)
                .modifiers(KeyModifiers::empty())
                .kind(KeyEventKind::Release,)
        )),
    );
    assert_eq!(
        parse_event(b"\x1B[6;5:3~").unwrap(),
        Some(Event::Key(
            KeyEvent::new(KeyCode::PageDown)
                .modifiers(KeyModifiers::CTRL)
                .kind(KeyEventKind::Release,)
        )),
    );

    let mut buf = [0; 8];
    let written = Event::Key(KeyEvent::new(KeyCode::PageDown).modifiers(KeyModifiers::empty()))
        .encode(&mut buf, Encoding::Xterm)
        .unwrap();
    assert_eq!(buf[..written], *b"\x1B[6~");
}
