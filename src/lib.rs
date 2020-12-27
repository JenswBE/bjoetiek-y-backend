#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{middleware, middleware::normalize::TrailingSlash, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::db::DbActor;

mod api;
mod db;
mod models;
mod schema;

diesel_migrations::embed_migrations!();

#[derive(Clone)]
struct State {
    db: Addr<DbActor>,
}

impl State {
    pub fn new(db: Addr<db::DbActor>) -> Self {
        Self { db }
    }
}

pub async fn run() -> std::io::Result<()> {
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

    let bind = env::var("BIND").unwrap_or("0.0.0.0:8090".to_string());
    let state = State::new(db_addr);

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .data(state.clone())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .service(api::manufacturers_scope("/manufacturers"))
            .service(fs::Files::new("/", "docs").index_file("index.html"))
    })
    .bind(&bind)?
    .run()
    .await
}
