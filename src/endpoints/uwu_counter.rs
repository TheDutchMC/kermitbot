use actix_web::{web, get, HttpResponse};
use crate::error::{HttpResult};
use serde::Serialize;
use mysql::prelude::Queryable;
use std::sync::Arc;
use mysql::{PooledConn, Row, params, Params};
use crate::AppData;

#[derive(Serialize)]
struct Response {
    users:  Vec<UserStatistic>
}

#[derive(Serialize)]
struct UserStatistic {
    user_id:    u64,
    count:      i32,
    name:       String,
}

#[get("/uwu-counter")]
pub async fn uwu_counter(data: web::Data<Arc<AppData>>) -> HttpResult {
    let mut conn = data.pool.get_conn()?;
    let counts = get_counts(&mut conn)?;

    let mut user_statistics = Vec::new();
    for count in counts {
        let nickname = match get_nickname(&mut conn, count.user_id)? {
            Some(n) => n,
            None => continue
        };

        user_statistics.push(UserStatistic {
            user_id: count.user_id,
            count: count.count,
            name: nickname
        });
    }

    let response = Response {
        users: user_statistics
    };

    Ok(HttpResponse::Ok().json(&response))
}

struct Count {
    user_id:    u64,
    count:      i32,
}

fn get_counts(conn: &mut PooledConn) -> anyhow::Result<Vec<Count>> {
    let row: Vec<Row> = conn.exec("SELECT user_id,count FROM uwu_counter", Params::Empty)?;
    let result: Vec<_> = row.into_iter()
        .map(|f| {
            Count {
                user_id: f.get("user_id").unwrap(),
                count: f.get("count").unwrap(),
            }
        })
        .collect();

    Ok(result)
}

fn get_nickname(conn: &mut PooledConn, user_id: u64) -> anyhow::Result<Option<String>> {
    let row: Option<Row> = conn.exec_first("SELECT nickname FROM user_nicknames WHERE user_id = :user_id", params! {
        "user_id" => &user_id
    })?;

    if let Some(row) = row {
        let nickname: String = row.get("nickname").unwrap();
        Ok(Some(nickname))
    } else {
        Ok(None)
    }
}