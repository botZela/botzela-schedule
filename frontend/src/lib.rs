mod api;
mod components;
use components::schedule::*;
use leptos::*;
use leptos_router::*;

pub const SHOW_BRANCH: bool = false;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <main>
                <Routes>
                    <Route
                        path="/"
                        view=|cx| {
                            view! { cx, <Schedule/> }
                        }
                    />
                    <Route
                        path="/*any"
                        view=|cx| {
                            view! { cx, <h1>"Not Found"</h1> }
                        }
                    />
                </Routes>
            </main>
        </Router>
    }
}
