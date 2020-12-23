use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{categories, manufacturers};

// #[derive(Queryable)]
// pub struct Product {
//     pub id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub name: String,
//     pub slug: String,
//     pub description_short: String,
//     pub description_long: String,
//     pub price: i32,
//     pub manufacturer_id: Uuid,
//     pub status: String,
//     pub image_url: String,
// }

#[derive(Insertable)]
#[table_name = "manufacturers"]
pub struct NewManufacturer {
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}

#[derive(Queryable)]
pub struct Manufacturer {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}

// #[derive(Debug, Clone, Serialize, Insertable)]
// #[table_name = "categories"]
// pub struct NewCategory {
//     pub name: String,
//     pub description: String,
//     pub image_url: String,
//     pub sort_order: i16,
// }

// #[derive(Queryable)]
// pub struct Category {
//     pub id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub updated_at: NaiveDateTime,
//     pub name: String,
//     pub description: String,
//     pub image_url: String,
//     pub sort_order: i16,
// }

// #[derive(Queryable)]
// pub struct CategoryProduct {
//     pub product_id: Uuid,
//     pub category_id: Uuid,
// }
