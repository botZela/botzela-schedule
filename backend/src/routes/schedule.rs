use crate::structs::schedule::{self, gen_module_num};
use axum::{extract::State, http::StatusCode, Json};
use common::schedule::{Days, PostParams};

use mongodb::Database;

pub async fn post(
    State(database): State<Database>,
    Json(payload): Json<PostParams>,
) -> (StatusCode, Json<Days>) {
    let fetched = schedule::fetch(&database, &payload).await;

    match fetched {
        Some(out) => {
            let output = gen_module_num(out.days);
            (StatusCode::OK, Json(output))
        }
        _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::new())),
    }
}
