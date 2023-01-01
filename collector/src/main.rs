use actix_web::{get, web, App, HttpServer, Responder, HttpRequest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct IntegrationQueryParams {
    integration_name: String,
    start_date: String,
    end_date: String,
}

#[get("/api/v1/getData")]
async fn get_data(req: HttpRequest) -> impl Responder {
    // let params = web::Query::<IntegrationQueryParams>::from_query(req.query_string()).unwrap();
    let params = IntegrationQueryParams {
        integration_name: req.query_string().to_string(),
        start_date: "2020-01-01".to_string(),
        end_date: "2020-01-01".to_string(),
    };
    return web::Json(params);
}

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello bobo")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(greet).service(get_data)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}