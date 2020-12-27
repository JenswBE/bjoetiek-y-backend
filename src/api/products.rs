use actix_web::{delete, get, post, put, web, Error, HttpResponse, Scope};

use crate::db::products::*;
use crate::models;
use crate::State;

pub fn public_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_products)
        .service(get_product)
}

pub fn admin_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_products)
        .service(get_product)
        .service(add_product)
        .service(update_product)
        .service(delete_product)
}

/// List all products
#[get("")]
async fn list_products(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let products = state
        .db
        .send(ListProducts {})
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch product");
    Ok(HttpResponse::Ok().json(products))
}

/// Find product by ID
#[get("/{product_id}")]
async fn get_product(
    state: web::Data<State>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = GetProduct {
        id: product_id.clone(),
    };
    let product = state
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch product");
    if let Some(product) = product {
        Ok(HttpResponse::Ok().json(product))
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No product found with id: {}",
            product_id
        ));
        Ok(res)
    }
}

/// Insert new product from form
#[post("")]
async fn add_product(
    state: web::Data<State>,
    form: web::Json<models::ProductData>,
) -> Result<HttpResponse, Error> {
    let msg = InsertProduct {
        data: form.into_inner(),
    };
    let product = state
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch product");

    Ok(HttpResponse::Ok().json(product))
}

/// Update product from form
#[put("/{product_id}")]
async fn update_product(
    state: web::Data<State>,
    product_id: web::Path<uuid::Uuid>,
    form: web::Json<models::ProductData>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = UpdateProduct {
        id: product_id.clone(),
        data: form.into_inner(),
    };
    let product = state.db.send(msg).await.expect("Failed to contact DbActor");

    if let Ok(product) = product {
        Ok(HttpResponse::Ok().json(product))
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No product found with id: {}",
            product_id
        ));
        Ok(res)
    }
}

/// Delete product with ID
#[delete("/{product_id}")]
async fn delete_product(
    state: web::Data<State>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = DeleteProduct {
        id: product_id.clone(),
    };
    let result = state.db.send(msg).await.expect("Failed to contact DbActor");

    if result.is_ok() {
        Ok(HttpResponse::Ok().finish())
    } else {
        let res = HttpResponse::NotFound().body(format!(
            "No product found with id: {}",
            product_id
        ));
        Ok(res)
    }
}
