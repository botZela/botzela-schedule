use super::header::*;
use super::week::*;
use crate::api::fetch_schedule;
use common::schedule;
use leptos::logging::log;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Schedule() -> impl IntoView {
    let params = use_query::<schedule::PostParams>();

    let body = move || {
        params.with(|params| {
            let p = params.clone().unwrap_or_default();
            schedule::PostParams {
                year: Some(p.year.unwrap_or("1A".to_owned())),
                filiere: Some(p.filiere.unwrap_or("2IA".to_owned())),
                groupe: Some(p.groupe.unwrap_or("G1".to_owned())),
                week: Some(p.week.unwrap_or("S16".to_owned())),
            }
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
