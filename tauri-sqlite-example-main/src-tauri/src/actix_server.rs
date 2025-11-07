use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_web::http::header;
use actix_cors::Cors;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from Actix inside Tauri!")
}

pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
         let cors = Cors::default()
            .allowed_origin("http://localhost:1420") // dein Tauri Dev-Port
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .supports_credentials()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .route("/api/hello", web::get().to(hello))
    })
    .bind("127.0.0.1:7878")?
    .run()
    .await
}
