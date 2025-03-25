use terminput::KeyCode::*;
use terminput::{
    ALT, CAPS_LOCK, CTRL, KeyEvent, KeyEventState, KeyModifiers, NUM_LOCK, key, modifiers, states,
};

#[test]
fn test_match() {
    let key_event = KeyEvent::new(Char('c'));
    assert!(matches!(key_event, key!(Char('c'))));
    assert!(matches!(key_event, key!(KeyModifiers::NONE, Char('c'))));
    assert!(matches!(
        key_event,
        key!(KeyEventState::NONE, KeyModifiers::NONE, Char('c'))
    ));
    assert!(matches!(
        key_event,
        key!(KeyEventState::NONE, KeyModifiers::NONE, Char('c'))
    ));
}

#[test]
fn test_match_with_state() {
    let key_event = KeyEvent::new(Char('c')).state(KeyEventState::CAPS_LOCK);
    assert!(matches!(key_event, key!(Char('c'))));
    assert!(matches!(key_event, key!(KeyModifiers::NONE, Char('c'))));
    assert!(matches!(
        key_event,
        key!(KeyEventState::CAPS_LOCK, KeyModifiers::NONE, Char('c'))
    ));
}

#[test]
fn test_match_with_modifiers() {
    let key_event = KeyEvent::new(Char('c')).modifiers(CTRL);
    assert!(!matches!(key_event, key!(Char('c'))));
    assert!(matches!(key_event, key!(CTRL, Char('c'))));
    assert!(matches!(key_event, key!(CTRL | ALT, Char('c'))));
    assert!(matches!(
        key_event,
        key!(KeyEventState::NONE, CTRL, Char('c'))
    ));
}

#[test]
fn test_match_with_compound_modifiers() {
    let key_event = KeyEvent::new(Char('c')).modifiers(CTRL | ALT);
    const CTRL_ALT: KeyModifiers = modifiers!(CTRL, ALT);
    assert!(!matches!(key_event, key!(Char('c'))));
    assert!(!matches!(key_event, key!(CTRL, Char('c'))));
    assert!(matches!(key_event, key!(CTRL_ALT, Char('c'))));
    assert!(matches!(
        key_event,
        key!(KeyEventState::NONE, CTRL_ALT, Char('c'))
    ));
}

#[test]
fn test_match_with_compound_state() {
    let key_event = KeyEvent::new(Char('c')).state(CAPS_LOCK | NUM_LOCK);
    const CAPS_NUM_LOCK: KeyEventState = states!(CAPS_LOCK, NUM_LOCK);
    assert!(matches!(key_event, key!(Char('c'))));
    assert!(matches!(
        key_event,
        key!(CAPS_NUM_LOCK, KeyModifiers::NONE, Char('c'))
    ));
}
