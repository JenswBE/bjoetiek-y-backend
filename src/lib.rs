#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix::{Addr, SyncArbiter};
use actix_cors::Cors;
use actix_files as fs;
use actix_web::{
    get, middleware, middleware::normalize::TrailingSlash, post, web, web::Data, App, Error,
    HttpResponse, HttpServer,
};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

use crate::db::{DbActor, GetManufacturer, InsertManufacturer, ListManufacturers};

mod db;
mod models;
mod schema;

diesel_migrations::embed_migrations!();

/// List all manufacturers
#[get("/manufacturers")]
async fn list_manufacturers(state: Data<State>) -> Result<HttpResponse, Error> {
    let manufacturers = state
        .db
        .send(ListManufacturers {})
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch manufacturer");
    Ok(HttpResponse::Ok().json(manufacturers))
}

/// Find manufacturer by UID
#[get("/manufacturers/{manufacturer_id}")]
async fn get_manufacturer(
    state: Data<State>,
    manufacturer_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let manufacturer_uid = manufacturer_uid.into_inner();
    let msg = GetManufacturer {
        id: manufacturer_uid.clone(),
    };
    let manufacturer = state
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch manufacturer");
    if let Some(manufacturer) = manufacturer {
        Ok(HttpResponse::Ok().json(manufacturer))
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No manufacturer found with uid: {}",
            manufacturer_uid
        ));
        Ok(res)
    }
}

/// Insert new manufacturer from form
#[post("/manufacturers")]
async fn add_manufacturer(
    state: Data<State>,
    form: web::Json<models::NewManufacturer>,
) -> Result<HttpResponse, Error> {
    let msg = InsertManufacturer {
        manufacturer: form.into_inner(),
    };
    let manufacturer = state
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch manufacturer");

    Ok(HttpResponse::Ok().json(manufacturer))
}

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
            .service(list_manufacturers)
            .service(get_manufacturer)
            .service(add_manufacturer)
            .service(fs::Files::new("/", "docs").index_file("index.html"))
    })
    .bind(&bind)?
    .run()
    .await
}
