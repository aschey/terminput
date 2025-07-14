#![deny(missing_docs, clippy::unwrap_used)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../README.md")]

use terminput::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierDirection, ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind, ScrollDirection,
    UnsupportedEvent,
};
use web_sys::wasm_bindgen::JsValue;

/// Converts the [`web_sys`] [`MouseEvent`](web_sys::MouseEvent) to a terminput [`MouseEvent`].
pub fn to_terminput_mouse(
    mouse_event: web_sys::MouseEvent,
) -> Result<MouseEvent, UnsupportedEvent> {
    let event_kind = mouse_event.type_();
    let mouse_button = to_terminput_mouse_button(mouse_event.button());
    Ok(MouseEvent {
        kind: to_terminput_mouse_kind(event_kind.as_str(), mouse_button)?,
        modifiers: to_terminput_mouse_modifiers(&mouse_event),
        column: mouse_event.client_x() as u16,
        row: mouse_event.client_y() as u16,
    })
}

/// Converts the [`web_sys`] [`DragEvent`](web_sys::DragEvent) to a terminput [`MouseEvent`].
///
/// **NOTE:** this should be used with the `dragover` event to ensure the row/column values are
/// populated.
pub fn to_terminput_mouse_drag(drag_event: web_sys::DragEvent) -> MouseEvent {
    let mouse_button = to_terminput_mouse_button(drag_event.button());

    MouseEvent {
        kind: MouseEventKind::Drag(mouse_button),
        modifiers: to_terminput_drag_modifiers(&drag_event),
        column: drag_event.client_x() as u16,
        row: drag_event.client_y() as u16,
    }
}

/// Converts the [`web_sys`] [`WheelEvent`](`web_sys::WheelEvent`) to a terminput [`MouseEvent`].
pub fn to_terminput_mouse_scroll(event: web_sys::WheelEvent) -> MouseEvent {
    let direction = to_terminput_scroll_direction(&event);
    MouseEvent {
        kind: MouseEventKind::Scroll(direction),
        modifiers: KeyModifiers::empty(),
        column: event.client_x() as u16,
        row: event.client_y() as u16,
    }
}

/// Converts the [`web_sys`] [`KeyboardEvent`](`web_sys::KeyboardEvent`) to a terminput
/// [`KeyEvent`].
pub fn to_terminput_key(key_event: web_sys::KeyboardEvent) -> Result<KeyEvent, UnsupportedEvent> {
    Ok(KeyEvent {
        code: to_terminput_key_code(
            &key_event.key(),
            to_terminput_modifier_direction(key_event.location()),
        )?,
        modifiers: to_terminput_key_modifiers(&key_event),
        state: key_state_to_terminput(&key_event),
        kind: to_terminput_key_kind(&key_event),
    })
}

/// Converts the [`web_sys`] [`ClipboardEvent`](`web_sys::ClipboardEvent`) to a terminput paste
/// [`Event`].
pub fn to_terminput_paste(
    clipboard_event: web_sys::ClipboardEvent,
) -> Result<Event, UnsupportedEvent> {
    if let Some(data) = clipboard_event.clipboard_data() {
        let content = data
            .get_data("text")
            .map_err(|e| map_js_error("failed to read clipboard data: ", e))?;
        Ok(Event::Paste(content))
    } else {
        Err(UnsupportedEvent("no clipboard data available".to_string()))
    }
}

/// Converts the [`web_sys`] window size to a terminput resize [`Event`].
pub fn to_terminput_resize(window: &web_sys::Window) -> Result<Event, UnsupportedEvent> {
    let height = window
        .inner_height()
        .map_err(|e| map_js_error("failed to read height: ", e))?;
    let width = window
        .inner_width()
        .map_err(|e| map_js_error("failed to read width: ", e))?;
    let height = height.as_f64().unwrap_or_default();
    let width = width.as_f64().unwrap_or_default();
    Ok(Event::Resize {
        rows: height as u32,
        cols: width as u32,
    })
}

fn map_js_error(message: &str, error: JsValue) -> UnsupportedEvent {
    UnsupportedEvent(message.to_string() + &error.as_string().unwrap_or_default())
}

fn to_terminput_scroll_direction(event: &web_sys::WheelEvent) -> ScrollDirection {
    if event.delta_x() > 0.0 {
        ScrollDirection::Right
    } else if event.delta_x() < 0.0 {
        ScrollDirection::Left
    } else if event.delta_y() > 0.0 {
        ScrollDirection::Down
    } else {
        ScrollDirection::Up
    }
}

fn to_terminput_mouse_kind(
    event_kind: &str,
    mouse_button: MouseButton,
) -> Result<MouseEventKind, UnsupportedEvent> {
    Ok(match event_kind {
        "mousemove" => MouseEventKind::Moved,
        "mousedown" => MouseEventKind::Down(mouse_button),
        "mouseup" => MouseEventKind::Up(mouse_button),
        kind => return Err(UnsupportedEvent(kind.to_string())),
    })
}

fn to_terminput_mouse_button(button: i16) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => MouseButton::Unknown,
    }
}

fn to_terminput_mouse_modifiers(event: &web_sys::MouseEvent) -> KeyModifiers {
    let mut modifiers = KeyModifiers::empty();
    if event.ctrl_key() {
        modifiers |= KeyModifiers::CTRL;
    }
    if event.shift_key() {
        modifiers |= KeyModifiers::SHIFT;
    }
    if event.alt_key() {
        modifiers |= KeyModifiers::ALT;
    }
    if event.meta_key() {
        modifiers |= KeyModifiers::META;
    }

    modifiers
}

fn to_terminput_drag_modifiers(event: &web_sys::DragEvent) -> KeyModifiers {
    let mut modifiers = KeyModifiers::empty();
    if event.ctrl_key() {
        modifiers |= KeyModifiers::CTRL;
    }
    if event.shift_key() {
        modifiers |= KeyModifiers::SHIFT;
    }
    if event.alt_key() {
        modifiers |= KeyModifiers::ALT;
    }
    if event.meta_key() {
        modifiers |= KeyModifiers::META;
    }

    modifiers
}

fn to_terminput_key_kind(key_event: &web_sys::KeyboardEvent) -> KeyEventKind {
    if key_event.repeat() {
        KeyEventKind::Repeat
    } else if key_event.type_() == "keyup" {
        KeyEventKind::Release
    } else {
        KeyEventKind::Press
    }
}

fn to_terminput_key_modifiers(event: &web_sys::KeyboardEvent) -> KeyModifiers {
    let mut modifiers = KeyModifiers::empty();
    if event.ctrl_key() {
        modifiers |= KeyModifiers::CTRL;
    }
    if event.shift_key() {
        modifiers |= KeyModifiers::SHIFT;
    }
    if event.alt_key() {
        modifiers |= KeyModifiers::ALT;
    }
    if event.meta_key() {
        modifiers |= KeyModifiers::META;
    }

    modifiers
}

fn to_terminput_modifier_direction(location: u32) -> ModifierDirection {
    match location {
        1 => ModifierDirection::Left,
        2 => ModifierDirection::Right,
        _ => ModifierDirection::Unknown,
    }
}

fn key_state_to_terminput(event: &web_sys::KeyboardEvent) -> KeyEventState {
    let mut state = KeyEventState::empty();
    if event.location() == 3 {
        state |= KeyEventState::KEYPAD;
    }

    if event.get_modifier_state("CapsLock") {
        state |= KeyEventState::CAPS_LOCK;
    }
    if event.get_modifier_state("NumLock") {
        state |= KeyEventState::NUM_LOCK;
    }

    state
}

fn to_terminput_key_code(
    key: &str,
    direction: ModifierDirection,
) -> Result<KeyCode, UnsupportedEvent> {
    let key = key.to_ascii_lowercase();
    if key.len() == 1 {
        let key_char = key.chars().next().expect("length checked");
        if key_char.is_alphanumeric() {
            return Ok(KeyCode::Char(key_char));
        }
    }
    Ok(match key.as_str() {
        "f1" => KeyCode::F(1),
        "f2" => KeyCode::F(2),
        "f3" => KeyCode::F(3),
        "f4" => KeyCode::F(4),
        "f5" => KeyCode::F(5),
        "f6" => KeyCode::F(6),
        "f7" => KeyCode::F(7),
        "f8" => KeyCode::F(8),
        "f9" => KeyCode::F(9),
        "f10" => KeyCode::F(10),
        "f11" => KeyCode::F(11),
        "f12" => KeyCode::F(12),
        "backspace" => KeyCode::Backspace,
        "enter" => KeyCode::Enter,
        "arrowleft" => KeyCode::Left,
        "arrowright" => KeyCode::Right,
        "arrowup" => KeyCode::Up,
        "arrowdown" => KeyCode::Down,
        "tab" => KeyCode::Tab,
        "delete" => KeyCode::Delete,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "capslock" => KeyCode::CapsLock,
        "scrolllock" => KeyCode::ScrollLock,
        "numlock" => KeyCode::NumLock,
        "printscreen" => KeyCode::PrintScreen,
        "alt" => KeyCode::Modifier(ModifierKeyCode::Alt, direction),
        "control" => KeyCode::Modifier(ModifierKeyCode::Control, direction),
        "hyper" => KeyCode::Modifier(ModifierKeyCode::Hyper, direction),
        "meta" => KeyCode::Modifier(ModifierKeyCode::Meta, direction),
        "super" => KeyCode::Modifier(ModifierKeyCode::Super, direction),
        "shift" => KeyCode::Modifier(ModifierKeyCode::Shift, direction),
        "mediaplay" => KeyCode::Media(MediaKeyCode::Play),
        "mediapause" => KeyCode::Media(MediaKeyCode::Pause),
        "mediaplaypause" => KeyCode::Media(MediaKeyCode::PlayPause),
        "mediastop" => KeyCode::Media(MediaKeyCode::Stop),
        "mediatracknext" => KeyCode::Media(MediaKeyCode::TrackNext),
        "mediatrackprevious" => KeyCode::Media(MediaKeyCode::TrackPrevious),
        "mediafastforward" => KeyCode::Media(MediaKeyCode::FastForward),
        "mediarewind" => KeyCode::Media(MediaKeyCode::Rewind),
        "mediarecord" => KeyCode::Media(MediaKeyCode::Record),
        key => return Err(UnsupportedEvent(key.to_string())),
    })
}
