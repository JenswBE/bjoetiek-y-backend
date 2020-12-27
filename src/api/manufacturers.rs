use actix_web::{delete, get, post, put, web, Error, HttpResponse, Scope};

use crate::db::manufacturers::*;
use crate::models;
use crate::State;

pub fn manufacturers_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_manufacturers)
        .service(get_manufacturer)
        .service(add_manufacturer)
        .service(update_manufacturer)
        .service(delete_manufacturer)
}

/// List all manufacturers
#[get("")]
async fn list_manufacturers(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let manufacturers = state
        .db
        .send(ListManufacturers {})
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch manufacturer");
    Ok(HttpResponse::Ok().json(manufacturers))
}

/// Find manufacturer by ID
#[get("/{manufacturer_id}")]
async fn get_manufacturer(
    state: web::Data<State>,
    manufacturer_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let manufacturer_id = manufacturer_id.into_inner();
    let msg = GetManufacturer {
        id: manufacturer_id.clone(),
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
            "No manufacturer found with id: {}",
            manufacturer_id
        ));
        Ok(res)
    }
}

/// Insert new manufacturer from form
#[post("")]
async fn add_manufacturer(
    state: web::Data<State>,
    form: web::Json<models::ManufacturerData>,
) -> Result<HttpResponse, Error> {
    let msg = InsertManufacturer {
        data: form.into_inner(),
    };
    let manufacturer = state
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch manufacturer");

    Ok(HttpResponse::Ok().json(manufacturer))
}

/// Update manufacturer from form
#[put("/{manufacturer_id}")]
async fn update_manufacturer(
    state: web::Data<State>,
    manufacturer_id: web::Path<uuid::Uuid>,
    form: web::Json<models::ManufacturerData>,
) -> Result<HttpResponse, Error> {
    let manufacturer_id = manufacturer_id.into_inner();
    let msg = UpdateManufacturer {
        id: manufacturer_id.clone(),
        data: form.into_inner(),
    };
    let manufacturer = state.db.send(msg).await.expect("Failed to contact DbActor");

    if let Ok(manufacturer) = manufacturer {
        Ok(HttpResponse::Ok().json(manufacturer))
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No manufacturer found with id: {}",
            manufacturer_id
        ));
        Ok(res)
    }
}

/// Delete manufacturer with ID
#[delete("/{manufacturer_id}")]
async fn delete_manufacturer(
    state: web::Data<State>,
    manufacturer_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let manufacturer_id = manufacturer_id.into_inner();
    let msg = DeleteManufacturer {
        id: manufacturer_id.clone(),
    };
    let result = state.db.send(msg).await.expect("Failed to contact DbActor");

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No manufacturer found with id: {}",
            manufacturer_id
        ));
        Ok(res)
    }
}
