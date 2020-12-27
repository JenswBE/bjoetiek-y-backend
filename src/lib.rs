#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, middleware::normalize::TrailingSlash, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::api::{auth, manufacturers};
use crate::db::DbActor;

mod api;
mod db;
pub mod models;
mod schema;

diesel_migrations::embed_migrations!();

#[derive(Clone)]
struct State {
    creds: auth::BasicCreds,
    db: Addr<DbActor>,
}

impl State {
    pub fn new(db: Addr<db::DbActor>, admin_username: String, admin_password: String) -> Self {
        let creds = auth::BasicCreds::new(&admin_username, &admin_password);
        Self { creds, db }
    }
}

pub async fn run(config: models::Config) -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    // set up database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    {
        let conn = pool.get().expect("Couldn't get db connection from pool");
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
            .expect("Failed to migrate database");
    }

    let db_addr = SyncArbiter::start(3, move || DbActor::new(pool.clone()));

    let state = State::new(db_addr, config.admin_username, config.admin_password);

    log::info!("Starting server at: {}:{}", config.host, config.port);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(Cors::permissive())
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .service(web::scope("/public").service(manufacturers::public_scope("/manufacturers")))
            .service(
                web::scope("/admin")
                    .wrap(HttpAuthentication::basic(auth::validator))
                    .wrap(Cors::permissive())
                    .service(manufacturers::admin_scope("/manufacturers")),
            )
            .service(fs::Files::new("/", "docs").index_file("index.html"))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
