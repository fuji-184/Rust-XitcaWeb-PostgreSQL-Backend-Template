#![allow(dead_code)]

use serde::{ser::SerializeStruct, Serialize, Serializer};
use xitca_http::{
    body::Once,
    bytes::{BufMutWriter, Bytes},
    http::{
        self,
        const_header_value::JSON,
        header::CONTENT_TYPE,
        IntoResponse as _, RequestExt, StatusCode,
    },
};

use crate::util::{Error, State};

#[derive(Debug)]
pub struct Tes {
    pub nama: String
}

impl Tes {
    #[inline]
    pub const fn new(nama: String) -> Self {
        Self { nama }
    }
}


impl Serialize for Tes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut res = serializer.serialize_struct("Tes", 1)?;
        // res.serialize_field("id", &self.id)?;
        res.serialize_field("nama", &self.nama)?;
        res.end()
    }
}

pub type Request<B> = http::Request<RequestExt<B>>;
pub type Response = http::Response<Once<Bytes>>;

pub trait IntoResponse: Sized {
    fn json_response<C>(self, state: &State<C>, val: &impl Serialize) -> Result<Response, Error>;

}

impl<Ext> IntoResponse for Request<Ext> {
    fn json_response<C>(self, state: &State<C>, val: &impl Serialize) -> Result<Response, Error> {
        let buf = &mut *state.write_buf.borrow_mut();
        serde_json::to_writer(BufMutWriter(buf), val)?;
        let mut res = self.into_response(buf.split().freeze());
        res.headers_mut().insert(CONTENT_TYPE, JSON);
        Ok(res)
    }


}

pub fn error_response(status: StatusCode) -> Response {
    http::Response::builder()
        .status(status)
        .body(Once::new(Bytes::new()))
        .unwrap()
}
