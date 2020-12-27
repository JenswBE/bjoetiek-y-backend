use actix::{Handler, Message};
use diesel::{prelude::*, result::Error::NotFound};
use failure::Error;

use super::DbActor;
use crate::models::{Manufacturer, ManufacturerData};
use crate::schema::manufacturers::dsl;

#[derive(Debug)]
pub struct ListManufacturers {}

impl Message for ListManufacturers {
    type Result = Result<Vec<Manufacturer>, Error>;
}

impl Handler<ListManufacturers> for DbActor {
    type Result = Result<Vec<Manufacturer>, Error>;

    fn handle(&mut self, _msg: ListManufacturers, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let manufacturers = dsl::manufacturers
            .load::<Manufacturer>(&conn)
            .expect("Error loading manufacturers");

        Ok(manufacturers)
    }
}

#[derive(Debug)]
pub struct GetManufacturer {
    pub id: uuid::Uuid,
}

impl Message for GetManufacturer {
    type Result = Result<Option<Manufacturer>, Error>;
}

impl Handler<GetManufacturer> for DbActor {
    type Result = Result<Option<Manufacturer>, Error>;

    fn handle(&mut self, msg: GetManufacturer, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let manufacturer = dsl::manufacturers
            .find(msg.id)
            .first::<Manufacturer>(&conn)
            .optional()?;

        Ok(manufacturer)
    }
}

#[derive(Debug)]
pub struct InsertManufacturer {
    pub data: ManufacturerData,
}

impl Message for InsertManufacturer {
    type Result = Result<Manufacturer, Error>;
}

impl Handler<InsertManufacturer> for DbActor {
    type Result = Result<Manufacturer, Error>;

    fn handle(&mut self, msg: InsertManufacturer, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::insert_into(dsl::manufacturers)
            .values(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct UpdateManufacturer {
    pub id: uuid::Uuid,
    pub data: ManufacturerData,
}

impl Message for UpdateManufacturer {
    type Result = Result<Manufacturer, Error>;
}

impl Handler<UpdateManufacturer> for DbActor {
    type Result = Result<Manufacturer, Error>;

    fn handle(&mut self, msg: UpdateManufacturer, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::update(dsl::manufacturers.find(msg.id))
            .set(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct DeleteManufacturer {
    pub id: uuid::Uuid,
}

impl Message for DeleteManufacturer {
    type Result = Result<(), Error>;
}

impl Handler<DeleteManufacturer> for DbActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteManufacturer, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::delete(dsl::manufacturers.find(msg.id))
            .execute(&conn)
            .and_then(|c| if c > 0 { Ok(()) } else { Err(NotFound) })
            .map_err(Error::from)
    }
}
