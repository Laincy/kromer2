mod wallet;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    // cfg.service(index_get);
    // cfg.service(version_get);
    cfg.configure(wallet::config);
    // cfg.configure(transaction::config);
    // cfg.configure(name::config);
}
