use actix_web::{delete, get, post, put, web, Error, HttpResponse, Scope};

use crate::actors::DeleteImage;
use crate::db::products::*;
use crate::models;
use crate::Context;

pub fn public_scope(path: &str) -> Scope {
    web::scope(path).service(list_products).service(get_product)
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
async fn list_products(ctx: web::Data<Context>) -> Result<HttpResponse, Error> {
    let products = ctx
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
    ctx: web::Data<Context>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = GetProduct {
        id: product_id.clone(),
    };
    let product = ctx
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch product");
    if let Some(product) = product {
        Ok(HttpResponse::Ok().json(product))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No product found with id: {}", product_id));
        Ok(res)
    }
}

/// Insert new product from form
#[post("")]
async fn add_product(
    ctx: web::Data<Context>,
    form: web::Json<models::ProductData>,
) -> Result<HttpResponse, Error> {
    let msg = InsertProduct {
        data: form.into_inner(),
    };
    let product = ctx
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
    ctx: web::Data<Context>,
    product_id: web::Path<uuid::Uuid>,
    form: web::Json<models::ProductData>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = UpdateProduct {
        id: product_id.clone(),
        data: form.into_inner(),
    };
    let product = ctx.db.send(msg).await.expect("Failed to contact DbActor");

    if let Ok(product) = product {
        Ok(HttpResponse::Ok().json(product))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No product found with id: {}", product_id));
        Ok(res)
    }
}

/// Delete product with ID
#[delete("/{product_id}")]
async fn delete_product(
    ctx: web::Data<Context>,
    product_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let product_id = product_id.into_inner();
    let msg = DeleteProduct {
        id: product_id.clone(),
    };
    let result = ctx.db.send(msg).await.expect("Failed to contact DbActor");

    if result.is_ok() {
        // Request deletion of image and thumbnails
        let msg = DeleteImage { id: product_id };
        ctx.image.do_send(msg);

        // Send success response
        Ok(HttpResponse::Ok().finish())
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No product found with id: {}", product_id));
        Ok(res)
    }
}
