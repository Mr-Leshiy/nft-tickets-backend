use actix_web::{web, App, HttpRequest, HttpServer};

async fn hello_world(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello_world)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(|| App::new().configure(config_app))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
