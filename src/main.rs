#![allow(unused)]

use std::net::SocketAddr;

use crate::model::ModelController;

pub use self::error::{Error, Result};

use axum::{Router, middleware};
use axum::extract::{Query, Path};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse, Response};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;

#[tokio::main]
async fn main() -> Result<()> {
    
    let mc = ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    // region: ---Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING ON http://{addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

    Ok(())

}

async fn main_response_mapper (res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}


fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

//region: ---- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/:name",get(handler_hello2))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=Jen`
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("->> {:12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g. `/hello2/Mike`
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello2 <strong>{name}</strong>"))
}