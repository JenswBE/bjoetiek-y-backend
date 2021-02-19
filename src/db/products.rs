use actix::{Handler, Message};
use diesel::{prelude::*, result::Error::NotFound};
use failure::Error;
use uuid::Uuid;

use super::helpers;
use super::DbActor;
use crate::models::{CategoryProduct, Product, ProductDataWithMeta, ProductWithMeta};
use crate::schema::category_products::dsl as cp_dsl;
use crate::schema::products::dsl;

#[derive(Debug)]
pub struct ListProducts {}

impl Message for ListProducts {
    type Result = Result<Vec<ProductWithMeta>, Error>;
}

impl Handler<ListProducts> for DbActor {
    type Result = Result<Vec<ProductWithMeta>, Error>;

    fn handle(&mut self, _msg: ListProducts, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        let products = dsl::products
            .load::<Product>(&conn)
            .expect("Error loading products");
        let category_ids = CategoryProduct::belonging_to(&products)
            .load::<CategoryProduct>(&conn)
            .expect("Error loading category products")
            .grouped_by(&products);
        let products_with_meta = products
            .into_iter()
            .zip(category_ids)
            .map(|(product, category_ids)| {
                let category_ids = category_ids.into_iter().map(|c| c.category_id).collect();
                ProductWithMeta {
                    product,
                    category_ids,
                }
            })
            .collect();

        Ok(products_with_meta)
    }
}

#[derive(Debug)]
pub struct GetProduct {
    pub id: uuid::Uuid,
}

impl Message for GetProduct {
    type Result = Result<Option<ProductWithMeta>, Error>;
}

impl Handler<GetProduct> for DbActor {
    type Result = Result<Option<ProductWithMeta>, Error>;

    fn handle(&mut self, msg: GetProduct, _: &mut Self::Context) -> Self::Result {
        // Fetch product
        let conn = self.pool.get()?;
        let product = dsl::products
            .find(msg.id)
            .first::<Product>(&conn)
            .optional()?;
        if product.is_none() {
            return Ok(None);
        }

        // Fetch related data
        let product = product.unwrap();
        let category_ids = CategoryProduct::belonging_to(&product)
            .select(cp_dsl::category_id)
            .load::<Uuid>(&conn)
            .expect("Error loading category products");

        // Build result
        let product_with_meta = ProductWithMeta {
            product,
            category_ids,
        };
        Ok(Some(product_with_meta))
    }
}

#[derive(Debug)]
pub struct InsertProduct {
    pub data: ProductDataWithMeta,
}

impl Message for InsertProduct {
    type Result = Result<ProductWithMeta, Error>;
}

impl Handler<InsertProduct> for DbActor {
    type Result = Result<ProductWithMeta, Error>;

    fn handle(&mut self, msg: InsertProduct, ctx: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        // Insert product
        let product = diesel::insert_into(dsl::products)
            .values(&msg.data.product)
            .get_result::<Product>(&conn)?;

        // Update product to set slug and CategoryProducts
        let msg_update = UpdateProduct {
            id: product.id,
            data: msg.data,
        };
        self.handle(msg_update, ctx)
    }
}

#[derive(Debug)]
pub struct UpdateProduct {
    pub id: uuid::Uuid,
    pub data: ProductDataWithMeta,
}

impl Message for UpdateProduct {
    type Result = Result<ProductWithMeta, Error>;
}

impl Handler<UpdateProduct> for DbActor {
    type Result = Result<ProductWithMeta, Error>;

    fn handle(&mut self, mut msg: UpdateProduct, _: &mut Self::Context) -> Self::Result {
        let conn = self.pool.get()?;
        conn.transaction(|| {
            // Update product
            msg.data.product.slug = helpers::generate_slug(&msg.data.product.name, &msg.id);
            let product = diesel::update(dsl::products.find(msg.id))
                .set(&msg.data.product)
                .get_result::<Product>(&conn)?;

            // Remove old CategoryProducts
            diesel::delete(cp_dsl::category_products.filter(cp_dsl::product_id.eq_all(product.id)))
                .execute(&conn)?;

            // Recreate CategoryProducts
            for category_id in msg.data.category_ids.iter() {
                let category_product = CategoryProduct {
                    product_id: product.id,
                    category_id: category_id.clone(),
                };
                diesel::insert_into(cp_dsl::category_products)
                    .values(category_product)
                    .execute(&conn)?;
            }

            // Update successful
            Ok(ProductWithMeta {
                product,
                category_ids: msg.data.category_ids,
            })
        })
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
