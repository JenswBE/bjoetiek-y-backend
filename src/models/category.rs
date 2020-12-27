use serde::Serialize;
use uuid::Uuid;

use crate::schema::categories;

#[derive(Debug, Clone, Serialize, Insertable, AsChangeset)]
#[table_name = "categories"]
pub struct CategoryData {
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
