mod db;
mod ser;
mod util;

use xitca_http::{
    h1::RequestBody,
    http::{header::SERVER, StatusCode},
    util::{
        middleware::context::{Context, ContextBuilder},
        service::{
            route::get,
            router::{Router, RouterError},
        },
    },
    HttpServiceBuilder,
};
use xitca_service::{fn_service, Service, ServiceExt};

use db::Client;
use ser::{error_response, IntoResponse, Request, Response};
use util::{HandleResult, State, SERVER_HEADER_VALUE};

type Ctx<'a> = Context<'a, Request<RequestBody>, State<Client>>;

fn main() -> std::io::Result<()> {
    let service = Router::new()
        .insert("/api", get(fn_service(db)))
        .enclosed_fn(middleware)
        .enclosed(ContextBuilder::new(|| async { db::create().await.map(State::new) }))
        .enclosed(HttpServiceBuilder::h1().io_uring());
    xitca_server::Builder::new()
        .bind("fuji", "0.0.0.0:8080", service)?
        .build()
        .wait()
}

async fn middleware<S>(service: &S, req: Ctx<'_>) -> Result<Response, core::convert::Infallible>
where
    S: for<'c> Service<Ctx<'c>, Response = Response, Error = RouterError<util::Error>>,
{
    let mut res = service.call(req).await.unwrap_or_else(error_handler);
    res.headers_mut().insert(SERVER, SERVER_HEADER_VALUE);
    Ok(res)
}

#[cold]
#[inline(never)]
fn error_handler(e: RouterError<util::Error>) -> Response {
    match e {
        RouterError::Match(_) => error_response(StatusCode::NOT_FOUND),
        RouterError::NotAllowed(_) => error_response(StatusCode::METHOD_NOT_ALLOWED),
        RouterError::Service(e) => {
            println!("error service: {e}");
            error_response(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn db(ctx: Ctx<'_>) -> HandleResult<Response> {
    println!("masuk");
    let (req, state) = ctx.into_parts();
    let hasil = state.client.db_json().await?;
    println!("{:?}", hasil);
    req.json_response(state, &hasil)
}
