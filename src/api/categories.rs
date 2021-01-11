use actix_web::{delete, get, post, put, web, Error, HttpResponse, Scope};

use crate::actors::DeleteImage;
use crate::db::categories::*;
use crate::models;
use crate::Context;

pub fn public_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_categories)
        .service(get_category)
}

pub fn admin_scope(path: &str) -> Scope {
    web::scope(path)
        .service(list_categories)
        .service(get_category)
        .service(add_category)
        .service(update_category)
        .service(delete_category)
}

/// List all categories
#[get("")]
async fn list_categories(ctx: web::Data<Context>) -> Result<HttpResponse, Error> {
    let categories = ctx
        .db
        .send(ListCategories {})
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch category");
    Ok(HttpResponse::Ok().json(categories))
}

/// Find category by ID
#[get("/{category_id}")]
async fn get_category(
    ctx: web::Data<Context>,
    category_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let category_id = category_id.into_inner();
    let msg = GetCategory {
        id: category_id.clone(),
    };
    let category = ctx
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch category");
    if let Some(category) = category {
        Ok(HttpResponse::Ok().json(category))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No category found with id: {}", category_id));
        Ok(res)
    }
}

/// Insert new category from form
#[post("")]
async fn add_category(
    ctx: web::Data<Context>,
    form: web::Json<models::CategoryData>,
) -> Result<HttpResponse, Error> {
    let msg = InsertCategory {
        data: form.into_inner(),
    };
    let category = ctx
        .db
        .send(msg)
        .await
        .expect("Failed to contact DbActor")
        .expect("Failed to fetch category");

    Ok(HttpResponse::Ok().json(category))
}

/// Update category from form
#[put("/{category_id}")]
async fn update_category(
    ctx: web::Data<Context>,
    category_id: web::Path<uuid::Uuid>,
    form: web::Json<models::CategoryData>,
) -> Result<HttpResponse, Error> {
    let category_id = category_id.into_inner();
    let msg = UpdateCategory {
        id: category_id.clone(),
        data: form.into_inner(),
    };
    let category = ctx.db.send(msg).await.expect("Failed to contact DbActor");

    if let Ok(category) = category {
        Ok(HttpResponse::Ok().json(category))
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No category found with id: {}", category_id));
        Ok(res)
    }
}

/// Delete category with ID
#[delete("/{category_id}")]
async fn delete_category(
    ctx: web::Data<Context>,
    category_id: web::Path<uuid::Uuid>,
) -> Result<HttpResponse, Error> {
    let category_id = category_id.into_inner();
    let msg = DeleteCategory {
        id: category_id.clone(),
    };
    let result = ctx.db.send(msg).await.expect("Failed to contact DbActor");

    if result.is_ok() {
        // Request deletion of image and thumbnails
        let msg = DeleteImage { id: category_id };
        ctx.image.do_send(msg);

        // Send success response
        Ok(HttpResponse::Ok().finish())
    } else {
        let res =
            HttpResponse::NotFound().body(format!("No category found with id: {}", category_id));
        Ok(res)
    }
}
