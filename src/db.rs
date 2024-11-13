#[path = "./db_util.rs"]
mod db_util;

use core::cell::RefCell;

use xitca_postgres::{iter::AsyncLendingIterator, pool::Pool, Execute};

use super::{
    ser::Tes,
    util::{HandleResult, DB_URL},
};

use db_util::{not_found, TES_STMT};

pub struct Client {
    pool: Pool,
    shared: RefCell<String>
}

pub async fn create() -> HandleResult<Client> {
    Ok(Client {
        pool: Pool::builder(DB_URL).capacity(1).build()?,
        shared: String::from("fuji").into()
    })
}

impl Client {
    pub async fn db_json(&self) -> HandleResult<Tes> {
        let mut conn = self.pool.get().await?;
        let stmt = TES_STMT.execute(&mut conn).await?;
        let nama = self.shared.borrow();
        let mut res = stmt.bind([&*nama]).query(&conn.consume()).await?;
        let row = res.try_next().await?.ok_or_else(not_found)?;
        Ok(Tes::new(row.get(0)))
    }
}
