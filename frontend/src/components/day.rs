use super::session::*;
use crate::SHOW_BRANCH;
use common::schedule::Seance;
use leptos::*;

#[component]
pub fn Day(cx: Scope, day: &'static str, seances: Vec<Option<Seance>>) -> impl IntoView {
    let out: Vec<_> = seances
        .iter()
        .map(|s| view! { cx, <Session session=s/> }.into_view(cx))
        .collect();

    let mid = out.len() / 2;

    let first_half = Vec::from(&out[..mid]);
    let second_half = Vec::from(&out[mid..]);

    view! { cx,
        <tr class="day">
            <td class="day-name">
                <b>{day}</b>
            </td>
            <Show
                when=move || SHOW_BRANCH
                fallback=|_| {
                    view! { cx,  }
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
