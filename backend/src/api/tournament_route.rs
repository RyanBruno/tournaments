use http::{Request, Response, StatusCode};
use std::error::Error;

use crate::not_found_route;
use crate::{TournamentStore, TournamentCommand, TournamentModel};
use models::{Bracket, ScoreUpdate};

pub fn create_tournament_route(
    _req: &Request<()>,
    store: TournamentStore,
    id: String,
    name: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let bracket = Bracket { id, name, matches: Vec::new() };
    let mut store = store;
    store.command(&TournamentCommand::CreateBracket(bracket))?;

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"created\"}".to_vec())?)
}

pub fn update_score_route(
    _req: &Request<()>,
    store: TournamentStore,
    bracket_id: String,
    match_id: String,
    score_a: u32,
    score_b: u32,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let update = ScoreUpdate { match_id, score_a, score_b };
    let mut store = store;
    store.command(&TournamentCommand::UpdateScore((bracket_id, update)))?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "*")
        .header("Access-Control-Allow-Headers", "*")
        .body(b"{\"status\":\"updated\"}".to_vec())?)
}

pub fn live_results_route(
    _req: &Request<()>,
    store: TournamentStore,
    bracket_id: String,
) -> Result<Response<Vec<u8>>, Box<dyn Error>> {
    let bracket = store.borrow_inner().query_owned(bracket_id)?;
    match bracket {
        Some(TournamentModel::Bracket(bracket)) => {
            let json = serde_json::to_vec(&bracket)?;
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .header("Access-Control-Allow-Origin", "*")
                .header("Access-Control-Allow-Methods", "*")
                .header("Access-Control-Allow-Headers", "*")
                .body(json)?)
        }
        _ => not_found_route(),
    }
}
