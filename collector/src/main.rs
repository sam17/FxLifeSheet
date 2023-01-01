use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct Params {
    name: String,
    color: String,
}

#[get("/path/to/page")]
async fn handler(req: HttpRequest) -> HttpResponse {
    let params = web::Query::<Params>::from_query(req.query_string()).unwrap();
    HttpResponse::Ok().body(format!("{:?}", params))
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(handler)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}