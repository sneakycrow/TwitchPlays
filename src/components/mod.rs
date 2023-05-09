use dioxus::prelude::*;
use tracing::{event, Level};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage::Privmsg;

pub(crate) fn App(cx: Scope) -> Element {
    let config = ClientConfig::default();
    let mut current_message = use_state(cx, || "no message".to_string());
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    let chat: &Coroutine<()> = use_coroutine(cx, |rx| {
        let current_message = current_message.to_owned();
        async move {
            // Create an instance of Analyzer for executing thresholds and triggers
            // Spawn an async instance that catches the messages
            client.join(String::from("criken")).unwrap();
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    Privmsg(msg) => {
                        event!(
                            target: "chat_messages",
                            Level::INFO,
                            channel = msg.channel_login,
                            sender = msg.sender.name,
                            text = msg.message_text,
                            server_timestamp = msg.server_timestamp.to_string(),
                            id = msg.message_id,
                        );
                        let updated_message = msg.message_text;
                        current_message.set(updated_message.to_owned());
                    }
                    _ => {}
                }
            }
        }
    });
    cx.render(rsx! {
        div {
            "{current_message}"
        },
        Button {},
        if *current_message.get() == "OMEGALUL" {
            cx.render(rsx! {
                div {
                    "THIS IS ANOTHER DIV"
                }
            })
        }
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