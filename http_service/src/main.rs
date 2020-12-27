use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use cli::{wallet};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Service Working!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn gen_wallet() -> impl Responder {
    HttpResponse::Ok().body(wallet::get_wallet(Wallet))
    // wallets::get_wallet(wallet)
}

async fn check_balance() -> impl Responder {
    HttpResponse::Ok().body("Start work on POST!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/getwallet", web::get().to(gen_wallet))
            .route("/getbalance", web::get().to(check_balance))
    })
    .bind("127.0.0.1:9002")?
    .run()
    .await
}