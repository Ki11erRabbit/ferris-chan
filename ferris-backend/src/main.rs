mod endpoints;
mod transfer;
mod constants;

use actix_web::{App, HttpRequest, HttpServer};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {


    HttpServer::new(|| {
        App::new()
            .service(endpoints::get_home)
            .service(endpoints::admin::login_admin)
            .service(endpoints::admin::logout_admin)
            .service(endpoints::admin::admin_remove_post)
            .service(endpoints::post::get_posts)
            .service(endpoints::post::get_post_replies)
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await

}
