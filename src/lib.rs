use std::net::TcpListener;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

#[derive(serde::Deserialize)]
struct FormData{
    name:String,
    email:String
}
// fn index(form:web::Form<FormData>)->String{
//     format!("Welcome {}!",form.username)
// }
async fn health_check() -> HttpResponse {
    // HttpResponse::Ok().finish()
    HttpResponse::Ok().finish()
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("世界1");
    format!("hello {} !", &name)
}
async fn subscribe(_form:web::Form<FormData>)->HttpResponse{
    HttpResponse::Ok().finish()
}

pub  fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
  let server =   HttpServer::new(|| {
        App::new().route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions",web::post().to(subscribe))
            .route("/{name}", web::get().to(greet))
    }).listen(listener)?
        .run();
    Ok(server)
}
