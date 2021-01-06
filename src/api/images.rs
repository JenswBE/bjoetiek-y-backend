// Based on https://github.com/actix/examples/blob/master/multipart/src/main.rs

use actix_multipart::Multipart;
use actix_web::{put, web, Error, HttpResponse, Scope};
use futures::{StreamExt, TryStreamExt};

use crate::actors::UploadImage;
use crate::State;

pub fn admin_scope(path: &str) -> Scope {
    web::scope(path).service(upload_image)
}

/// Upload a new image
#[put("/{image_id}")]
async fn upload_image(
    state: web::Data<State>,
    image_id: web::Path<uuid::Uuid>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    // Extract field from multipart
    if let Ok(Some(mut field)) = payload.try_next().await {
        // Collect data into vector
        let mut image: Vec<u8> = Vec::new();
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            image.extend_from_slice(data.as_ref());
        }

        // Call actor
        let msg = UploadImage {
            id: image_id.into_inner(),
            data: image,
        };
        let result = state
            .image
            .send(msg)
            .await
            .expect("Failed to call ImageActor");

        // Handle and translate errors
        if let Err(e) = result {
            if let Some(libvips::error::Error::InitializationError(_)) = e.downcast_ref() {
                return Ok(HttpResponse::BadRequest().body("Image format not supported"));
            } else {
                return Ok(HttpResponse::InternalServerError().body(e.to_string()));
            }
        }
    }
    Ok(HttpResponse::Ok().into())
}
