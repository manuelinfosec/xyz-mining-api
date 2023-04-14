#[macro_use]
extern crate actix_web;

mod wallet;
mod miner;
mod util;
mod controllers {
    pub mod miner;
    pub mod wallet;
}

use {
    actix_web::{middleware, App, HttpServer},
    std::{env, io},
};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(controllers::wallet::list_wallets)
            .service(controllers::wallet::get_wallet)
            .service(controllers::wallet::create_wallet)
            .service(controllers::miner::list_miners)
            .service(controllers::miner::get_miner)
            .service(controllers::miner::create_miner)
    })
        .bind("0.0.0.0:9090").expect("Could not start server")
        .run()
        .await
}
