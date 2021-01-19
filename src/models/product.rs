use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::products;

#[derive(Debug, Identifiable, Queryable, Serialize)]
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
    pub stock_count: i32,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "products"]
pub struct ProductData {
    pub name: String,
    pub slug: String,
    pub description_short: String,
    pub description_long: String,
    pub price: i32,
    pub manufacturer_id: Option<Uuid>,
    pub status: String,
    pub stock_count: i32,
}
