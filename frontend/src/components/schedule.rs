use super::header::*;
use super::week::*;
use common::schedule::{self, Seance};
use leptos::ev::SubmitEvent;
use leptos::html::Select;
use leptos::log;
use leptos::*;

async fn fetch_schedule(body: schedule::PostParams) -> Option<Vec<Vec<Option<Seance>>>> {
    let json_body = serde_json::to_string(&body).expect("msg");

    let res = gloo_net::http::Request::post("/api/schedule")
        .header("content-type", "application/json")
        .body(json_body)
        .send()
        .await
        .map_err(|e| log!("{e}"))
        .ok()?
        .json::<Vec<Vec<Option<Seance>>>>()
        .await
        .ok()?;

    Some(res)
}

#[component]
pub fn Schedule(cx: Scope) -> impl IntoView {
    let body = schedule::PostParams {
        year: "1A".to_owned(),
        filiere: "2IA".to_owned(),
        groupe: "G1".to_owned(),
        week: "S22".to_owned(),
    };

    let (schedule_body, set_schedule_body) = create_signal(cx, body);

    let once = create_resource(cx, schedule_body, |value| async move {
        fetch_schedule(value).await
    });

    let select_element_fl: NodeRef<Select> = create_node_ref(cx);
    let select_element_grp: NodeRef<Select> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value_fl = select_element_fl().expect("<input> to exist").value();
        let value_grp = select_element_grp().expect("<input> to exist").value();
        set_schedule_body.update(|count| {
            count.filiere = value_fl;
            count.groupe = value_grp;
        });
    };

    let fl_vec = vec![
        "2IA", "2SCL", "BI&A", "GD", "GL", "IDF", "IDSIT", "SSI", "SSE",
    ];
    let grp_vec = vec!["G1", "G2", "G3", "G4", "G5", "G6", "G7", "G8"];

    view! { cx,
        <div class="container">
            <Header header_args=schedule_body/>
            <form on:submit=on_submit>
                <select name="Filiere" id="filiere" node_ref=select_element_fl>
                    {fl_vec
                        .iter()
                        .map(|&fl| {
                            view! { cx,
                                <option value=fl selected=schedule_body.get().filiere == fl.to_string()>
                                    {fl}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
                <select name="Groupe" id="group" node_ref=select_element_grp>
                    {grp_vec
                        .iter()
                        .map(|&grp| {
                            view! { cx,
                                <option value=grp selected=schedule_body.get().groupe == grp.to_string()>
                                    {grp}
                                </option>
                            }
                        })
                        .collect::<Vec<_>>()}
                </select>
                <input type="submit" value="Submit"/>
            </form>
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
