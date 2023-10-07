mod api;
mod components;
use components::notfound::NotFound;
use components::schedule::Schedule;
use leptos::*;
use leptos_router::*;

pub const SHOW_BRANCH: bool = false;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=|| {
                            view! { <Schedule/> }
                        }
                    />

                    <Route
                        path="/*any"
                        view=|| {
                            view! { <NotFound/> }
                        }
                    />

                </Routes>
            </main>
        </Router>
    }
}
