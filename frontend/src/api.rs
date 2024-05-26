use common::schedule::{self, Days};
use leptos::logging::{error, log};

pub async fn fetch_schedule(body: schedule::PostParams) -> Option<Days> {
    let json_body = serde_json::to_string(&body).expect("msg");

    match gloo_net::http::Request::post("/api/schedule")
        .header("content-type", "application/json")
        .body(json_body)
        .ok()?
        .send()
        .await
    {
        Err(e) => {
            log!("Something is wrong");
            error!("{e}");
            None
        }
        Ok(x) => match x.json::<Days>().await {
            Ok(x) => Some(x),
            Err(_) => {
                log!("Nothing Here");
                None
            }
        },
    }
}
