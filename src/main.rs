use actix_web::{get, /*post, web,*/ middleware, App, HttpResponse, HttpServer, Responder};
use std::{env, io};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
        env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
        env_logger::init();

        HttpServer::new(|| {
                App::new()
                    .wrap(middleware::Logger::default())
                    .service(hello)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        
}

#[get("/")]
async fn hello() -> impl Responder { 
    HttpResponse::Ok().body("hello !")
} 



