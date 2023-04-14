pub use {
    actix_web::HttpResponse,
    actix_web::web::Json,
    actix_web::get,
    actix_web::post,

    crate::miner::*,
    crate::util::*
};

// Defining REST endpoints for the API

// List all Miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    /*
        TODO: Get all MinerDAO objects from DB and convert to Miner objects
    */

    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

// Get a Miner by ID
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    /*
        TODO: Get the MinerDAO object from DB WHERE id = requested id and convert to Miner object
     */
    let miner: Option<Miner> = None;
    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found".to_string())
        ).get_response(),
    }
}

#[post("/wallets/{id}/miners")]
pub async fn create_miner(miner_request: Json<NewMiner>) -> HttpResponse {
    let miner: Vec<Miner> = vec![]; // empty for now
    ResponseType::Created(miner).get_response()
}