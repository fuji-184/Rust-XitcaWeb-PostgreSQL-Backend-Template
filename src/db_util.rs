use crate::util::Error;

#[cold]
#[inline(never)]
pub fn not_found() -> Error {
    "request World does not exist".into()
}

#[cfg(feature = "pg")]
pub use pg::*;

#[cfg(feature = "pg")]
pub mod pg {
    use xitca_postgres::{
        statement::{Statement, StatementNamed},
        types::Type,
    };

    pub const TES_STMT: StatementNamed = Statement::named("SELECT nama FROM tes WHERE nama = $1", &[Type::TEXT]);
}
