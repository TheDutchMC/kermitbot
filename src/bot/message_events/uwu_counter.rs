use mysql::{PooledConn, Row, params};
use mysql::prelude::Queryable;
use serenity::model::channel::Message;
use crate::bot::Handler;
use log::debug;

pub(super) async fn handle(handler: &Handler, message: &Message) -> anyhow::Result<()> {
    let lower = message.content.to_lowercase();

    if !lower.contains("uwu") && !lower.contains("owo") {
        return Ok(());
    }

    debug!("Messgae container either 'uwu' or 'owo'");

    let mut conn = handler.data.pool.get_conn()?;
    let user_id = message.author.id.0;
    let nickname = message.author.nick_in(&handler.data.http, message.guild_id.unwrap()).await; // Unwrap on Guild ID is safe as it is checked in ther general message event handler

    let nickname = match nickname {
        Some(n) => n,
        None => message.author.name.clone()
    };

    if user_exists(&mut conn, user_id)? {
        debug!("User {} exists in uwu_counter", &nickname);
        increment_count(&mut conn, user_id)?;
    } else {
        debug!("User {} does not yet exist in uwu_counter", &nickname);
        create_user(&mut conn, user_id)?;
    }

    if nickname_exists(&mut conn, user_id)? {
        debug!("User {} exists in user_nicknames", &nickname);
        update_username(&mut conn, user_id, nickname)?;
    } else {
        debug!("User {} does not yet exist in user_nicknames", &nickname);
        create_nickname(&mut conn, user_id, nickname)?;
    }

    Ok(())
}

fn increment_count(conn: &mut PooledConn, user_id: u64) -> anyhow::Result<()> {
    conn.exec_drop("UPDATE uwu_counter SET count = count + 1 WHERE user_id = :user_id", params! {
        "user_id" => &user_id
    })?;

    Ok(())
}

fn update_username<S: AsRef<str>>(conn: &mut PooledConn, user_id: u64, nickname: S) -> anyhow::Result<()> {
    conn.exec_drop("UPDATE user_nicknames SET nickname = :nickname WHERE user_id = :user_id", params! {
        "nickname" => nickname.as_ref(),
        "user_id" => user_id
    })?;

    Ok(())
}

fn nickname_exists(conn: &mut PooledConn, user_id: u64) -> anyhow::Result<bool> {
    let maybe_row: Option<Row> = conn.exec_first("SELECT 1 FROM user_nicknames WHERE user_id = :user_id", params! {
        "user_id" => user_id
    })?;

    Ok(maybe_row.is_some())
}

fn create_nickname<S: AsRef<str>>(conn: &mut PooledConn, user_id: u64, nickname: S) -> anyhow::Result<()> {
    conn.exec_drop("INSERT INTO user_nicknames (user_id, nickname) VALUES (:user_id, :nickname)", params! {
        "user_id" => user_id,
        "nickname" => nickname.as_ref()
    })?;

    Ok(())
}

fn create_user(conn: &mut PooledConn, user_id: u64) -> anyhow::Result<()> {
    conn.exec_drop("INSERT INTO uwu_counter (user_id) VALUES (:user_id)", params! {
        "user_id" => user_id
    })?;

    Ok(())
}

fn user_exists(conn: &mut PooledConn, user_id: u64) -> anyhow::Result<bool> {
    let maybe_row: Option<Row> = conn.exec_first("SELECT 1 FROM uwu_counter WHERE user_id = :user_id", params! {
        "user_id" => &user_id
    })?;

    Ok(maybe_row.is_some())
}