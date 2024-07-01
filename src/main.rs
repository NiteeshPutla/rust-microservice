use actix_web::{cookie::time::format_description::well_known::iso8601::Config, get, web, App, HttpResponse, HttpServer, Responder};



#[get("/")]
async fn index() -> impl Responder{
    HttpResponse::Ok().body("Rust service prototype")
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder{
    HttpResponse::Ok().body("I'm alive")
}


pub fn init(config: &mut web:: ServiceConfig){
    config.service(
        web::scope("")
                .service(index)
                .service(healthcheck)
    );
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {

        App::new()
            .configure(init)  
    })

    .bind("0.0.0.0: 8000")?
    .run()
    .await
}
