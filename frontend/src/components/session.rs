use common::schedule::Seance;
use leptos::*;

#[component]
pub fn Session<'a>(cx: Scope, session: &'a Option<Seance>) -> impl IntoView {
    if let Some(ref session) = session {
        view! { cx,
            <td class="class" >{&session.class}</td>
            <td class="matiere" data-module={session.module_num.unwrap_or(0)}>{&session.name}</td>
        }
    } else {
        view! { cx,
            <td class="class empty" ></td>
            <td class="matiere empty"></td>
        }
    }
}
