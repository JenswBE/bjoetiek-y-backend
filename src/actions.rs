use diesel::prelude::*;
use models::NewManufacturer;
use uuid::Uuid;

use crate::models;

/// Run query using Diesel to find manufacturer by id and return it.
pub fn find_manufacturer_by_id(
    id: Uuid,
    conn: &PgConnection,
) -> Result<Option<models::Manufacturer>, diesel::result::Error> {
    use crate::schema::manufacturers::dsl;

    let manufacturer = dsl::manufacturers
        .filter(dsl::id.eq(id))
        .first::<models::Manufacturer>(conn)
        .optional()?;

    Ok(manufacturer)
}

/// Run query using Diesel to insert a new database row and return the result.
pub fn insert_new_manufacturer(
    new_manufacturer: NewManufacturer,
    conn: &PgConnection,
) -> Result<models::Manufacturer, diesel::result::Error> {
    // It is common when using Diesel with Actix web to import schema-related
    // modules inside a function's scope (rather than the normal module's scope)
    // to prevent import collisions and namespace pollution.
    use crate::schema::manufacturers::dsl;

    diesel::insert_into(dsl::manufacturers)
        .values(&new_manufacturer)
        .get_result(conn)
}
