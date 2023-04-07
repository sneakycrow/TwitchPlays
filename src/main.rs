// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use tauri::Manager;
use tracing::{event, info, Level};
use tracing_subscriber::EnvFilter;
use twitch_irc::{ClientConfig, SecureTCPTransport, TwitchIRCClient};
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::message::ServerMessage::Privmsg;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
// A function that sends a message from Rust to JavaScript via a Tauri Event
fn send_message<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    info!(?message, "send_message");
    manager
        .emit_all("send_message", message)
        .unwrap();
}


fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("chat_messages=info".parse().unwrap())
                .add_directive("twitch_irc=off".parse().unwrap()),
        )
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339()).init();
    tauri::Builder::default()
        .setup(|app| {
            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
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
                                send_message(msg.message_text, &app_handle);
                            }
                            _ => {}
                        }
                    }
                });

                client.join(String::from("moistcr1tikal")).unwrap();

                join_handle.await.unwrap();
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
