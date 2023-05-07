#![allow(non_snake_case)]


use tracing_subscriber::EnvFilter;

use crate::components::App;

mod components;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("chat_messages=info".parse().unwrap())
                .add_directive("twitch_irc=off".parse().unwrap()),
        )
        .with_target(true)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339()).init();
    dioxus_desktop::launch(App);
}