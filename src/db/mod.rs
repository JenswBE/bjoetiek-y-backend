pub mod categories;
pub mod manufacturers;
pub mod products;

use actix::{Actor, SyncContext};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbActor {
    pool: DbPool,
}

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}

impl DbActor {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}
