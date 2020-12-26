use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::schema::products;

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
