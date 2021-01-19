use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::Product;
use crate::schema::{categories, category_products};

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

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(Category)]
#[belongs_to(Product)]
#[primary_key(product_id, category_id)]
pub struct CategoryProduct {
    pub product_id: Uuid,
    pub category_id: Uuid,
}
