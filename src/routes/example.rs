use actix_web::{HttpResponse, get, Responder};

#[get("/example")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Reached Example Route")
}
