mod uwu_counter;

use serenity::model::prelude::Message;
use super::Handler;
use log::debug;

pub(super) async fn handle(handler: &Handler, message: Message) -> anyhow::Result<()> {
    let recv_guild_id = if let Some(recv_guild_id) = message.guild_id {
        recv_guild_id.0.to_string()
    } else {
        return Ok(());
    };

    if recv_guild_id.ne(&handler.data.env.guild_id) {
        return Ok(());
    }

    debug!("Got message event for the selected guild");

    uwu_counter::handle(handler, &message).await?;

    Ok(())
}