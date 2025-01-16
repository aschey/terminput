use super::*;

#[test]
fn test_esc_key() {
    assert_eq!(
        parse_event(b"\x1B").unwrap(),
        Some(Event::Key(KeyCode::Esc.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Esc.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B"
    );

    assert_eq!(
        parse_event(b"\x1B\x1B").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::ALT))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B\x1B"
    );

    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B"
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Esc,
            KeyModifiers::ALT | KeyModifiers::CTRL
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B\x1B"
    );
}

#[test]
fn test_backspace() {
    assert_eq!(
        parse_event(b"\x7F").unwrap(),
        Some(Event::Key(KeyCode::Backspace.into()))
    );
    assert_eq!(
        Event::Key(KeyCode::Backspace.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x7F"
    );

    assert_eq!(
        parse_event(b"\x1B\x7F").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Backspace,
            KeyModifiers::ALT
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B\x7F",
    );

    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x08",
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Backspace,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B\x08",
    );
}

#[test]
fn test_focus_gained() {
    assert_eq!(parse_event(b"\x1B[I").unwrap(), Some(Event::FocusGained));
    assert_eq!(Event::FocusGained.to_escape_sequence().unwrap(), b"\x1B[I");
    assert_eq!(
        Event::FocusGained.to_kitty_escape_sequence().unwrap(),
        b"\x1B[I"
    );
}

#[test]
fn test_focus_lost() {
    assert_eq!(parse_event(b"\x1B[O").unwrap(), Some(Event::FocusLost));
    assert_eq!(Event::FocusLost.to_escape_sequence().unwrap(), b"\x1B[O");
    assert_eq!(
        Event::FocusLost.to_kitty_escape_sequence().unwrap(),
        b"\x1B[O"
    );
}

#[test]
fn test_enter() {
    assert_eq!(
        parse_event(b"\r").unwrap(),
        Some(Event::Key(KeyCode::Enter.into()))
    );
    assert_eq!(
        Event::Key(KeyCode::Enter.into())
            .to_escape_sequence()
            .unwrap(),
        b"\r"
    );

    assert_eq!(
        parse_event(b"\x1B\r").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B\r"
    );
}

#[test]
fn test_alt_key() {
    assert_eq!(
        parse_event(b"\x1Bc").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::ALT
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1Bc"
    );
}

#[test]
fn test_ctrl_key() {
    assert_eq!(
        parse_event(b"\x03").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::CTRL
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x03"
    );
}

#[test]
fn test_alt_shift() {
    assert_eq!(
        parse_event(b"\x1BH").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('H'),
            KeyModifiers::ALT | KeyModifiers::SHIFT
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Char('H'),
            KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1BH"
    )
}

#[test]
fn test_ctrl_alt() {
    assert_eq!(
        parse_event(b"\x1B\x14").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('t'),
            KeyModifiers::ALT | KeyModifiers::CTRL
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Char('t'),
            KeyModifiers::ALT | KeyModifiers::CTRL
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B\x14"
    );
}

#[test]
fn test_home_key() {
    assert_eq!(
        parse_event(b"\x1B[H").unwrap(),
        Some(Event::Key(KeyCode::Home.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Home.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[H"
    );

    assert_eq!(
        parse_event(b"\x1B[2H").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2H").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2H"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5H").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5H"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3H").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Home, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3H"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7H").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7H"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8H").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Home,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8H"
    );
}

#[test]
fn test_end_key() {
    assert_eq!(
        parse_event(b"\x1B[F").unwrap(),
        Some(Event::Key(KeyCode::End.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::End.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[F"
    );

    assert_eq!(
        parse_event(b"\x1B[2F").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::SHIFT)))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2F").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::SHIFT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2F"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5F").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5F"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3F").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::End, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3F"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7F").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::End,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::End,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7F"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8F").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::End,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::End,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8F"
    );
}

#[test]
fn test_page_up() {
    assert_eq!(
        parse_event(b"\x1B[5~").unwrap(),
        Some(Event::Key(KeyCode::PageUp.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::PageUp.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[5~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;2~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[5;2~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;5~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::CTRL
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[5;5~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;3~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageUp, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[5;3~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;7~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[5;7~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::PageUp,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[5;8~"
    );
}

#[test]
fn test_page_down() {
    assert_eq!(
        parse_event(b"\x1B[6~").unwrap(),
        Some(Event::Key(KeyCode::PageDown.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::PageDown.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[6~"
    );

    assert_eq!(
        parse_event(b"\x1B[6;2~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[6;2~"
    );

    assert_eq!(
        parse_event(b"\x1B[6;5~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::CTRL
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[6;5~"
    );

    assert_eq!(
        parse_event(b"\x1B[6;3~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PageDown, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[6;3~"
    );

    assert_eq!(
        parse_event(b"\x1B[6;7~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[6;7~"
    );

    assert_eq!(
        parse_event(b"\x1B[6;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::PageDown,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[6;8~"
    );
}

#[test]
fn test_left_arrow() {
    assert_eq!(
        parse_event(b"\x1B[D").unwrap(),
        Some(Event::Key(KeyCode::Left.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Left.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[D"
    );

    assert_eq!(
        parse_event(b"\x1B[2D").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2D").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2D"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5D").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5D"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3D").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Left, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3D"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7D").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7D"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8D").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8D"
    );
}

#[test]
fn test_right_arrow() {
    assert_eq!(
        parse_event(b"\x1B[C").unwrap(),
        Some(Event::Key(KeyCode::Right.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Right.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[C"
    );

    assert_eq!(
        parse_event(b"\x1B[2C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2C"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::CTRL
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5C"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3C").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Right, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3C"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7C"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Right,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8C"
    );
}

#[test]
fn test_up_arrow() {
    assert_eq!(
        parse_event(b"\x1B[A").unwrap(),
        Some(Event::Key(KeyCode::Up.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Up.into()).to_escape_sequence().unwrap(),
        b"\x1B[A"
    );

    assert_eq!(
        parse_event(b"\x1B[2A").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT)))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2A").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2A"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5A").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5A"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3A").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Up, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3A"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7A").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Up,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Up,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7A"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8A").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Up,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Up,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8A"
    );
}

#[test]
fn test_down_arrow() {
    assert_eq!(
        parse_event(b"\x1B[B").unwrap(),
        Some(Event::Key(KeyCode::Down.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Down.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[B"
    );

    assert_eq!(
        parse_event(b"\x1B[2B").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        parse_event(b"\x1B[1;2B").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;2B"
    );

    assert_eq!(
        parse_event(b"\x1B[1;5B").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5B"
    );

    assert_eq!(
        parse_event(b"\x1B[1;3B").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Down, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;3B"
    );

    assert_eq!(
        parse_event(b"\x1B[1;7B").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;7B"
    );

    assert_eq!(
        parse_event(b"\x1B[1;8B").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Down,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8B"
    );
}

#[test]
fn test_delete() {
    assert_eq!(
        parse_event(b"\x1B[3~").unwrap(),
        Some(Event::Key(KeyCode::Delete.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Delete.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[3~"
    );

    assert_eq!(
        parse_event(b"\x1B[3;2~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[3;2~"
    );

    assert_eq!(
        parse_event(b"\x1B[3;5~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::CTRL
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[3;5~"
    );

    assert_eq!(
        parse_event(b"\x1B[3;3~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Delete, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[3;3~"
    );

    assert_eq!(
        parse_event(b"\x1B[3;7~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[3;7~"
    );

    assert_eq!(
        parse_event(b"\x1B[3;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[3;8~"
    );
}

#[test]
fn test_insert() {
    assert_eq!(
        parse_event(b"\x1B[2~").unwrap(),
        Some(Event::Key(KeyCode::Insert.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Insert.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[2~"
    );

    assert_eq!(
        parse_event(b"\x1B[2;2~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Insert, KeyModifiers::SHIFT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[2;2~"
    );

    assert_eq!(
        parse_event(b"\x1B[2;5~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::CTRL
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Insert, KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[2;5~"
    );

    assert_eq!(
        parse_event(b"\x1B[2;3~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Insert, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[2;3~"
    );

    assert_eq!(
        parse_event(b"\x1B[2;7~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::CTRL | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::CTRL | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[2;7~"
    );

    assert_eq!(
        parse_event(b"\x1B[2;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::Insert,
            KeyModifiers::CTRL | KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[2;8~"
    );
}

#[test]
fn test_parse_event_subsequent_calls() {
    // The main purpose of this test is to check if we're passing
    // correct slice to other parse_ functions.

    // parse_csi_cursor_position
    // assert_eq!(
    //     parse_event(b"\x1B[20;10R").unwrap(),
    //     Some(InternalEvent::CursorPosition(9, 19))
    // );

    // parse_csi

    assert_eq!(
        parse_event(b"\x1B[C").unwrap(),
        Some(Event::Key(KeyCode::Right.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::Right.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[C"
    );

    assert_eq!(
        parse_event(b"\x1B[A").unwrap(),
        Some(Event::Key(KeyCode::Up.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::Up.into()).to_escape_sequence().unwrap(),
        b"\x1B[A"
    );

    assert_eq!(
        parse_event(b"\x1B[B").unwrap(),
        Some(Event::Key(KeyCode::Down.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::Down.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[B"
    );

    assert_eq!(
        parse_event(b"\x1B[H").unwrap(),
        Some(Event::Key(KeyCode::Home.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::Home.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[H"
    );

    assert_eq!(
        parse_event(b"\x1B[F").unwrap(),
        Some(Event::Key(KeyCode::End.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::End.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[F"
    );

    assert_eq!(
        parse_event(b"\x1B[2~").unwrap(),
        Some(Event::Key(KeyCode::Insert.into())),
    );

    assert_eq!(
        Event::Key(KeyCode::Insert.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[2~"
    );

    // parse_csi_bracketed_paste
    assert_eq!(
        parse_event(b"\x1B[200~on and on and on\x1B[201~").unwrap(),
        Some(Event::Paste("on and on and on".to_string())),
    );

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
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('Ž'),
            KeyModifiers::SHIFT
        ))),
    );
}

#[test]
fn test_parse_tab() {
    assert_eq!(
        parse_event(b"\t").unwrap(),
        Some(Event::Key(KeyCode::Tab.into())),
    );
    assert_eq!(
        Event::Key(KeyCode::Tab.into())
            .to_escape_sequence()
            .unwrap(),
        b"\t"
    );

    assert_eq!(
        parse_event(b"\x1B[Z").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::BackTab,
            KeyModifiers::SHIFT,
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        Event::Key(KeyCode::BackTab.into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[Z"
    );

    assert_eq!(
        parse_event(b"\x1B\x09").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Tab, KeyModifiers::ALT))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B\x09"
    );

    assert_eq!(
        parse_event(b"\x1B\x1B[Z").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::BackTab,
            KeyModifiers::ALT | KeyModifiers::SHIFT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::BackTab,
            KeyModifiers::ALT | KeyModifiers::SHIFT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B\x1B[Z"
    );
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
        Some(Event::Key(KeyEvent::new(
            KeyCode::Left,
            KeyModifiers::SHIFT
        ))),
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
        Some(Event::Key(KeyEvent::new(
            KeyCode::Delete,
            KeyModifiers::SHIFT
        ))),
    );
}

#[test]
fn test_parse_csi_bracketed_paste() {
    //
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
    assert_eq!(parse_csi(b"\x1B[O").unwrap(), Some(Event::FocusLost));
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
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty()
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<35;20;10M"
    );
    assert_eq!(
        parse_event(b"\x1B[<39;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 19,
            row: 9,
            modifiers: KeyModifiers::SHIFT
        }))
    );
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Moved,
            column: 19,
            row: 9,
            modifiers: KeyModifiers::SHIFT
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<39;20;10M"
    );

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
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<0;20;10M"
    );

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
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Up(MouseButton::Left),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<0;20;10m"
    );

    assert_eq!(
        parse_event(b"\x1B[<2;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Right),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<2;20;10M"
    );

    assert_eq!(
        parse_event(b"\x1B[<1;20;10M").unwrap(),
        Some(Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Middle),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        }))
    );
    assert_eq!(
        Event::Mouse(MouseEvent {
            kind: MouseEventKind::Down(MouseButton::Middle),
            column: 19,
            row: 9,
            modifiers: KeyModifiers::empty(),
        })
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[<1;20;10M"
    );
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
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('c'),
            KeyModifiers::empty()
        ))),
    );

    assert_eq!(
        Event::Key(KeyCode::Char('c').into())
            .to_escape_sequence()
            .unwrap(),
        b"c"
    );
}

#[test]
fn test_parse_char_event_uppercase() {
    assert_eq!(
        parse_event(b"C").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('C'),
            KeyModifiers::SHIFT
        ))),
    );

    assert_eq!(
        Event::Key(KeyCode::Char('C').into())
            .to_escape_sequence()
            .unwrap(),
        b"C"
    );
}

#[test]
fn test_parse_basic_csi_u_encoded_key_code() {
    assert_eq!(
        parse_event(b"\x1B[97u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;2u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('A'),
            KeyModifiers::SHIFT
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;7u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::ALT | KeyModifiers::CTRL
        ))),
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
    assert_eq!(
        Event::Key(KeyCode::F(1).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1BOP"
    );
    assert_eq!(
        parse_event(b"\x1B[1;5P").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(1), KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::F(1), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5P"
    );
    assert_eq!(
        parse_event(b"\x1B[1;8P").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(1),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::F(1),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8P"
    );

    assert_eq!(
        parse_event(b"\x1B[12~").unwrap(),
        Some(Event::Key(KeyCode::F(2).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOQ").unwrap(),
        Some(Event::Key(KeyCode::F(2).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(2).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1BOQ"
    );
    assert_eq!(
        parse_event(b"\x1B[1;5Q").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::F(2), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5Q"
    );
    assert_eq!(
        parse_event(b"\x1B[1;8Q").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(2),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::F(2),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8Q"
    );

    assert_eq!(
        parse_event(b"\x1B[13~").unwrap(),
        Some(Event::Key(KeyCode::F(3).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOR").unwrap(),
        Some(Event::Key(KeyCode::F(3).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(3).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1BOR"
    );
    assert_eq!(
        parse_event(b"\x1B[1;5R").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(3), KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::F(3), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5R"
    );
    assert_eq!(
        parse_event(b"\x1B[1;8R").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(3),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::F(3),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8R"
    );

    assert_eq!(
        parse_event(b"\x1B[14~").unwrap(),
        Some(Event::Key(KeyCode::F(4).into())),
    );
    assert_eq!(
        parse_event(b"\x1BOS").unwrap(),
        Some(Event::Key(KeyCode::F(4).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(4).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1BOS"
    );
    assert_eq!(
        parse_event(b"\x1B[1;5S").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(4), KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::F(4), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[1;5S"
    );
    assert_eq!(
        parse_event(b"\x1B[1;8S").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(4),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::F(4),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[1;8S"
    );

    assert_eq!(
        parse_event(b"\x1B[15~").unwrap(),
        Some(Event::Key(KeyCode::F(5).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(5).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[15~"
    );
    assert_eq!(
        parse_event(b"\x1B[15;5~").unwrap(),
        Some(Event::Key(KeyEvent::new(KeyCode::F(5), KeyModifiers::CTRL)))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::F(5), KeyModifiers::CTRL))
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[15;5~"
    );
    assert_eq!(
        parse_event(b"\x1B[15;8~").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(5),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        )))
    );
    assert_eq!(
        Event::Key(KeyEvent::new(
            KeyCode::F(5),
            KeyModifiers::CTRL | KeyModifiers::SHIFT | KeyModifiers::ALT
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[15;8~"
    );

    assert_eq!(
        parse_event(b"\x1B[17~").unwrap(),
        Some(Event::Key(KeyCode::F(6).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(6).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[17~"
    );

    assert_eq!(
        parse_event(b"\x1B[18~").unwrap(),
        Some(Event::Key(KeyCode::F(7).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(7).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[18~"
    );

    assert_eq!(
        parse_event(b"\x1B[19~").unwrap(),
        Some(Event::Key(KeyCode::F(8).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(8).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[19~"
    );

    assert_eq!(
        parse_event(b"\x1B[20~").unwrap(),
        Some(Event::Key(KeyCode::F(9).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(9).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[20~"
    );

    assert_eq!(
        parse_event(b"\x1B[21~").unwrap(),
        Some(Event::Key(KeyCode::F(10).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(10).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[21~"
    );

    assert_eq!(
        parse_event(b"\x1B[23~").unwrap(),
        Some(Event::Key(KeyCode::F(11).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(11).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[23~"
    );

    assert_eq!(
        parse_event(b"\x1B[24~").unwrap(),
        Some(Event::Key(KeyCode::F(12).into())),
    );
    assert_eq!(
        Event::Key(KeyCode::F(12).into())
            .to_escape_sequence()
            .unwrap(),
        b"\x1B[24~"
    );
}

#[test]
fn test_parse_basic_csi_u_encoded_key_code_special_keys() {
    assert_eq!(
        parse_event(b"\x1B[13u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Enter,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[27u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Esc,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[27u"
    );

    assert_eq!(
        parse_event(b"\x1B[57358u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::CapsLock,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::CapsLock, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57358u"
    );

    assert_eq!(
        parse_event(b"\x1B[57359u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::ScrollLock,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::ScrollLock, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57359u"
    );

    assert_eq!(
        parse_event(b"\x1B[57360u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::NumLock,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::NumLock, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57360u"
    );

    assert_eq!(
        parse_event(b"\x1B[57361u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::PrintScreen,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::PrintScreen, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57361u"
    );

    assert_eq!(
        parse_event(b"\x1B[57362u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Pause,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Pause, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57362u"
    );

    assert_eq!(
        parse_csi_u_encoded_key_code(b"\x1B[57363u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Menu,
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new(KeyCode::Menu, KeyModifiers::empty()))
            .to_kitty_escape_sequence()
            .unwrap(),
        b"\x1B[57363u"
    );

    assert_eq!(
        parse_event(b"\x1B[57376u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::F(13),
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57428u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Media(MediaKeyCode::Play),
            KeyModifiers::empty()
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57441u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Modifier(ModifierKeyCode::Shift, ModifierDirection::Left),
            KeyModifiers::SHIFT,
        ))),
    );
}

#[test]
fn test_parse_csi_u_encoded_keypad_code() {
    assert_eq!(
        parse_event(b"\x1B[57399u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('0'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
            KeyEventState::KEYPAD,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57419u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind_and_state(
            KeyCode::Up,
            KeyModifiers::empty(),
            KeyEventKind::Press,
            KeyEventState::KEYPAD,
        ))),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[97;1u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))
        .to_kitty_escape_sequence()
        .unwrap(),
        b"\x1B[97u"
    );

    assert_eq!(
        parse_event(b"\x1B[97;1:1u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;5:1u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::CTRL,
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;1:2u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Repeat,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;1:3u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Release,
        ))),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_has_modifier_on_modifier_press() {
    assert_eq!(
        parse_event(b"\x1B[57449u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right),
            KeyModifiers::ALT,
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57449;3:3u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Modifier(ModifierKeyCode::Alt, ModifierDirection::Right),
            KeyModifiers::ALT,
            KeyEventKind::Release,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57450u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Modifier(ModifierKeyCode::Super, ModifierDirection::Right),
            KeyModifiers::SUPER,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57451u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Modifier(ModifierKeyCode::Hyper, ModifierDirection::Right),
            KeyModifiers::HYPER,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[57452u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Modifier(ModifierKeyCode::Meta, ModifierDirection::Right),
            KeyModifiers::META,
        ))),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_extra_modifiers() {
    assert_eq!(
        parse_event(b"\x1B[97;9u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::SUPER
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;17u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::HYPER,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[97;33u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('a'),
            KeyModifiers::META,
        ))),
    );
}

#[test]
fn test_parse_csi_u_encoded_key_code_with_extra_state() {
    assert_eq!(
        parse_event(b"\x1B[97;65u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('a'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
            KeyEventState::CAPS_LOCK,
        ))),
    );
    assert_eq!(
        parse_csi_u_encoded_key_code(b"\x1B[49;129u").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind_and_state(
            KeyCode::Char('1'),
            KeyModifiers::empty(),
            KeyEventKind::Press,
            KeyEventState::NUM_LOCK,
        ))),
    );
}

#[test]
fn test_parse_csi_u_with_shifted_keycode() {
    assert_eq!(
        // A-S-9 is equivalent to A-(
        parse_event(b"\x1B[57:40;4u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('('),
            KeyModifiers::ALT,
        ))),
    );
    assert_eq!(
        // A-S-minus is equivalent to A-_
        parse_event(b"\x1B[45:95;4u").unwrap(),
        Some(Event::Key(KeyEvent::new(
            KeyCode::Char('_'),
            KeyModifiers::ALT,
        ))),
    );
}

#[test]
fn test_parse_csi_special_key_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[;1:3B").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Down,
            KeyModifiers::empty(),
            KeyEventKind::Release,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[1;1:3B").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::Down,
            KeyModifiers::empty(),
            KeyEventKind::Release,
        ))),
    );
}

#[test]
fn test_parse_csi_numbered_escape_code_with_types() {
    assert_eq!(
        parse_event(b"\x1B[5~").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageUp,
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[5;1:1~").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageUp,
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))),
    );

    assert_eq!(
        Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageUp,
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[5~"
    );

    assert_eq!(
        parse_event(b"\x1B[5;1:3~").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageUp,
            KeyModifiers::empty(),
            KeyEventKind::Release,
        ))),
    );
    assert_eq!(
        parse_event(b"\x1B[6;5:3~").unwrap(),
        Some(Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageDown,
            KeyModifiers::CTRL,
            KeyEventKind::Release,
        ))),
    );

    assert_eq!(
        Event::Key(KeyEvent::new_with_kind(
            KeyCode::PageDown,
            KeyModifiers::empty(),
            KeyEventKind::Press,
        ))
        .to_escape_sequence()
        .unwrap(),
        b"\x1B[6~"
    );
}
