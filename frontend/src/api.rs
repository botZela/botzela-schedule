use common::schedule::{self, Seance};
use leptos::log;

pub async fn fetch_schedule(body: schedule::PostParams) -> Option<Vec<Vec<Option<Seance>>>> {
    let json_body = serde_json::to_string(&body).expect("msg");

    let res = gloo_net::http::Request::post("/api/schedule")
        .header("content-type", "application/json")
        .body(json_body)
        .expect("a JSON Body")
        .send()
        .await
        .map_err(|e| log!("{e}"))
        .ok()?
        .json::<Vec<Vec<Option<Seance>>>>()
        .await
        .ok()?;

    Some(res)
}
