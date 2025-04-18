use actix_web::{web, App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
            .service(web::resource("/index.html").to(|| async { "Hello World!"}))
            .service(web::resource("/").to(index))
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await

}
