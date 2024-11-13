#![allow(dead_code)]

use core::cell::RefCell;

use xitca_http::{bytes::BytesMut, http::header::HeaderValue};

#[allow(clippy::declare_interior_mutable_const)]
pub const SERVER_HEADER_VALUE: HeaderValue = HeaderValue::from_static("X");

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type HandleResult<T> = Result<T, Error>;

pub const DB_URL: &str = "postgres://fuji:fuji@localhost/tes";

pub struct State<DB> {
    pub client: DB,
    pub write_buf: RefCell<BytesMut>,
}

impl<DB> State<DB> {
    pub fn new(client: DB) -> Self {
        Self {
            client,
            write_buf: Default::default(),
        }
    }
}
