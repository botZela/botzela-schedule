mod api;
mod components;
use components::notfound::NotFound;
use components::schedule::Schedule;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

pub const SHOW_BRANCH: bool = false;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <Router>
                <Routes fallback=|| view! { <NotFound /> }>
                   <Route path=path!("/") view=Schedule />
                </Routes>
            </Router>
        </main>
    }
}
