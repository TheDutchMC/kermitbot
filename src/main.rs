#[macro_use]
extern crate lazy_static;

use std::process::exit;
use std::sync::Arc;
use actix_governor::GovernorConfigBuilder;
use actix_web::http::Method;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::normalize::TrailingSlash;
use log::{info, debug, error};
use serde::Deserialize;
use serenity::http::Http;

mod bot;
mod endpoints;
mod error;

lazy_static! {
    pub static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().expect("Failed to initialize Tokio runtime");
}

mod migrations {
    use refinery::embed_migrations;
    embed_migrations!("./migrations");
}

#[derive(Clone, Deserialize)]
pub struct Env {
    bot_token:          String,
    guild_id:           String,
    mysql_host:         String,
    mysql_database:     String,
    mysql_username:     String,
    mysql_password:     String,
}

pub struct AppData {
    pool: mysql::Pool,
    env:  Env,
    http: Http
}

impl AppData {
    fn new(env: &Env) -> anyhow::Result<Self> {
        let opts = mysql::OptsBuilder::new()
            .ip_or_hostname(Some(&env.mysql_host))
            .db_name(Some(&env.mysql_database))
            .user(Some(&env.mysql_username))
            .pass(Some(&env.mysql_password));

        let pool = mysql::Pool::new(opts)?;
        let http = Http::new_with_token(&env.bot_token);

        Ok(Self {
            pool,
            http,
            env: env.clone()
        })
    }

    fn apply_migrations(&self) -> anyhow::Result<()> {
        let mut conn = self.pool.get_conn()?;
        migrations::migrations::runner().run(&mut conn)?;
        Ok(())
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    info!("Starting bot");

    debug!("Reading environment");
    let env: Env = match envy::from_env() {
        Ok(e) => e,
        Err(e) => {
            error!("Unable to create Env object: {:?}", e);
            exit(1);
        }
    };

    debug!("Creating AppData");
    let appdata = match AppData::new(&env) {
        Ok(a) => a,
        Err(e) => {
            error!("Unable to create AppData object: {:?}", e);
            exit(1);
        }
    };

    debug!("Applying migrations");
    match appdata.apply_migrations() {
        Ok(_) => {},
        Err(e) => {
            error!("Failed to apply migrations: {:?}", e);
            exit(1);
        }
    }

    let appdata = Arc::new(appdata);

    debug!("Starting Serenity");
    let appdata_clone = appdata.clone();
    std::thread::spawn(move || {
        let _guard = RT.enter();
        match RT.block_on(bot::start(env.bot_token.clone(), appdata_clone)) {
            Ok(_) => {},
            Err(e) => {
                error!("Failed to initialize Serenity: {:?}", e);
                exit(1);
            }
        }
    });

    let get_gov_config = GovernorConfigBuilder::default()
        .methods(vec![Method::GET])
        .per_second(10)
        .burst_size(20)
        .finish()
        .unwrap();

    let option_gov_config = GovernorConfigBuilder::default()
        .methods(vec![Method::GET])
        .per_second(20)
        .burst_size(30)
        .finish()
        .unwrap();

    debug!("Starting Actix server");
    let server = HttpServer::new(move || {
        let app = App::new()
            .data(appdata.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(actix_web::middleware::NormalizePath::new(TrailingSlash::Trim))
            .wrap(actix_cors::Cors::permissive())
            .wrap(actix_governor::Governor::new(&get_gov_config))
            .wrap(actix_governor::Governor::new(&option_gov_config))
            .service(web::scope("/api/v1")
                .service(endpoints::uwu_counter::uwu_counter));

        #[cfg(not(debug_assertations))]
        app.service(web::scope("/static")
            .service(actix_files::Files::new("", "./frontend_dist").show_files_listing()))
    }).bind("[::]:8080")?.run();

    info!("Started");
    server.await
}