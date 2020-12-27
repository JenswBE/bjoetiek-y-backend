use actix_web::{get, post, web, Error, HttpResponse, Scope};

use crate::db::manufacturers::*;
use crate::models;
use crate::State;

pub fn manufacturers_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_manufacturers)
        .service(get_manufacturer)
        .service(add_manufacturer)
}

/// List all manufacturers
#[get("/manufacturers")]
async fn list_manufacturers(state: web::Data<State>) -> Result<HttpResponse, Error> {
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
    state: web::Data<State>,
    manufacturer_uid: web::Path<uuid::Uuid>,
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
    state: web::Data<State>,
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
