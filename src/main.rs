use actix_web::{App, HttpServer};
use tera::Tera;

mod views;


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
    .bind(("0.0.0.0", 8000)).unwrap()
    .run()
    .await.unwrap()
}

