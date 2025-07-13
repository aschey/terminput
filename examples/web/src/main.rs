use terminput::Event;
use terminput_web_sys::{
    to_terminput_key, to_terminput_mouse, to_terminput_mouse_drag, to_terminput_mouse_scroll,
    to_terminput_paste, to_terminput_resize,
};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{ClipboardEvent, Document, DragEvent, KeyboardEvent, MouseEvent, WheelEvent, window};

fn main() {
    console_error_panic_hook::set_once();

    let window = window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let on_mouse = Closure::<dyn Fn(MouseEvent)>::new({
        let document = document.clone();
        move |e| {
            let mouse_event = to_terminput_mouse(e);
            append_text(&document, &format!("{mouse_event:?}\n"));
        }
    });
    let mouse_ref = Some(on_mouse.as_ref().unchecked_ref());
    document.set_onmousedown(mouse_ref);
    document.set_onmouseup(mouse_ref);

    // disabling this because it spams the window, but it does work
    // document.set_onmousemove(mouse_ref);

    on_mouse.forget();

    let on_drag = Closure::<dyn Fn(DragEvent)>::new({
        let document = document.clone();
        move |e| {
            let drag_event = to_terminput_mouse_drag(e);
            append_text(&document, &format!("{drag_event:?}\n"));
        }
    });
    // Note: this only fires if you're dragging a text node, not an empty space on the document
    body.set_ondragover(Some(on_drag.as_ref().unchecked_ref()));
    on_drag.forget();

    let on_key = Closure::<dyn Fn(KeyboardEvent)>::new({
        let document = document.clone();
        move |e| {
            let key_event = to_terminput_key(e);
            append_text(&document, &format!("{key_event:?}\n"));
        }
    });
    let key_ref = Some(on_key.as_ref().unchecked_ref());
    document.set_onkeydown(key_ref);
    document.set_onkeyup(key_ref);
    on_key.forget();

    let on_paste = Closure::<dyn Fn(ClipboardEvent)>::new({
        let document = document.clone();
        move |e| {
            let clipboard_event = to_terminput_paste(e);
            append_text(&document, &format!("{clipboard_event:?}\n"));
        }
    });
    document.set_onpaste(Some(on_paste.as_ref().unchecked_ref()));
    on_paste.forget();

    let on_wheel = Closure::<dyn Fn(WheelEvent)>::new({
        let document = document.clone();
        move |e| {
            let scroll_event = to_terminput_mouse_scroll(e);
            append_text(&document, &format!("{scroll_event:?}\n"));
        }
    });
    document.set_onwheel(Some(on_wheel.as_ref().unchecked_ref()));
    on_wheel.forget();

    let on_resize = Closure::<dyn Fn()>::new({
        let window = window.clone();
        let document = document.clone();
        move || {
            let resize_event = to_terminput_resize(&window);
            append_text(&document, &format!("{resize_event:?}\n"));
        }
    });
    // Note that this only fires when set on the window object
    window.set_onresize(Some(on_resize.as_ref().unchecked_ref()));
    on_resize.forget();

    let on_focus = Closure::<dyn Fn()>::new({
        let document = document.clone();
        move || {
            let focus = Event::FocusGained;
            append_text(&document, &format!("{focus:?}\n"));
        }
    });
    window.set_onfocus(Some(on_focus.as_ref().unchecked_ref()));
    on_focus.forget();

    let on_blur = Closure::<dyn Fn()>::new({
        let document = document.clone();
        move || {
            let focus = Event::FocusLost;
            append_text(&document, &format!("{focus:?}\n"));
        }
    });
    window.set_onblur(Some(on_blur.as_ref().unchecked_ref()));
    on_blur.forget();
}

fn append_text(document: &Document, text: &str) {
    let text_node = document.create_text_node(text);
    document.body().unwrap().append_child(&text_node).unwrap();
}
