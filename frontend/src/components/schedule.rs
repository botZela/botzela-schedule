use super::header::*;
use super::week::*;
use crate::api::fetch_schedule;
use common::schedule;
use leptos::prelude::*;
use leptos_router::hooks::use_query;

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

    let async_data = LocalResource::new(move || fetch_schedule(body()));

    view! {
        <div class="container">
            <Header/>
            <Transition
                fallback=move || { view! { <Week days=None/> } }
            >
                {move || Suspend::new(async move {
                    view! { <Week days={async_data.await}/> }.into_any()
                })}
            </Transition>
        </div>
    }
}
