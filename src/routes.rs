use actix_web::web;

use crate::handlers;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/contas", web::post().to(handlers::criar_conta))
            .route("/contas/{id}", web::get().to(handlers::obter_conta))
            .route("/pix", web::post().to(handlers::transferencia_pix))
            .route("/saldo/{id}", web::get().to(handlers::obter_saldo)),
    );
}
