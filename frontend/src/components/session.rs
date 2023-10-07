use common::schedule::Seance;
use leptos::*;

#[component]
pub fn Session<'a>(session: &'a Option<Seance>) -> impl IntoView {
    if let Some(ref session) = session {
        view! {
            <td class="class">{&session.class}</td>
            <td class="matiere" data-module=session.module_num.unwrap_or(0)>
                {&session.name}
            </td>
        }
    } else {
        view! {
            <td class="class empty"></td>
            <td class="matiere empty"></td>
        }
    }
}
