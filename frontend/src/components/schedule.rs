use super::header::*;
use super::week::*;
use crate::api::fetch_schedule;
use common::schedule;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Schedule(cx: Scope) -> impl IntoView {
    let params = use_query::<schedule::PostParams>(cx);

    let body = move || {
        params.with(|params| {
            params.clone().unwrap_or(schedule::PostParams {
                year: "1A".to_owned(),
                filiere: "2IA".to_owned(),
                groupe: "G1".to_owned(),
                week: "S22".to_owned(),
            })
        })
    };

    let once = create_resource(cx, body, |value| async move { fetch_schedule(value).await });

    view! { cx,
        <div class="container">
            <Header/>
            <Suspense fallback=|| {
                view! { cx, "Loading..." }
            }>
                {move || match once.read(cx) {
                    None => {
                        view! { cx, <p>"Loading..."</p> }
                            .into_view(cx)
                    }
                    Some(data) => {
                        match data {
                            Some(days) => {
                                view! { cx, <Week days/> }
                                    .into_view(cx)
                            }
                            None => {
                                view! { cx, "Not Found" }
                                    .into_view(cx)
                            }
                        }
                    }
                }}
            </Suspense>
        </div>
    }
}
