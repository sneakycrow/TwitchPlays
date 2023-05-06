use dioxus::prelude::*;

pub(crate) fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        },
        Button {}
    })
}

pub(crate) fn Button(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |event| {
                println!("Clicked! Event: {event:?}")
            },
            "Click me!"
        }
    })
}