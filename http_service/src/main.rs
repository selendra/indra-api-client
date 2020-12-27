use actix_web::{web, App, HttpServer, Result, HttpResponse};
use cli::{ 
    models::Wallet,
    wallet::get_wallet,
} ;

/// extract `Info` using serde
async fn http_get_wallet(wl: web::Json<Wallet>) -> Result<HttpResponse> {
    let wallet = Wallet {
        label: wl.label.clone(),
        name: wl.name.clone(),
        location: wl.location.clone(),
        phrase: wl.phrase.clone(),
        password: wl.password.clone(),
    };
    let res = get_wallet(wallet);
    Ok(
        HttpResponse::Ok().json(res)
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
        .route("/", web::post().to(http_get_wallet)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}