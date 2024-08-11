use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

async fn health_check() -> HttpResponse {
    // HttpResponse::Ok().finish()
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("世界1");
    format!("hello {} !", &name)
}

pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server =   HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    }).listen(listener)?
        .run();
    Ok(server)
}
