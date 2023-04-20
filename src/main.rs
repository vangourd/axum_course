#![allow(unused)]

use std::net::SocketAddr;

use axum::Router;
use axum::extract::{Query, Path};
use axum::routing::{get, get_service};
use axum::response::{Html, IntoResponse};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    
    let routes_all = Router::new().merge(routes_hello());

    // region: ---Start Server
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    println!("->> LISTENING ON {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();

}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

//region: ---- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello",get(handler_hello))
        .route("/hello2/:name",get(handler_hello2));
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