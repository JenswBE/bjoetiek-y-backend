#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_files as fs;
use actix_web::{get, middleware, post, web, App, Error, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use uuid::Uuid;

mod actions;
mod models;
mod schema;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

diesel_migrations::embed_migrations!();

/// Finds manufacturer by UID.
#[get("/manufacturer/{manufacturer_id}")]
async fn get_manufacturer(
    pool: web::Data<DbPool>,
    manufacturer_uid: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let manufacturer_uid = manufacturer_uid.into_inner();
    let conn = pool.get().expect("Couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let manufacturer =
        web::block(move || actions::find_manufacturer_by_id(manufacturer_uid, &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

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

/// Inserts new manufacturer with name defined in form.
#[post("/manufacturers")]
async fn add_manufacturer(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewManufacturer>,
) -> Result<HttpResponse, Error> {
    let conn = pool.get().expect("couldn't get db connection from pool");

    // use web::block to offload blocking Diesel code without blocking server thread
    let manufacturer =
        web::block(move || actions::insert_new_manufacturer(form.into_inner(), &conn))
            .await
            .map_err(|e| {
                eprintln!("{}", e);
                HttpResponse::InternalServerError().finish()
            })?;

    Ok(HttpResponse::Ok().json(manufacturer))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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

    let bind = env::var("BIND").unwrap_or("0.0.0.0:8090".to_string());

    println!("Starting server at: {}", &bind);

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(get_manufacturer)
            .service(add_manufacturer)
            .service(fs::Files::new("/", "docs").index_file("index.html"))
    })
    .bind(&bind)?
    .run()
    .await
}
