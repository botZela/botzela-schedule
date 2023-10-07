use super::session::*;
use crate::SHOW_BRANCH;
use common::schedule::Seance;
use leptos::*;

#[component]
pub fn Day(day: &'static str, seances: Vec<Option<Seance>>) -> impl IntoView {
    let out: Vec<_> = seances
        .iter()
        .map(|s| view! { <Session session=s/> }.into_view())
        .collect();

    let mid = out.len() / 2;

    let first_half = Vec::from(&out[..mid]);
    let second_half = Vec::from(&out[mid..]);

    view! {
        <tr class="day">
            <td class="day-name">
                <b>{day}</b>
            </td>
            <Show
                when=move || SHOW_BRANCH
                fallback=|| {
                    view! {}
                }
            >

                <td class="filiere">"SSI"</td>
            </Show>
            {first_half}
            <td class="delimiter"></td>
            {second_half}
        </tr>
    }
}
