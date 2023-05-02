mod components;
use components::schedule::*;
use leptos::*;

pub const SHOW_BRANCH: bool = false;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Schedule />
    }
}
