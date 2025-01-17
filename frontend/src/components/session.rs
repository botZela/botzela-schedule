use common::schedule::Seance;
use leptos::prelude::*;

#[component]
pub fn Session(session: Option<Seance>) -> impl IntoView {
    if let Some(ref session) = session {
        view! {
            <td class="class">{session.class.clone()}</td>
            <td class="matiere" data-module=session.module_num.unwrap_or(0)>
                {session.name.clone()}
            </td>
        }
        .into_any()
    } else {
        view! {
            <td class="class empty"></td>
            <td class="matiere empty"></td>
        }
        .into_any()
    }
}
