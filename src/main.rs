use actix_web::{web, App, HttpServer};

struct AppState {
    app_name: String,
}

fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;

    format!("Hello from {}!", app_name)
}

fn config(cfg: &mut web::ServiceConfig) {
    cfg.data(AppState {
        app_name: String::from("Draft Application of Remote Control"),
    })
    .route("/", web::get().to(index));
}

fn main() {
    HttpServer::new(|| App::new().configure(config))
        .bind("127.0.0.1:8088")
        .unwrap()
        .run()
        .unwrap();
}
