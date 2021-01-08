use actix_web::{App, HttpServer};
use tera::Tera;

mod views;


fn port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(8000)
}

#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        let tera =
            Tera::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")
            ).unwrap();

        App::new()
            .data(tera)
            .service(views::index)
            .service(views::rolled)
            .service(views::joke)
    })
    .bind(("0.0.0.0", port())).unwrap()
    .run()
    .await.unwrap()
}

