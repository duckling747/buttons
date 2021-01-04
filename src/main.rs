use actix_web::{App, HttpServer};
use tera::Tera;

mod views;


#[actix_web::main]
async fn main() {
    HttpServer::new(|| {
        let tera =
            Tera::new(
                concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")
            ).unwrap();

        App::new()
            .data(tera)
            .service(views::index)
            .service(views::rolled)
            .service(views::joke)
    })
    .bind("127.0.0.1:8080").unwrap()
    .run()
    .await.unwrap()
}

