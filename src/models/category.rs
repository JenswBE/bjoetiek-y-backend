use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::categories;

#[derive(Debug, Identifiable, Queryable, Serialize)]
#[table_name = "categories"]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub sort_order: i16,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "categories"]
pub struct CategoryData {
    pub name: String,
    pub description: String,
    pub sort_order: i16,
}

#[derive(Queryable)]
pub struct CategoryProduct {
    pub product_id: Uuid,
    pub category_id: Uuid,
}
