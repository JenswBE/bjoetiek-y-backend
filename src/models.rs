use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{categories, manufacturers, products};

#[derive(Queryable, Identifiable)]
pub struct Product {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub price: i32,
    pub manufacturer_id: Option<Uuid>,
    pub status: String,
    pub image_url: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "manufacturers"]
pub struct NewManufacturer {
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}

#[derive(Identifiable, Queryable, Serialize)]
pub struct Manufacturer {
    pub id: Uuid,
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}

#[derive(Debug, Clone, Serialize, Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub sort_order: i16,
}

#[derive(Identifiable, Queryable)]
#[table_name = "categories"]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub sort_order: i16,
}

#[derive(Queryable)]
pub struct CategoryProduct {
    pub product_id: Uuid,
    pub category_id: Uuid,
}
