use actix_web::{web, App, HttpResponse, HttpServer};

mod game_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    game_config::read_config_field("./static/my_field.toml".to_string());

    HttpServer::new(|| {
        App::new()
            .route(
                "/",
                web::get().to(|| async { HttpResponse::Ok().body("/") }),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
