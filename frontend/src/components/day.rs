use super::session::*;
use crate::SHOW_BRANCH;
use common::schedule::Seance;
use leptos::prelude::*;

#[component]
pub fn Day(day: &'static str, seances: Vec<Option<Seance>>) -> impl IntoView {
    let mid = seances.len() / 2;
    let mut first_half: Vec<_> = seances
        .into_iter()
        .map(|s| view! { <Session session=s/> }.into_any())
        .collect();

    let second_half = first_half.split_off(mid);

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
