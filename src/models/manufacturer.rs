use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::manufacturers;

#[derive(Debug, Identifiable, Queryable, Serialize)]
pub struct Manufacturer {
    pub id: Uuid,
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}

#[derive(Debug, Insertable, AsChangeset, Deserialize)]
#[table_name = "manufacturers"]
pub struct ManufacturerData {
    pub name: String,
    pub website_url: String,
    pub logo_url: String,
}
