#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    error, middleware, middleware::normalize::TrailingSlash, web, App, HttpResponse, HttpServer,
};
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

use crate::actors::ImageActor;
use crate::api::{auth, categories, images, manufacturers, products};
use crate::db::DbActor;

mod actors;
mod api;
mod db;
pub mod models;
mod schema;

pub use models::Config;

diesel_migrations::embed_migrations!();

#[derive(Clone)]
struct Context {
    pub creds: auth::BasicCreds,
    pub db: Addr<DbActor>,
    pub image: Addr<ImageActor>,
}

pub async fn run(config: Config) -> std::io::Result<()> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    // Setup database connection pool
    let connspec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(connspec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // Run migrations
    {
        let conn = pool.get().expect("Couldn't get db connection from pool");
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
            .expect("Failed to migrate database");
    }

    // Build state
    let images_path = config.images_path.clone();
    let ctx = Context {
        creds: auth::BasicCreds::new(&config.admin_username, &config.admin_password),
        db: SyncArbiter::start(3, move || DbActor::new(pool.clone())),
        image: SyncArbiter::start(3, move || ImageActor::new(images_path.clone())),
    };

    // Start HTTP server
    log::info!("Starting server at: {}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(|err, _| {
                let msg = format!("{}", err);
                error::InternalError::from_response(err, HttpResponse::BadRequest().body(msg))
                    .into()
            }))
            .data(ctx.clone())
            .service(
                web::scope("/public")
                    .service(categories::public_scope("/categories"))
                    .service(manufacturers::public_scope("/manufacturers"))
                    .service(products::public_scope("/products")),
            )
            .service(
                web::scope("/admin")
                    .wrap(HttpAuthentication::basic(auth::validator))
                    .service(categories::admin_scope("/categories"))
                    .service(images::admin_scope("/images"))
                    .service(manufacturers::admin_scope("/manufacturers"))
                    .service(products::admin_scope("/products")),
            )
            .wrap(Cors::permissive().allow_any_origin())
            .wrap(middleware::DefaultHeaders::new().header("Content-Type", "text/plain"))
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::new(TrailingSlash::Trim))
            .service(fs::Files::new("/images", "images"))
            .service(fs::Files::new("/", "docs").index_file("index.html"))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
