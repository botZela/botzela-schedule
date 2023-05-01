use crate::structs::schedule::{gen_module_num, Schedule, Seance};
use axum::{extract::State, http::StatusCode, Json};
use serde::Deserialize;

use mongodb::{bson::doc, Database};

#[derive(Deserialize)]
pub struct PostParams {
    pub year: String,
    pub filiere: String,
    pub groupe: String,
    pub week: String,
}

pub async fn post(
    State(database): State<Database>,
    Json(payload): Json<PostParams>,
) -> (StatusCode, Json<Vec<Vec<Option<Seance>>>>) {
    let fetched = Schedule::fetch(&database, &payload).await;

    match fetched {
        Some(out) => {
            let output = gen_module_num(out.days);
            (StatusCode::OK, Json(output))
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}
