use actix_web::{get, /*post, web,*/ middleware, App, HttpResponse, HttpServer, Responder};
use actix_web::web::{Path};
use std::{env};
use std::str;
//use uuid::Uuid;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
        env_logger::init();

        HttpServer::new(|| {
                App::new()
                    .wrap(middleware::Logger::default())
                    .service(hello)
                    .service(get_status)
                    .service(get_application)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello !")
} 

#[get("/status")]
async fn get_status() -> impl Responder {
    HttpResponse::Ok().body("this is the status endpoint")
}

//Application endpoints

#[get("/application/{id}/")]
pub async fn get_application(path: Path<(String,)>) -> impl Responder {
    let id = path.into_inner();

    let message = format!("This is the application {}" , id.0);

    HttpResponse::Ok().body(message)
}


