use super::session::*;
use crate::SHOW_BRANCH;
use common::schedule::Seance;
use leptos::*;

#[component]
pub fn Day(cx: Scope, day: &'static str, seances: Vec<Option<Seance>>) -> impl IntoView {
    let session1 = seances.get(0).unwrap_or(&None);
    let session2 = seances.get(1).unwrap_or(&None);
    let session3 = seances.get(2).unwrap_or(&None);
    let session4 = seances.get(3).unwrap_or(&None);

    view! { cx,
        <tr class="day">
            <td class="day-name"><b>{day}</b></td>

            <Show when=move || SHOW_BRANCH fallback=|_| view!{ cx, } >
                <td class="filiere">"SSI"</td>
            </Show>

            <Session session=session1 />

            <Session session=session2 />

            <td class="delimiter"></td>

            <Session session=session3 />

            <Session session=session4 />
        </tr>
    }
}
