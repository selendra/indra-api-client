use crate::api::*;
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configurating routes...");
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(
                        web::resource("/signup").route(web::post().to(account_controller::signup)),
                    )
                    .service(
                        web::resource("/login").route(web::post().to(account_controller::login)),
                    )
                    .service(
                        web::resource("/logout").route(web::post().to(account_controller::logout)),
                    ),
            )
            .service(
                web::scope("/indracore")
                    .service(
                        web::resource("/total").route(web::get().to(balance_controller::get_total)),
                    )
                    .service(
                        web::resource("/free")
                            .route(web::post().to(balance_controller::get_balance)),
                    )
                    .service(
                        web::resource("/transaction")
                            .route(web::post().to(balance_controller::make_transaction)),
                    ),
            ),
    );
}
