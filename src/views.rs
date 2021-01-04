use actix_web::{get, web, HttpResponse, Responder};
use rand::seq::SliceRandom;


const JOKES: &'static [&str]
    = &[
        "What rock group has four men that don't sing? Mount Rushmore.",
        "When I was a kid, my mother told me I could be anyone I wanted to be. Turns out, identity theft is a crime.",
        "A guy goes to his doctor because he can see into the future. The doctor asks him, \"How long have you suffered from that condition?\" The guy tells him, \"Since next Monday.\"",
        "What do sprinters eat before a race? Nothing, they fast!",
        "What concert costs just 45 cents? 50 Cent featuring Nickelback!",
        "Why couldn't the bicycle stand up by itself? It was two tired!",
        "Did you hear about the restaurant on the moon? Great food, no atmosphere!",
        "A cheese factory exploded in France. Da brie is everywhere!",
        "What do you call someone with no body and no nose? Nobody knows.",
        "How do you make a tissue dance? You put a little boogie in it.",
        "Why did the math book look so sad? Because of all of its problems!"
    ];


#[get("/")]
async fn index(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/rolled")]
async fn rolled(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = tmpl.render("rolled.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/joke")]
async fn joke(tmpl: web::Data<tera::Tera>) -> impl Responder {
    let mut ctx = tera::Context::new();
    let mut rng = rand::thread_rng();
    ctx.insert("joke", JOKES.choose(&mut rng).unwrap());
    let rendered = tmpl.render("joke.html", &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}
