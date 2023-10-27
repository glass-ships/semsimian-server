use actix_web::{App, HttpServer};

use semsimian_server::{compare_termsets, say_hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(say_hello)
            .service(compare_termsets)
            // .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}