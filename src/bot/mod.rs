mod message_events;

use std::sync::Arc;
use log::warn;
use crate::AppData;
use serenity::{
    async_trait,
    prelude::{EventHandler, Context},
    model::prelude::{Ready, Message},
    framework::StandardFramework,
    client::Client
};

struct Handler {
    data: Arc<AppData>
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, message: Message) {
        match message_events::handle(&self, message).await {
            Ok(_) => {},
            Err(e) => {
                warn!("Unable to handle message event: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, _: Ready) {}
}

/// Initialize the Serenity framework.
/// This function should be called from a non-main thread as it is blocking
pub(crate) async fn start<S: AsRef<str>>(token: S, data: Arc<AppData>) -> anyhow::Result<()> {
    let framework = StandardFramework::new()
        .configure(|c| {
            c
                .ignore_bots(true)
                .ignore_webhooks(true)
        });

    let handler = Handler {
        data
    };

    let mut client = Client::builder(token.as_ref())
        .event_handler(handler)
        .framework(framework)
        .await?;

    client.start().await?;
    Ok(())
}