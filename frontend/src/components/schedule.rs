use super::header::*;
use super::week::*;
use crate::api::fetch_schedule;
use common::schedule;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Schedule() -> impl IntoView {
    let params = use_query::<schedule::PostParams>();

    let body = move || {
        params.with(|params| {
            params.clone().unwrap_or(schedule::PostParams {
                year: Some("1A".to_owned()),
                filiere: Some("2IA".to_owned()),
                groupe: Some("G1".to_owned()),
                week: Some("S22".to_owned()),
            })
        })
    };

    let once = create_resource(body, |value| async move { fetch_schedule(value).await });

    view! {
        <div class="container">
            <Header/>
            <Suspense fallback=|| {
                view! { "Loading..." }
            }>
                {move || match once.get() {
                    None => view! { <p>"Not Found"</p> }.into_view(),
                    Some(data) => {
                        match data {
                            Some(days) => view! { <Week days/> }.into_view(),
                            None => view! { "Not Found" }.into_view(),
                        }
                    }
                }}

            </Suspense>
        </div>
    }
}
