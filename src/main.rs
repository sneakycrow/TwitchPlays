#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{event, Level};
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage::Privmsg;

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            "Hello, world!"
        },
        Button {}
    })
}

fn Button(cx: Scope) -> Element {
    cx.render(rsx! {
        button {
            onclick: move |event| {
                println!("Clicked! Event: {event:?}")
            },
            "Click me!"
        }
    })
}

async fn connect_to_chat() {
    let config = ClientConfig::default();

    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    // Create an instance of Analyzer for executing thresholds and triggers
    // Spawn an async instance that catches the messages
    let join_handle = tokio::spawn(async move {
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
                }
                _ => {}
            }
        }
    });

    client.join(String::from("zackrawrr")).unwrap();

    join_handle.await.unwrap();
}