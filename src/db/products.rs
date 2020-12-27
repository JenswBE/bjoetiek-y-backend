use actix::{Handler, Message};
use diesel::{prelude::*, result::Error::NotFound};
use failure::Error;

use super::DbActor;
use crate::models::{Product, ProductData};
use crate::schema::products::dsl;

#[derive(Debug)]
pub struct ListProducts {}

impl Message for ListProducts {
    type Result = Result<Vec<Product>, Error>;
}

impl Handler<ListProducts> for DbActor {
    type Result = Result<Vec<Product>, Error>;

    fn handle(&mut self, _msg: ListProducts, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let products = dsl::products
            .load::<Product>(&conn)
            .expect("Error loading products");

        Ok(products)
    }
}

#[derive(Debug)]
pub struct GetProduct {
    pub id: uuid::Uuid,
}

impl Message for GetProduct {
    type Result = Result<Option<Product>, Error>;
}

impl Handler<GetProduct> for DbActor {
    type Result = Result<Option<Product>, Error>;

    fn handle(&mut self, msg: GetProduct, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let product = dsl::products
            .find(msg.id)
            .first::<Product>(&conn)
            .optional()?;

        Ok(product)
    }
}

#[derive(Debug)]
pub struct InsertProduct {
    pub data: ProductData,
}

impl Message for InsertProduct {
    type Result = Result<Product, Error>;
}

impl Handler<InsertProduct> for DbActor {
    type Result = Result<Product, Error>;

    fn handle(&mut self, msg: InsertProduct, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::insert_into(dsl::products)
            .values(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct UpdateProduct {
    pub id: uuid::Uuid,
    pub data: ProductData,
}

impl Message for UpdateProduct {
    type Result = Result<Product, Error>;
}

impl Handler<UpdateProduct> for DbActor {
    type Result = Result<Product, Error>;

    fn handle(&mut self, msg: UpdateProduct, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::update(dsl::products.find(msg.id))
            .set(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct DeleteProduct {
    pub id: uuid::Uuid,
}

impl Message for DeleteProduct {
    type Result = Result<(), Error>;
}

impl Handler<DeleteProduct> for DbActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteProduct, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::delete(dsl::products.find(msg.id))
            .execute(&conn)
            .and_then(|c| if c > 0 { Ok(()) } else { Err(NotFound) })
            .map_err(Error::from)
    }
}
