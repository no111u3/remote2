use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

fn main() {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
