use {
    actix_web::HttpResponse,
    actix_web::web::Json,
    actix_web::get,
    actix_web::post,

    crate::wallet::*,
    crate::util::*
};

#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    /*
        TODO: Get all WalletDAO object from DB and convert to Wallet objects
     */

    let wallets : Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    /*
        TODO: Get the WalletDAO object from WHERE id = requested id and convert to Wallet object
     */

    let wallet: Option<Wallet> = None; // None for now
    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),

        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet/Club not found".to_string())
        ).get_response(),
    }
}

#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<NewWallet>) -> HttpResponse {
    /*
        TODO: Create a new WalletDAO object from request and write to DB
     */

    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}


