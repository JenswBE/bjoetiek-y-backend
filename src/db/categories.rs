use actix::{Handler, Message};
use diesel::{prelude::*, result::Error::NotFound};
use failure::Error;

use super::DbActor;
use crate::models::{Category, CategoryData};
use crate::schema::categories::dsl;

#[derive(Debug)]
pub struct ListCategories {}

impl Message for ListCategories {
    type Result = Result<Vec<Category>, Error>;
}

impl Handler<ListCategories> for DbActor {
    type Result = Result<Vec<Category>, Error>;

    fn handle(&mut self, _msg: ListCategories, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let categories = dsl::categories
            .load::<Category>(&conn)
            .expect("Error loading categories");

        Ok(categories)
    }
}

#[derive(Debug)]
pub struct GetCategory {
    pub id: uuid::Uuid,
}

impl Message for GetCategory {
    type Result = Result<Option<Category>, Error>;
}

impl Handler<GetCategory> for DbActor {
    type Result = Result<Option<Category>, Error>;

    fn handle(&mut self, msg: GetCategory, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let category = dsl::categories
            .find(msg.id)
            .first::<Category>(&conn)
            .optional()?;

        Ok(category)
    }
}

#[derive(Debug)]
pub struct InsertCategory {
    pub data: CategoryData,
}

impl Message for InsertCategory {
    type Result = Result<Category, Error>;
}

impl Handler<InsertCategory> for DbActor {
    type Result = Result<Category, Error>;

    fn handle(&mut self, msg: InsertCategory, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::insert_into(dsl::categories)
            .values(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct UpdateCategory {
    pub id: uuid::Uuid,
    pub data: CategoryData,
}

impl Message for UpdateCategory {
    type Result = Result<Category, Error>;
}

impl Handler<UpdateCategory> for DbActor {
    type Result = Result<Category, Error>;

    fn handle(&mut self, msg: UpdateCategory, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::update(dsl::categories.find(msg.id))
            .set(msg.data)
            .get_result(&conn)
            .map_err(Error::from)
    }
}

#[derive(Debug)]
pub struct DeleteCategory {
    pub id: uuid::Uuid,
}

impl Message for DeleteCategory {
    type Result = Result<(), Error>;
}

impl Handler<DeleteCategory> for DbActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeleteCategory, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        diesel::delete(dsl::categories.find(msg.id))
            .execute(&conn)
            .and_then(|c| if c > 0 { Ok(()) } else { Err(NotFound) })
            .map_err(Error::from)
    }
}
